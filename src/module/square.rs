use module::Module;

pub struct Square
{
    freq: Box<Module>,
    phase: f32,
}

impl Square
{
    pub fn new<M: Module + 'static>(freq: M) -> Square
    {
        Square
        {
            freq: Box::new(freq),
            phase: 0.0,
        }
    }
}

impl Module for Square
{
    fn generate(&mut self, step: f32) -> Option<f32>
    {
        self.phase %= 1.0;
        let res = if self.phase > 0.5
        {
            1.0
        }
        else
        {
            -1.0
        };
        self.phase += step * self.freq.generate(step).unwrap();
        Some(res)
    }
}
