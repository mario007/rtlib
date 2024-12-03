use crate::hash;
use crate::tile::Tile;
use crate::math::permutation_element;
use crate::rng::{PCGRng, Rng};


pub trait SamplerInterface {
    fn next_1d(&mut self) -> f32;
    fn next_2d(&mut self) -> (f32, f32);
    fn sample_pixel(&mut self, x: usize, y: usize, iteration: usize) -> (f32, f32);
    fn initialize(&mut self, tile: &Tile, iteration: u32);
}

pub struct RandomPathSampler {
    seed: u64,
    pcg_rng: PCGRng,
}

impl RandomPathSampler {
    pub fn new(seed: u64) -> RandomPathSampler {
        let pcg_rng = PCGRng::new(seed, 0);
        RandomPathSampler{seed, pcg_rng}
    }
}

impl SamplerInterface for RandomPathSampler {
    fn next_1d(&mut self) -> f32 {
        self.pcg_rng.rand_f32()
    }

    fn next_2d(&mut self) -> (f32, f32) {
        (self.next_1d(), self.next_1d())
    }

    fn sample_pixel(&mut self, _x: usize, _y: usize, _iteration: usize) -> (f32, f32) {
        self.next_2d()
    }

    fn initialize(&mut self, tile: &Tile, iteration: u32) {
        let seed = hash!(self.seed, tile.x1, tile.y1);
        self.pcg_rng = PCGRng::new(seed, iteration as u64);
    }
}

pub struct StratifiedPathSampler {
    seed: u64,
    jitter: bool,
    xsamples: u32,
    ysamples: u32,
    pcg_rng: PCGRng,

    x: u32,
    y: u32,
    iteration: u32,
    dimension: u32,
}

impl StratifiedPathSampler {
    pub fn new(seed: u64, xsamples: u32, ysamples: u32, jitter: bool) -> StratifiedPathSampler {
        let pcg_rng = PCGRng::new(seed, 0);
        StratifiedPathSampler{seed, jitter, xsamples, ysamples, pcg_rng, x: 0, y: 0, iteration: 0, dimension: 0}
    }
}

impl SamplerInterface for StratifiedPathSampler {

    fn next_1d(&mut self) -> f32 {
        let hash = hash!(self.seed, self.x, self.y, self.dimension);
        let total = self.xsamples * self.ysamples;
        let stratum = permutation_element(self.iteration, total, hash as u32);
        self.dimension += 1;

        let dx = if self.jitter {
            self.pcg_rng.rand_f32()
        } else {
            0.5
        };

        (stratum as f32 + dx) / total as f32
    }

    fn next_2d(&mut self) -> (f32, f32) {
        let hash = hash!(self.seed, self.x, self.y, self.dimension);
        let total = self.xsamples * self.ysamples;
        let stratum = permutation_element(self.iteration, total, hash as u32);
        self.dimension += 2;
    
        let x = stratum % self.xsamples;
        let y = stratum / self.xsamples;
        let (dx, dy) = if self.jitter {
            (self.pcg_rng.rand_f32(), self.pcg_rng.rand_f32())
        } else {
            (0.5, 0.5)
        };
        let px = (x as f32 + dx) / self.xsamples as f32;
        let py = (y as f32 + dy) / self.ysamples as f32;
        
        (px, py)
    }

    fn sample_pixel(&mut self, x: usize, y: usize, iteration: usize) -> (f32, f32) {
        self.x = x as u32;
        self.y = y as u32;
        self.iteration = iteration as u32;
        self.dimension = 0;
        self.next_2d()
    }

    fn initialize(&mut self, tile: &Tile, iteration: u32) {
        let seed = hash!(self.seed, tile.x1, tile.y1);
        self.pcg_rng = PCGRng::new(seed, iteration as u64);
    }
}
