use crate::vec3::Color;
use std::ops::AddAssign;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
}

impl Image {
    pub fn zeroes(width: u32, height: u32) -> Image {
        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Color::zeroes());
            }
            pixels.push(row);
        }
        Image {
            width,
            height,
            pixels,
        }
    }
}

impl AddAssign for Image {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i as usize][j as usize] += rhs.pixels[i as usize][j as usize];
            }
        }
    }
}
