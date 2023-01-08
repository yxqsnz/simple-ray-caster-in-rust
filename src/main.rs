mod map;
mod ppm;
mod target;
use ppm::Ppm;
use std::{fs::File, io};

use crate::{
    map::{MAP_HEIGHT, MAP_WIDTH},
    target::{TARGET_HEIGHT, TARGET_WIDTH},
};

mod render;

macro_rules! timeit {
    ($scope:expr=> $block:block) => {{
        let started = std::time::Instant::now();
        let res = $block;

        println!("{}: {:#?}", $scope, started.elapsed());
        res
    }};
}

fn main() -> io::Result<()> {
    let mut ppm = Ppm::new(TARGET_HEIGHT, TARGET_WIDTH);
    let mut out = File::create("target/out.ppm")?;
    let player = (3.456, 2.345, 1.523);
    println!("{}", map::RAW);

    assert_eq!(map::data().len(), MAP_HEIGHT * MAP_WIDTH);

    timeit! {
        "render" => { render::draw(&mut ppm, player); }
    };

    let written = timeit! { "write" => { ppm.write_to(&mut out)? }};
    println!("Written {written} bytes.");
    Ok(())
}
