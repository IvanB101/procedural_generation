pub mod perlin;
pub mod value;

pub trait Noise<I, O> {
    fn get(&self, input: &I) -> O;
}
