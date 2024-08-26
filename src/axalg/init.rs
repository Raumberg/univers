use rust_cuda::{CudaContext, CudaDevice, CudaModule};

fn main() {
    // Initialize CUDA
    let ctx = CudaContext::new().unwrap();
    let device = CudaDevice::get_device(0).unwrap();
    ctx.set_device(device).unwrap();

    // Load a CUDA module (kernel code)
    let module = CudaModule::load_from_file("kernel.cubin").unwrap();

    // Allocate memory on the GPU
    let mut buffer: Vec<f32> = vec![1.0; 1024];
    let gpu_buffer = ctx.alloc_device_memory(buffer.len() * std::mem::size_of::<f32>()).unwrap();

    // Copy data from host to device
    ctx.memcpy_h2d(gpu_buffer, &buffer).unwrap();

    // Launch the kernel
    let block_dim = 256;
    let grid_dim = (buffer.len() + block_dim - 1) / block_dim;
    let kernel = module.get_function("gpu_kerr").unwrap();
    kernel.launch((grid_dim, 1, 1), (block_dim, 1, 1), gpu_buffer).unwrap();

    // Synchronize the device
    ctx.synchronize().unwrap();

    // Copy data from device to host
    ctx.memcpy_d2h(&mut buffer, gpu_buffer).unwrap();

    // Print the result
    println!("{:?}", buffer);
}