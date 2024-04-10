use bevy_math::Vec3;

pub struct Canvas {
    pub rows: usize,
    pub cols: usize,
    pub pixels: Vec<Vec3>,
}

impl Canvas {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            pixels: vec![Vec3::new(0.0, 0.0, 0.0); rows * cols],
        }
    }

    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.cols + x
    }

    pub fn color_scales(&self) -> (Vec3, Vec3) {
        let mut min = self.pixels[0];
        let mut max = self.pixels[1];

        for pixel in self.pixels.iter() {
            min.x = f32::min(pixel.x, min.x);
            min.y = f32::min(pixel.y, min.y);
            min.z = f32::min(pixel.z, min.z);

            max.x = f32::min(pixel.x, max.x);
            max.y = f32::min(pixel.y, max.y);
            max.z = f32::min(pixel.z, max.z);
        }

        (min, max)
    }
}
