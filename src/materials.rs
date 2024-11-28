use crate::color::RGB;
use crate::rng::Rng;
use crate::vec::Vec3;
use crate::vec::Normal;
use crate::rng::PCGRng;
use crate::frame::Frame;
use crate::samplings::sample_cos_hemisphere;

pub struct BSDFEvalSample {
    pub color: RGB,
    pub pdfw: f32
}

pub struct BSDFSample {
    pub direction: Vec3,
    pub color: RGB,
    pub pdfw: f32
}

pub trait BSDFInterface {
    fn eval(&self, wo: Vec3, normal: Normal, wi: Vec3) -> Option<BSDFEvalSample>;
    fn sample(&self, wo: Vec3, normal: Normal, rng: &mut PCGRng) -> Option<BSDFSample>;
    fn is_emissive(&self) -> bool {
        false
    }
    fn emssion(&self) -> RGB {
        RGB::zero()
    }
}

pub struct MatteMaterial {
    reflectance: RGB
}

impl MatteMaterial {
    pub fn new(reflectance: RGB) -> MatteMaterial {
        MatteMaterial {reflectance}
    }
}

impl BSDFInterface for MatteMaterial {
    fn eval(&self, _wo: Vec3, normal: Normal, wi: Vec3) -> Option<BSDFEvalSample> {
        let color = self.reflectance * std::f32::consts::FRAC_1_PI;
        let pdfw = (normal * wi).abs() * std::f32::consts::FRAC_1_PI;
        Some(BSDFEvalSample{color, pdfw})
    }

    fn sample(&self, _wo: Vec3, normal: Normal, rng: &mut PCGRng) -> Option<BSDFSample> {
        let sample_direction = sample_cos_hemisphere(rng.rand_f32(), rng.rand_f32());
        let direction = Frame::from(normal).to_world(sample_direction.direction).normalize();
        let color = self.reflectance * std::f32::consts::FRAC_1_PI;
        let pdfw = sample_direction.pdfw;
        if pdfw == 0.0 {
            return None
        }
        Some(BSDFSample{direction, color, pdfw})
    }
}

pub enum MaterialType {
    Matte,
}

pub struct MaterialDescription {
    pub name: String,
    pub typ: MaterialType,
    pub diffuse: RGB
}

impl MaterialDescription {
    pub fn create(&self) -> Result<Box<dyn BSDFInterface>, String> { 
        match self.typ {
            MaterialType::Matte => Ok(Box::new(MatteMaterial::new(self.diffuse))),
        }
    }
}

impl Default for MaterialDescription {
    fn default() -> Self {
        MaterialDescription {
            name: "matte".to_string(),
            typ: MaterialType::Matte,
            diffuse: RGB::new(0.5, 0.5, 0.5)
        }
    }
}

