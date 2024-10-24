
use std::ops::{Add, AddAssign, Mul};

use crate::tile::Tile;
use crate::rgb::{RGBuffer, RGB8};

#[derive(Debug, Copy, Clone)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn zero() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Mul<f32> for RGB {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self{r: self.r * rhs, g: self.g * rhs, b: self.b * rhs}
    }
}

impl Mul<RGB> for f32 {
    type Output = RGB;

    #[inline(always)]
    fn mul(self, rhs: RGB) -> RGB {
        rhs * self
    }
}

impl Add for RGB {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}


impl From<RGB> for RGB8 {
    fn from(rgb: RGB) -> Self {
        let red = (rgb.r * 256.0) as u8;
        let green = (rgb.g * 256.0) as u8;
        let blue = (rgb.b * 256.0) as u8;
        RGB8 { red, green, blue }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct RGBPixelSample {
    pub spectrum: RGB,
    pub weight: f32,
}

impl RGBPixelSample {
    pub fn new(rgb: RGB, weight: f32) -> Self {
        Self { spectrum: rgb, weight }
    }

    pub fn zero() -> Self {
        RGBPixelSample::new(RGB::zero(), 0.0)
    }
}

impl AddAssign for RGBPixelSample {

    #[inline(always)]
    fn add_assign(&mut self, rhs: RGBPixelSample) {
        self.spectrum.r += rhs.spectrum.r;
        self.spectrum.g += rhs.spectrum.g;
        self.spectrum.b += rhs.spectrum.b;
        self.weight += rhs.weight;
    }
}

impl Default for RGBPixelSample {
    fn default() -> Self {
        RGBPixelSample::zero()
    }
}

impl From<RGBPixelSample> for RGB {
    fn from(sample: RGBPixelSample) -> Self {
        if sample.weight == 0.0 {
            RGB::zero()
        } else {
            sample.spectrum * sample.weight.recip()
        }
    }
}

pub enum TMOType {
    Linear,
    Gamma,
    Reinhard,

}

// http://filmicworlds.com/blog/filmic-tonemapping-operators/
fn tone_map(tmo_type: &TMOType, spec: &RGB) -> RGB {
    const INV_GAMMA: f32 = 1.0/2.2;

    fn gamma_correct(value: f32) -> f32 {
        value.powf(INV_GAMMA)
    }
    match tmo_type {
        TMOType::Linear => *spec,
        TMOType::Gamma => {RGB::new(
            gamma_correct(spec.r),
            gamma_correct(spec.g), 
            gamma_correct(spec.b)
        )},
        TMOType::Reinhard => {RGB::new(
            gamma_correct(spec.r / (spec.r + 1.0)),
            gamma_correct(spec.g / (spec.g + 1.0)),
            gamma_correct(spec.b / (spec.b + 1.0))
        )}
    }
}


pub struct AccumlationBuffer<T> {
    tile: Tile,
    width: usize,
    buffer: Vec<T>,
}

impl<T: Default + Clone + Copy + AddAssign + Into<RGB>> AccumlationBuffer<T> {
    pub fn new(tile: Tile) -> Self {
        let img_size = tile.size();
        let buffer = vec![T::default(); img_size.width * img_size.height];
        Self { tile, width: img_size.width, buffer}
    }

    pub fn add(&mut self, x: usize, y: usize, value: &T) {
        let index = y * self.width + x;
        self.buffer[index] += *value;
    }

    pub fn set(&mut self, x: usize, y: usize, value: &T) {
        let index = y * self.width + x;
        self.buffer[index] = *value;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = y * self.width + x;
        self.buffer.get(index)
    }

    pub fn to_rgb8_buffer(&self, tmo_type: &TMOType) -> RGBuffer {
        let vals: Vec<RGB8> = self.buffer.iter().map(
            |sample| tone_map(tmo_type,&(*sample).into()).into()).collect();
        RGBuffer::from((self.width, vals))
    }
}

