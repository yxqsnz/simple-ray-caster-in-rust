use std::{
    io::{self, Write},
    ops::{Deref, DerefMut},
};

#[derive(Clone)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub struct Ppm {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Ppm {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::with_capacity(width * height),
        }
    }

    pub fn write_to(&self, target: &mut impl Write) -> io::Result<usize> {
        let Self {
            width,
            height,
            data,
        } = self;

        let mut written = target.write(format!("P6\n{width} {height}\n255\n").as_bytes())?;

        let target_data: Vec<u8> = data
            .iter()
            .flat_map(|Color { r, g, b }| vec![*r as u8, *g as u8, *b as u8])
            .collect();

        written += target.write(&target_data)?;
        Ok(written)
    }
}

impl Deref for Ppm {
    type Target = Vec<Color>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Ppm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
