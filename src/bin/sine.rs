extern crate simpsynth;
extern crate byteorder;

use simpsynth::module::{Module, Sine, Square, Mult, Add, Envelope};
use simpsynth::module::envelope::AudioPoint;

use std::io;
use self::byteorder::{LittleEndian, WriteBytesExt};

const SAMPLE_FREQ: u32 = 44100;

fn main() {
    let mut stdout = io::stdout();

//    let sine = Square::new(440.0);
//    let multsine = Mult::new(Mult::new(Add::new(sine, 1.0), 0.5), 440.0);
//    let multsine = Volume::new(sine, 0.5);
//    let modulated = Sine::new(multsine);
//    let mut fnl = Mult::new(modulated, 0.5);
    let env = vec![
        AudioPoint{
            time: 0.0,
            amplitude: 0.0,
        },
        AudioPoint{
            time: 1.0,
            amplitude: 1.0,
        },
        AudioPoint{
            time: 1.5,
            amplitude: 0.5,
        },
        AudioPoint{
            time: 2.0,
            amplitude: 1.0,
        },
        AudioPoint{
            time: 5.0,
            amplitude: 0.0,
        },
    ];
    let mut fnl = Envelope::new(env);

    for _ in 0..10*SAMPLE_FREQ
    {
        let sample = fnl.generate(1.0 / (SAMPLE_FREQ as f32)).unwrap();
        stdout.write_f32::<LittleEndian>(sample).unwrap();
    }
}
