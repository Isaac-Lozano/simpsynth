use module::Module;

use byteorder::{LittleEndian, ReadBytesExt};

use std::fs::File;
use std::io::Read;
use std::u16;

pub enum FileType
{
    U16LE,
}

pub struct Sample
{
    file: File,
    filetype: FileType,
}

impl Sample
{
    pub fn new(filename: &str, filetype: FileType) -> Sample
    {
        Sample
        {
            file: File::open(filename).unwrap(),
            filetype: filetype,
        }
    }
}

impl Module for Sample
{
    fn generate(&mut self, _step: f32) -> Option<f32>
    {
        /* XXX: This isn't taking into account
         * the sample frequency of the smaple.
         */
        let sample = self.file.read_u16::<LittleEndian>().ok().unwrap_or(u16::MAX / 2);
        Some((sample as f32 / (u16::MAX / 2) as f32) - 1.0)
    }
}
