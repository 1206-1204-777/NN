use burn::prelude::Backend;
use burn::tensor::Tensor;
pub fn execute<B: Backend>(device: &B::Device){
    let tensor = Tensor::<B, 2>::from_floats([[1.5, 3.5], [5.5, 4.5]], &device);
    let output = tensor.clone().matmul(tensor.transpose());

    println!("{}", output);
}