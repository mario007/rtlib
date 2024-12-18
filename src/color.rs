use std::ops::{Add, AddAssign, Mul};

use crate::rgb::ImageSize;
use crate::tile::Tile;
use crate::rgb::{RGB8uffer, RGB8};

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

impl Mul<RGB> for RGB {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: RGB) -> Self {
        Self{r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b}
    }
}

impl Add for RGB {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Default for RGB {
    fn default() -> Self {
        RGB::zero()
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

#[derive(Debug, Copy, Clone)]
pub struct PixelSample<T> {
    pub spectrum: T,
    pub weight: f32,
}

impl<T: Default> Default for PixelSample<T> {
    fn default() -> Self {
        PixelSample{spectrum: T::default(), weight: 0.0}
    }
}

impl<T: AddAssign> AddAssign for PixelSample<T> {
    fn add_assign(&mut self, rhs: PixelSample<T>) {
        self.spectrum += rhs.spectrum;
        self.weight += rhs.weight;
    }
}

impl<T: Mul<f32, Output = T> + Into<RGB>> From<PixelSample<T>> for RGB {
    fn from(sample: PixelSample<T>) -> Self {
        if sample.weight == 0.0 {
            RGB::zero()
        } else {
            (sample.spectrum * sample.weight.recip()).into()
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


pub struct AccumlationBuffer<PixelSample> {
    size: ImageSize,
    buffer: Vec<PixelSample>,
}

impl<T: Default + Clone + Copy + AddAssign + Into<RGB> + Mul<f32, Output = T>> AccumlationBuffer<PixelSample<T>> {
    pub fn new(size: ImageSize) -> Self {
        let buffer = vec![PixelSample::default(); size.width * size.height];
        Self { size, buffer}
    }

    pub fn add(&mut self, x: usize, y: usize, value: &T) {
        let index = y * self.size.width + x;
        let sample = PixelSample{spectrum: *value, weight: 1.0};
        self.buffer[index] += sample;
    }

    pub fn set(&mut self, x: usize, y: usize, value: &T) {
        let index = y * self.size.width + x;
        let sample = PixelSample{spectrum: *value, weight: 1.0};
        self.buffer[index] = sample;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&PixelSample<T>> {
        let index = y * self.size.width + x;
        self.buffer.get(index)
    }

    pub fn to_rgb8_buffer(&self, tmo_type: &TMOType) -> RGB8uffer {
        let vals: Vec<RGB8> = self.buffer.iter().map(
            |sample| tone_map(tmo_type,&(*sample).into()).into()).collect();
        RGB8uffer::from((self.size.width, vals))
    }

    pub fn add_accumulation_tile_buffer(&mut self, tile_buffer: &AccumlationTileBuffer<PixelSample<T>>) {
        let tile = tile_buffer.tile;
        let padding = tile_buffer.padding;
        let left = (tile.x1 as i32 - padding).max(0) as usize;
        let right = (tile.x2 as i32 + padding).min(self.size.width as i32) as usize;
        let top = (tile.y1 as i32 - padding).max(0) as usize;
        let bottom = (tile.y2 as i32 + padding).min(self.size.height as i32) as usize;
        
        let mut curx = 0;
        let mut cury = 0;
        for y in top..bottom {
            for x in left..right {
                let src_index = curx + cury * tile_buffer.width;
                let dst_index = x + y * self.size.width;
                self.buffer[dst_index] += tile_buffer.buffer[src_index];
                curx += 1;
            }
            curx = 0;
            cury += 1;
        }
    }
}

pub struct AccumlationTileBuffer<PixelSample> {
    tile: Tile,
    width: usize,
    height: usize,
    buffer: Vec<PixelSample>,
    filter_radius: Option<f32>,
    padding: i32,
}


impl<T: Default + Clone + Copy + AddAssign + Into<RGB> + Mul<f32, Output = T>> AccumlationTileBuffer<PixelSample<T>> {
    pub fn new(tile: Tile, filter_radius: Option<f32>, max_width: usize, max_height: usize) -> Self {
        match filter_radius {
            Some(radius) => {
                let padding = (0.5 + radius) as i32;
                let left = (tile.x1 as i32 - padding).max(0) as usize;
                let right = (tile.x2 as i32 + padding).min(max_width as i32) as usize;
                let top = (tile.y1 as i32 - padding).max(0) as usize;
                let bottom = (tile.y2 as i32 + padding).min(max_height as i32) as usize;

                let width = right - left;
                let height = bottom - top;
                let buffer = vec![PixelSample::default(); width * height];
                Self { tile, width, height, buffer, filter_radius: Some(radius), padding}
            }
            None => {
                let size = tile.size();
                let buffer = vec![PixelSample::default(); size.width * size.height];
                Self { tile, width: size.width, height: size.height, buffer, filter_radius: None, padding: 0}
            }
        }
    }

    pub fn add(&mut self, ix: usize, iy: usize, x: f32, y: f32, value: &T,
               calculate_weight_fn: &dyn Fn(f32, f32) -> f32) {
        
        let radius = match self.filter_radius {
            Some(radius) => radius,
            None => {
                // Convert to local tile coordinates
                let local_x = ix - self.tile.x1;
                let local_y = iy - self.tile.y1;
                let sample = PixelSample{spectrum: *value, weight: 1.0};
                let index = local_y * self.width + local_x;
                self.buffer[index] += sample;
                return;
            }
        };

        // Convert to local tile coordinates
        let local_x = x - self.tile.x1 as f32;
        let local_y = y - self.tile.y1 as f32;

        // Calculate pixel extent for the filter
        let x_min = ((local_x - radius).floor() as i32).max(0);
        let x_max = ((local_x + radius).ceil() as i32).min(self.width as i32);
        let y_min = ((local_y - radius).floor() as i32).max(0);
        let y_max = ((local_y + radius).ceil() as i32).min(self.height as i32);

        for py in y_min..y_max {
            for px in x_min..x_max {
                // Calculate distance from sample to pixel center
                let dx = local_x - (px as f32 + 0.5);
                let dy = local_y - (py as f32 + 0.5);
                let weight = calculate_weight_fn(dx, dy);
                if weight > 0.0 {
                    let index = py * self.width as i32 + px;
                    let spectrum = *value * weight;
                    let sample = PixelSample{spectrum, weight};
                    self.buffer[index as usize] += sample;
                }
            }
        }
    }
}
