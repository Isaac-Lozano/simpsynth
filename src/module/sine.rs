use module::Module;

use std::f32::consts::PI;

pub struct Sine
{
    freq: Box<Module>,
    time: f32,
}

impl Sine
{
    pub fn new<M: Module + 'static>(freq: M) -> Sine
    {
        Sine
        {
            freq: Box::new(freq),
            time: 0.0,
        }
    }
}

impl Module for Sine
{
    fn generate(&mut self, step: f32) -> Option<f32>
    {
        let res = self.time.sin();
        self.time += step * self.freq.generate(step).unwrap() * 2.0 * PI;
        Some(res)
    }
}
