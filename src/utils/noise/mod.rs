use map::Map;

pub mod map;

pub mod cellular;
pub mod perlin;
#[allow(dead_code)]
pub mod value;

pub trait Noise {
    type Input;
    type Output;

    fn get(&self, input: Self::Input) -> Self::Output;

    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map::new(self, f)
    }
}
