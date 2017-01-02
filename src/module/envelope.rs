use module::Module;

#[derive(Clone,Copy,Debug)]
pub struct AudioPoint
{
    pub time: f32,
    pub amplitude: f32,
}

#[derive(Clone,Copy,Debug)]
enum State
{
    Before,
    Interpolating
    {
        first: AudioPoint,
        second: AudioPoint,
        next_idx: usize,
    },
    After,
}

pub struct Envelope
{
    time: f32,
    env: Vec<AudioPoint>,
    state: State,
}

impl Envelope
{
    /* TODO: Error on env.len() < 2 */
    /* TODO: Error on unordered env */
    pub fn new(env: Vec<AudioPoint>) -> Envelope
    {
        Envelope
        {
            time: 0.0,
            env: env,
            state: State::Before,
        }
    }

    fn interpolate(&mut self) -> Option<f32>
    {
        match self.state
        {
            State::Before => unreachable!(),
            State::Interpolating{
                ref mut first,
                ref mut second, 
                ref mut next_idx
            } =>
            {
                let mut finished = false;

                while self.time > second.time
                {
                    if let Some(next) = self.env.get(*next_idx)
                    {
                        *first = *second;
                        *second = *next;
                        *next_idx += 1;
                    }
                    else
                    {
                        /* Fall through because
                         * we cannot change self.state
                         * here.
                         */
                        finished = true;
                        break;
                    }
                }


                if !finished
                {
                    /* second_time - first_time will never be 0.0 */
                    let amp = (((second.amplitude - first.amplitude) * (self.time - first.time)) / (second.time - first.time)) + first.amplitude;
                    return Some(amp);
                }
            },
            State::After => unreachable!(),
        }
        /* Change self.state here outside of borrow */
        self.state = State::After;
        Some(0.0)
    }
}

impl Module for Envelope
{
    fn generate(&mut self, step: f32) -> Option<f32>
    {
        let ret;
        match self.state
        {
            State::Before =>
            {
                let first = self.env[0];
                if first.time <= self.time
                {
                    self.state = State::Interpolating
                    {
                        first: first,
                        second: self.env[1],
                        next_idx: 2,
                    };
                    ret = self.interpolate();
                }
                else
                {
                    ret = Some(0.0);
                }
            },
            State::Interpolating{..} =>
            {
                ret = self.interpolate();
            },
            State::After =>
            {
                ret = Some(0.0);
            },
        }
        self.time += step;
        ret
    }
}
