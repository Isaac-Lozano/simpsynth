use module::Module;

pub struct Mult
{
    op1: Box<Module>,
    op2: Box<Module>,
}

impl Mult
{
    pub fn new<M: Module + 'static, N: Module + 'static>(op1: M, op2: N) -> Mult
    {
        Mult
        {
            op1: Box::new(op1),
            op2: Box::new(op2),
        }
    }
}

impl Module for Mult
{
    fn generate(&mut self, step: f32) -> Option<f32>
    {
        Some(self.op1.generate(step).unwrap() * self.op2.generate(step).unwrap())
    }
}
