use crate::{
    map::{self, MAP_HEIGHT, MAP_WIDTH},
    ppm::Color,
    target::{TARGET_HEIGHT, TARGET_WIDTH},
};

fn retangle(
    buf: &mut Vec<Color>,
    height: usize,
    (x, y): (usize, usize),
    (rect_width, rect_height): (usize, usize),
    color: Color,
) {
    for i in 0..rect_height {
        for j in 0..rect_width {
            let (tx, ty) = (x + i, y + j);

            buf[tx + ty * height] = color;
        }
    }
}

pub fn draw(buf: &mut Vec<Color>, (px, py, pat): (f64, f64, f64)) {
    let (rect_h, rect_w) = (TARGET_HEIGHT / MAP_HEIGHT, TARGET_WIDTH / MAP_WIDTH);
    let map = map::data();

    for i in 1..TARGET_HEIGHT {
        for j in 1..TARGET_WIDTH {
            buf[j + i * TARGET_WIDTH] = Color {
                r: (255 * i / TARGET_HEIGHT) as _,
                g: (255 * j / TARGET_WIDTH) as _,
                b: (100 * i * j & i % TARGET_WIDTH) as _,
            }
        }
    }

    for i in 0..MAP_HEIGHT {
        for j in 0..map::MAP_WIDTH {
            if map[j + i * MAP_HEIGHT] == ' ' {
                continue;
            }

            let target_rect = (j * rect_w, i * rect_h);

            retangle(
                buf,
                TARGET_HEIGHT,
                target_rect,
                (rect_w, rect_h),
                Color::new(i & j, i & j, i ^ j),
            )
        }
    }

    retangle(
        buf,
        TARGET_HEIGHT,
        ((px * rect_w as f64) as usize, (py * rect_h as f64) as usize),
        (5, 5),
        Color::new(255usize, 255, 255),
    );

    let mut distance: f64 = 0.0;

    loop {
        if distance < 20.0 {
            let rayx = px + distance * pat.cos();
            let rayy = py + distance * pat.sin();

            if map[rayx as usize + rayy as usize * MAP_WIDTH] != ' ' {
                break;
            }

            let to_draw_x = rayx * rect_w as f64;
            let to_draw_y = rayy * rect_h as f64;
            buf[to_draw_x as usize + to_draw_y as usize * TARGET_WIDTH] =
                Color::new(255usize, 255, 255);

            distance += 0.05;
        }
    }
}
