pub mod mapper;

pub mod cellular;
pub mod perlin;
#[allow(dead_code)]
pub mod value;

pub trait Noise {
    type Input;
    type Output;

    fn get(&self, input: Self::Input) -> Self::Output;
}
