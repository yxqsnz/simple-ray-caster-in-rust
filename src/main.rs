mod ppm;
use std::{fs::File, io};

use ppm::{Color, Ppm};

fn main() -> io::Result<()> {
    let mut ppm = Ppm::new(512, 512);
    let mut out = File::create("target/out.ppm")?;

    for i in 0..512 {
        for j in 0..512 {
            (*ppm).push(Color {
                r: (255 * i / 512),
                g: (255 * j / 512),
                b: 0,
            })
        }
    }

    ppm.write_to(&mut out)?;

    Ok(())
}
