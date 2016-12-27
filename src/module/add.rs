use module::Module;

pub struct Add
{
    op1: Box<Module>,
    op2: Box<Module>,
}

impl Add
{
    pub fn new<M: Module + 'static, N: Module + 'static>(op1: M, op2: N) -> Add
    {
        Add
        {
            op1: Box::new(op1),
            op2: Box::new(op2),
        }
    }
}

impl Module for Add
{
    fn generate(&mut self, step: f32) -> Option<f32>
    {
        Some(self.op1.generate(step).unwrap() + self.op2.generate(step).unwrap())
    }
}
