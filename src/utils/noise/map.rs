use super::Noise;

pub struct Map<N, F> {
    noise: N,
    f: F,
}

impl<N, F> Map<N, F> {
    pub(super) fn new(noise: N, f: F) -> Map<N, F> {
        Map { noise, f }
    }
}

impl<N, F, I, O, MO> Noise for Map<N, F>
where
    N: Noise<Input = I, Output = O>,
    F: Fn(O) -> MO,
{
    type Input = I;
    type Output = MO;

    fn get(&self, input: Self::Input) -> Self::Output {
        (self.f)(self.noise.get(input))
    }
}
