mod add;
mod cat;
mod coeff;
mod crossentropy;
mod dropout;
mod layer_norm;
mod mask;
mod matmul;
mod relu;
mod softmax;
mod transpose;

pub use add::*;
pub use cat::*;
pub use coeff::*;
pub use crossentropy::*;
pub use dropout::*;
pub use layer_norm::*;
pub use mask::*;
pub use matmul::*;
pub use relu::*;
pub use softmax::*;
pub use transpose::*;

use super::tensor::*;

pub trait Function: std::fmt::Debug {
    fn run(&mut self, inps: &[&Tensor<f32>], training: bool) -> Tensor<f32>;
    fn grad(&self, inps: &[&Tensor<f32>], out_grad: &Tensor<f32>) -> Vec<Tensor<f32>>;
}

pub trait Loss: std::fmt::Debug {
    fn run(&self, inp: &Tensor<f32>) -> (Tensor<f32>, Tensor<f32>);
}
