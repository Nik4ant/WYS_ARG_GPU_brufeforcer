use std::{
	borrow::Cow
};
use wgpu::{
	self
};


// TODO: Check out .wgsl specifications (VERY helpful): https://www.w3.org/TR/WGSL/#numeric-literals
async fn execute_compute_shader(device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), wgpu::Error> {
	// Loading compute shader
	let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: None,
		source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("gota_go_fast.wgsl")))
	});

	todo!("Figure out how to pass two dimensional array of numbers");

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
	// let shader_params: Vec<u32> = vec![1, 2, 3, 4];
	let (device, queue) = setup_gpu().await;
    let gpu_output = execute_compute_shader(&device, &queue).await.unwrap();

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
