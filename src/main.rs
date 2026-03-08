use nn::inspection::{gpu::execute};
use burn_cuda::{Cuda, CudaDevice};
fn main() {
    type Backend = Cuda;
    let device = CudaDevice::default();
    execute::<Backend>(&device);
}
