extern crate simpsynth;
extern crate byteorder;

use simpsynth::module::{Module, Sine, Square, Mult, Add, Envelope, Sample};
use simpsynth::module::envelope::AudioPoint;
use simpsynth::module::sample::FileType;

use std::io;
use self::byteorder::{LittleEndian, WriteBytesExt};

const SAMPLE_FREQ: u32 = 44100;

fn main() {
    let mut stdout = io::stdout();

//    let modsrc = Sine::new(440.0);
    let modsrc = Sample::new("test.raw", FileType::U16LE);
    let modwave = Mult::new(Add::new(Mult::new(modsrc, 15.0), 1.0), 440.0);
    let modulated = Sine::new(modwave);
//    let mut fnl = Mult::new(modulated, 0.5);
//    let env = vec![
//        AudioPoint{
//            time: 0.0,
//            amplitude: 0.0,
//        },
//        AudioPoint{
//            time: 0.01,
//            amplitude: 0.5,
//        },
//        AudioPoint{
//            time: 1.0,
//            amplitude: 0.1,
//        },
//        AudioPoint{
//            time: 3.0,
//            amplitude: 0.0,
//        },
//    ];
//    let envelope = Envelope::new(env);
//    let sine = Sine::new(440.0);
    let mut fnl = Mult::new(modulated, 0.05);

    for _ in 0..11*SAMPLE_FREQ
    {
        let sample = fnl.generate(1.0 / (SAMPLE_FREQ as f32)).unwrap();
        stdout.write_f32::<LittleEndian>(sample).unwrap();
    }
}
