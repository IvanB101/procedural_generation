use super::Noise;

pub mod functions;

pub struct Mapper<N: Noise> {
    source: N,
    functions: Vec<fn(N::Output) -> N::Output>,
}

impl<N> Mapper<N>
where
    N: Noise,
{
    pub fn new(source: N) -> Self {
        Mapper {
            source,
            functions: Vec::new(),
        }
    }

    pub fn add_custom(&mut self, function: fn(N::Output) -> N::Output) -> &mut Self {
        self.functions.push(function);

        self
    }
}

impl<N, I, O> Noise for Mapper<N>
where
    N: Noise<Input = I, Output = O>,
{
    type Input = I;
    type Output = O;

    fn get(&self, input: N::Input) -> N::Output {
        let mut value = self.source.get(input);

        for function in &self.functions {
            value = function(value);
        }

        value
    }
}
