use std::{
    ffi::c_char,
    fmt::write,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use bevy_math::Vec3;
use canvas::Canvas;

mod canvas;

fn write_canvas(path: &Path, canvas: &Canvas) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, canvas.cols as u32, canvas.rows as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    let mut stream_writer = writer.stream_writer().unwrap();

    for y in 0..canvas.rows {
        for x in 0..canvas.cols {
            let index = canvas.index(x, y);
            let scaled_color = (canvas.pixels[index] * 255.0)
                .clamp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0));

            stream_writer
                .write(&[
                    scaled_color.x as u8,
                    scaled_color.y as u8,
                    scaled_color.z as u8,
                ])
                .unwrap();
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(480, 640);

    let mut chunk = 0;
    let mut c_select = false;
    let c1 = Vec3::new(1.0, 0.0, 0.0);
    let c2 = Vec3::new(0.0, 1.0, 0.0);
    for y in 0..canvas.rows {
        chunk += 1;

        if chunk > 50 {
            c_select = !c_select;
            chunk = 0;
        }

        for x in 0..canvas.cols {
            let index = canvas.index(x, y);

            if c_select {
                canvas.pixels[index] = c1;
            } else {
                canvas.pixels[index] = c2;
            }
        }
    }

    write_canvas(&PathBuf::from("xyz.png"), &canvas);
}
