use std::{
	borrow::Cow
};
use wgpu::{
	self, util::DeviceExt
};

// WARNING: Key length is hardcoded AND IT'S NOT SYNCED between .wgsl file at the moment. 
// This is very bad and will be changed later
const KEY_LENGTH: usize = 7;

// TODO: Check out .wgsl specifications (VERY helpful): https://www.w3.org/TR/WGSL/#numeric-literals
async fn execute_compute_shader(device: &wgpu::Device, queue: &wgpu::Queue, keys: &[[u32; KEY_LENGTH]]) -> Result<(), wgpu::Error> {
	// Loading compute shader
	let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Bruteforcer's compute shader"),
		source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("gota_go_fast.wgsl")))
	});

	// Buffer with array of keys.
	// Usage allowing the buffer to be:
    //  - A storage buffer (can be bound within a bind group and thus available to a shader).
    //  - The destination of a copy.
    //  - The source of a copy.
	let storage_keys_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Keys buffer"),
		contents: bytemuck::cast_slice(keys),
		// FIXME: How do I change this to leave readonly access?!
		usage: wgpu::BufferUsages::STORAGE,
	});

	// WARNING: THIS IS REALLY BAD. REMOVE LATER AFTER TESTING
	const EXPECTED_OUTPUT_LENGTH: usize = 50;
	const OUTPUT_BUFFER_SIZE: u64 = (EXPECTED_OUTPUT_LENGTH * std::mem::size_of::<u32>()) as u64;

	let staging_buffer_thing = device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("Staging wierd thing"),
		size: OUTPUT_BUFFER_SIZE,
		usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
		mapped_at_creation: true,
	});

	// Using empty array as init value <-- WARNING: NOTE: THAT'S THE KEY!
	let bruteforcer_output_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Output buffer"),
		contents: bytemuck::cast_slice(&[0_u32; 50]),
		usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
	});
	
	// A pipeline specifies the operation of a shader.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader_module,
        entry_point: "main",
    });

	// NOTE: Code below prepares buffer data for shader buffer on the GPU with the same binding index
	// On shader side it looks like this: `layout(set = 0, binding = 0) buffer`
    let keys_bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let keys_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &keys_bind_group_layout,
        entries: &[
			wgpu::BindGroupEntry {
            	binding: 0,
            	resource: storage_keys_buffer.as_entire_binding(),
        	}
		],
    });
	let bruteforcer_output_bind_group_layout = compute_pipeline.get_bind_group_layout(1);
    let bruteforcer_output_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bruteforcer_output_bind_group_layout,
        entries: &[
			wgpu::BindGroupEntry {
            	binding: 0,
            	resource: bruteforcer_output_buffer.as_entire_binding(),
        	}
		],
    });

	let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
	{
		let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &keys_bind_group, &[]);
        compute_pass.set_bind_group(1, &bruteforcer_output_bind_group, &[]);
        compute_pass.insert_debug_marker("l2a bruteforcer");
		// TODO: scale this later
        compute_pass.dispatch_workgroups(keys.len() as u32, 1, 1);
	}
	// Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    command_encoder.copy_buffer_to_buffer(&bruteforcer_output_buffer, 0, &staging_buffer_thing, 0, OUTPUT_BUFFER_SIZE);
	
	queue.submit(Some(command_encoder.finish()));
	let staging_buffer_thing_slice = staging_buffer_thing.slice(..);
	// This will map execution result to keys_buffer_slice
	let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    staging_buffer_thing_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

	device.poll(wgpu::Maintain::Wait);

	// Awaiting for data from the GPU
	if let Some(Ok(())) = receiver.receive().await {
		// 
		let data = staging_buffer_thing_slice.get_mapped_range();
		let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

		println!("SHADER OUTPUT: {:?}", result);
		// Freeing buffer from the memory, BUT FIRST it's required to drop all mapped views
		drop(data);
		storage_keys_buffer.unmap();
		bruteforcer_output_buffer.unmap();
		staging_buffer_thing.unmap();
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
