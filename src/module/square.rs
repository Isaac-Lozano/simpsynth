use module::Module;

pub struct Square
{
    freq: Box<Module>,
    time: f32,
}

impl Square
{
    pub fn new<M: Module + 'static>(freq: M) -> Square
    {
        Square
        {
            freq: Box::new(freq),
            time: 0.0,
        }
    }
}

impl Module for Square
{
    fn generate(&mut self, step: f32) -> Option<f32>
    {
        self.time %= 1.0;
        let res = if self.time > 0.5
        {
            1.0
        }
        else
        {
            -1.0
        };
        self.time += step * self.freq.generate(step).unwrap();
        Some(res)
    }
}
