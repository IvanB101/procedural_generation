pub mod perlin;
#[allow(dead_code)]
pub mod value;

pub trait Noise<I, O> {
    fn get(&self, input: I) -> O;
}
