use crate::color::RGB;
use crate::vec::Vec3;
use crate::vec::Normal;
use crate::frame::Frame;
use crate::samplings::sample_cos_hemisphere;
use crate::samplers::SamplerInterface;

pub struct BSDFEvalSample {
    pub color: RGB,
    pub pdfw: f32
}

pub struct BSDFSample {
    pub wi: Vec3,
    pub color: RGB,
    pub pdfw: f32
}

pub trait BSDFInterface {
    fn eval(&self, wo: Vec3, normal: Normal, wi: Vec3) -> Option<BSDFEvalSample>;
    fn sample(&self, wo: Vec3, normal: Normal, sampler: &mut Box<dyn SamplerInterface>) -> Option<BSDFSample>;
    fn is_emissive(&self) -> bool {
        false
    }
    fn emssion(&self, _wo: Vec3, _normal: Normal, _back_side: bool) -> RGB {
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
    fn eval(&self, wo: Vec3, normal: Normal, wi: Vec3) -> Option<BSDFEvalSample> {
        if !((normal * wi) * (normal * wo) > 0.0) {
            return None
        }
        let color = self.reflectance * std::f32::consts::FRAC_1_PI;
        let pdfw = (normal * wi).abs() * std::f32::consts::FRAC_1_PI;
        Some(BSDFEvalSample{color, pdfw})
    }

    fn sample(&self, wo: Vec3, normal: Normal, sampler: &mut Box<dyn SamplerInterface>) -> Option<BSDFSample> {
        let (u1, u2) = sampler.next_2d();
        let sample_direction = sample_cos_hemisphere(u1, u2);
        let wi = Frame::from(normal).to_world(sample_direction.direction).normalize();
        if !((normal * wi) * (normal * wo) > 0.0) {
            return None
        }
        let color = self.reflectance * std::f32::consts::FRAC_1_PI;
        let pdfw = sample_direction.pdfw;
        if pdfw == 0.0 {
            return None
        }
        Some(BSDFSample{wi, color, pdfw})
    }
}

pub struct EmissiveMatteMaterial {
    reflectance: RGB,
    emission: RGB
}

impl EmissiveMatteMaterial {
    pub fn new(reflectance: RGB, emission: RGB) -> EmissiveMatteMaterial {
        EmissiveMatteMaterial {reflectance, emission}
    }
}

impl BSDFInterface for EmissiveMatteMaterial {
    fn eval(&self, wo: Vec3, normal: Normal, wi: Vec3) -> Option<BSDFEvalSample> {
        if !((normal * wi) * (normal * wo) > 0.0) {
            return None
        }
        let color = self.reflectance * std::f32::consts::FRAC_1_PI;
        let pdfw = (normal * wi).abs() * std::f32::consts::FRAC_1_PI;
        Some(BSDFEvalSample{color, pdfw})
    }

    fn sample(&self, wo: Vec3, normal: Normal, sampler: &mut Box<dyn SamplerInterface>) -> Option<BSDFSample> {
        let (u1, u2) = sampler.next_2d();
        let sample_direction = sample_cos_hemisphere(u1, u2);
        let wi = Frame::from(normal).to_world(sample_direction.direction).normalize();
        if !((normal * wi) * (normal * wo) > 0.0) {
            return None
        }
        let color = self.reflectance * std::f32::consts::FRAC_1_PI;
        let pdfw = sample_direction.pdfw;
        if pdfw == 0.0 {
            return None
        }
        Some(BSDFSample{wi, color, pdfw})
    }

    fn is_emissive(&self) -> bool {
        true
    }
    fn emssion(&self, _wo: Vec3, _normal: Normal, back_side: bool) -> RGB {
        if back_side {
            return RGB::zero();
        }
        self.emission
    }
}


pub enum MaterialType {
    Matte,
    EmissiveMatte
}

pub struct MaterialDescription {
    pub name: String,
    pub typ: MaterialType,
    pub diffuse: RGB,
    pub emission: RGB
}

impl MaterialDescription {
    pub fn create(&self) -> Result<Box<dyn BSDFInterface>, String> { 
        match self.typ {
            MaterialType::Matte => Ok(Box::new(MatteMaterial::new(self.diffuse))),
            MaterialType::EmissiveMatte => Ok(Box::new(EmissiveMatteMaterial::new(self.diffuse, self.emission)))
        }
    }
}

impl Default for MaterialDescription {
    fn default() -> Self {
        MaterialDescription {
            name: "matte".to_string(),
            typ: MaterialType::Matte,
            diffuse: RGB::new(0.5, 0.5, 0.5),
            emission: RGB::zero()
        }
    }
}
