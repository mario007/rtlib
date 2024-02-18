use std::error::Error;
use std::path::Path;

//extern crate image;

/// A RGB color.
///
#[derive(Debug, Copy, Clone)]
pub struct RGB {
    /// The red component of the color.
    pub red: u8,
    /// The green component of the color.
    pub green: u8,
    /// The blue component of the color.
    pub blue: u8
}

pub struct RGBuffer {
    width: usize,
    height: usize,
    pixels: Vec<RGB>
}

impl RGBuffer {
    pub fn new(width: usize, height: usize) -> RGBuffer {
        let pixels = vec![RGB{red:0, green:0, blue:0}; width * height];
        RGBuffer {width, height, pixels}
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&RGB> {
        return self.pixels.get(y * self.width + x)
    }
    
    pub fn set(&mut self, x: usize, y: usize, rgb: &RGB) {
        if x >= self.width {
            panic!("index out of bounds: the width is {} but the index is {}", self.width, x);
        }
        if y >= self.height {
            panic!("index out of bounds: the height is {} but the index is {}", self.height, y);
        }
        self.pixels[y * self.width + x] = *rgb;
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let output: Vec<u8> = self.pixels.iter().flat_map(
            |val| [val.red, val.green, val.blue]).collect();

        let result = image::save_buffer(path,
                                        &output[0..output.len()],
                                        self.width as u32,
                                        self.height as u32,
                                        image::ColorType::Rgb8);

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rng::{PCGRng, Rng};

    #[test]
    fn sampling_pixels() {
        let mut rng = PCGRng::new(0xf12456955, 0x454555);
        
        let color = RGB{red:0, green:0, blue:255};
        let pixel_size = 256;
        let width = 3;
        let height = 3;
        let res_x = width * pixel_size;
        let res_y = height * pixel_size;
        let mut col_buffer = RGBuffer::new(res_x, res_y);
        for j in 1..height {
            for i in 0..res_x {
                col_buffer.set(i, j * pixel_size, &color);
            }
        }
        for i in 1..width {
            for j in 0..res_y {
                col_buffer.set(i * pixel_size, j, &color);
            }
        }
        let samples_per_pixel = 256;
        let red = RGB{red:255, green:0, blue:0};
        for _s in 0..samples_per_pixel {
            for i in 0..width {
                for j in 0..height {
                    let px = rng.rand_range(pixel_size as u32) as usize;
                    let py = rng.rand_range(pixel_size as u32) as usize;
                    col_buffer.set(i * pixel_size + px, j * pixel_size + py, &red);
                }
            }
        }
        let _result = col_buffer.save("samples.png");
    }
}
