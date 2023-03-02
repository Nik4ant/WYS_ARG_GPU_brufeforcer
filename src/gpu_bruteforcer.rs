use std::{
	borrow::Cow
};
use wgpu::{
	self, util::DeviceExt
};

use crate::arg_lib;

// WARNING: Those values are hardcoded AND NOT SYNCED between .wgsl file at the moment. 
// This is very bad and will be changed later
const KEY_LENGTH: usize = 7;
const USED_DATA_LENGTH: usize = arg_lib::DATA4_LEN;

async fn execute_compute_shader(device: &wgpu::Device, queue: &wgpu::Queue, keys: &[[u32; KEY_LENGTH]]) -> Result<(), wgpu::Error> {
	// Loading compute shader
	let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Bruteforcer's compute shader"),
		source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("gota_go_fast.wgsl")))
	});

	// FIXME: In reality it would be better to predict max buffer size based on bruteforcer configuration, 
	// but this sounds like work future me
	let output_buffer_size: u64 = (USED_DATA_LENGTH * std::mem::size_of::<u32>()) as u64;
	// NOTE:
	// cpu_side buffer describes empty buffer that will copy data from the shader (GPU) side at the end.
	// (cpu_side buffer is used once to read shader execution result from output storage buffer)
	// gpu_side buffer describes buffer that will be used on the shader (GPU) side.

	let output_buffer_cpu_side = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output buffer on CPU side"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
	let output_buffer_gpu_side = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Output buffer on GPU side"),
		// Placeholder for gpu output buffer is an array of 0 that will be replaced later with the actual data
        contents: bytemuck::cast_slice(vec![0_u32; output_buffer_size as usize].as_slice()),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
    });

	let keys_buffer_gpu_side = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Keys buffer"),
		// Placeholder for gpu output buffer is an array of 0 that will be replaced later with the actual data
        contents: bytemuck::cast_slice(keys),
        usage: wgpu::BufferUsages::STORAGE,
    });

	// A pipeline specifies the operation of a shader.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader_module,
        entry_point: "main",
    });

	// NOTE: I'm lazy to describe what is bind group, bind group layout and what it has to do with 
	// the same thing on the GPU side

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
	let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
			wgpu::BindGroupEntry {
				binding: 0,
				resource: output_buffer_gpu_side.as_entire_binding(),
			},
			wgpu::BindGroupEntry {
				binding: 1,
				resource: keys_buffer_gpu_side.as_entire_binding(),
			},
		],
    });

	let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
	{
		let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.insert_debug_marker("l2a bruteforcer");
		// TODO: scale this later
        compute_pass.dispatch_workgroups(keys.len() as u32, 1, 1);
	}

	// NOTE(Nik4ant): Those 2 comments were copied from official example and I understand only second one...
	// Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    command_encoder.copy_buffer_to_buffer(&output_buffer_gpu_side, 0, &output_buffer_cpu_side, 0, output_buffer_size);
	queue.submit(Some(command_encoder.finish()));

	let output_buffer_slice = output_buffer_cpu_side.slice(..);
	let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    output_buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

	device.poll(wgpu::Maintain::Wait);

	// Awaiting for data from the GPU
	if let Some(Ok(())) = receiver.receive().await {
		let data = output_buffer_slice.get_mapped_range();
		let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
		
		println!("SHADER OUTPUT: {:?}", result);
		// Freeing buffer from the memory, BUT FIRST it's required to drop all mapped views
		drop(data);
		output_buffer_cpu_side.unmap();
	}

	Ok(())
}

async fn setup_gpu() -> (wgpu::Device, wgpu::Queue) {
	// Instance of WebGPU
	let gpu_instance = wgpu::Instance::default();
	// `request_adapter` instantiates the general connection to the GPU
	let gpu_adapter_options = wgpu::RequestAdapterOptions {
		power_preference: wgpu::PowerPreference::HighPerformance,	
		..wgpu::RequestAdapterOptions::default()
	};
	let gpu_adapter: wgpu::Adapter = gpu_instance
										.request_adapter(&gpu_adapter_options)
										.await
										.unwrap();
	// `request_device` creates the feature specific connection to the GPU, defining some parameters,
    // where `features` are available features
    let (device, queue) = gpu_adapter
        .request_device(&wgpu::DeviceDescriptor {
                label: None,
				limits: wgpu::Limits::downlevel_defaults(),
                features: wgpu::Features::default(),
            },
            None,
        )
        .await
        .unwrap();
	
	let adapter_info = gpu_adapter.get_info();
	println!("Selected graphics card: \"{}\"", adapter_info.name);
	// Wierd check for specific PCI id
    if adapter_info.vendor == 0x10005 {
        panic!("Wgpu doesn't support this device. Official example suggests to: \"skip on LavaPipe temporarily\"");
    }

	return (device, queue);
}

async fn start_bruteforcer() {
	let keys = [[24, 4, 25, 15, 25, 15, 25]];

	let (device, queue) = setup_gpu().await;
    let gpu_output = execute_compute_shader(&device, &queue, &keys).await.unwrap();

    println!("Result: [{:?}]", gpu_output);
    #[cfg(target_arch = "wasm32")]
    log::info!("Result: [{:?}]", gpu_output);
}

pub fn start_bruteforcer_sync() {
	println!("Starting GPU bruteforcer...pray to all gods that you know...");
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(start_bruteforcer());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("can initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}
