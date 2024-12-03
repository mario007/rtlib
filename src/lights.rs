use crate::vec::Point3;
use crate::color::RGB;
use crate::vec::Vec3;

pub struct LightSample {
    pub intensity: RGB,
    pub position: Point3,
    pub wi: Vec3,
    pub pdfa: f32,
    pub cos_theta: f32
}

pub trait LightInterface {
    fn illuminate(&self, hit: Point3) -> Option<LightSample>;
    fn is_delta_light(&self) -> bool;
    fn is_area_light(&self) -> bool {
        false
    }
}

pub struct PointLight {
    intensity: RGB,
    position: Point3
}

impl PointLight {
    pub fn new(intensity: RGB, position: Point3) -> PointLight {
        PointLight { intensity, position }
    }
}

impl LightInterface for PointLight {
    fn illuminate(&self, hit: Point3) -> Option<LightSample> {
        let direction_to_light = self.position - hit;
        let wi = direction_to_light.normalize();
        let intensity = self.intensity * direction_to_light.length_sqr().recip();
        let position = self.position;
        let pdfa = 1.0;
        let cos_theta = 1.0;
        Some(LightSample { intensity, position, wi, pdfa, cos_theta})
    }

    fn is_delta_light(&self) -> bool {
        true
    }
}

pub enum LightType {
    Point
}

pub struct LightDescription {
    pub typ: LightType,
    pub intensity: RGB,
    pub position: Point3
}

impl LightDescription {
    pub fn create(&self) -> Box<dyn LightInterface> {
        match self.typ {
            LightType::Point => Box::new(PointLight::new(self.intensity, self.position))
        }
    }
}

impl Default for LightDescription {
    fn default() -> Self {
        Self {
            typ: LightType::Point,
            intensity: RGB::new(1.0, 1.0, 1.0),
            position: Point3::new(0.0, 0.0, 0.0)
        }
    }
}
