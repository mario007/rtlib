use std::collections::HashMap;

use crate::rgb::ImageSize;
use crate::color::TMOType;
use crate::camera::{PerspectiveCameraDescriptor, PerspectiveCamera};
use crate::materials::{MaterialDescription, BSDFInterface};
use crate::shapes::{Geometry, ShapeDescription};
use crate::lights::{LightDescription, LightInterface};

#[derive(Clone, Copy)]
pub struct AmbientOcclusionSettings {
    pub cossample: bool,
    pub maxdistance: f32
}

impl Default for AmbientOcclusionSettings {
    fn default() -> Self {
        Self { cossample: true, maxdistance: 1e38 }
    }
}

pub enum RenderingAlgorithm {
    AmbientOcclusion(AmbientOcclusionSettings),
    DirectLighting,
    PathTracer
}

pub struct Settings {
    pub resolution: ImageSize,
    pub spp: usize,
    pub rendering_algorithm: RenderingAlgorithm,
    pub tonemap: TMOType,
    pub output_fname: String,
    pub nthreads: usize
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resolution: ImageSize::new(256, 256),
            spp: 1,
            rendering_algorithm: RenderingAlgorithm::AmbientOcclusion(AmbientOcclusionSettings::default()),
            tonemap: TMOType::Linear,
            output_fname: "output.png".to_string(),
            nthreads: 1
        }
    }
}

pub struct SceneDescription {
    pub settings: Settings,
    pub camera_desc: PerspectiveCameraDescriptor,
    pub materials: Vec<MaterialDescription>,
    pub shapes: Vec<ShapeDescription>,
    pub lights: Vec<LightDescription>
}

impl SceneDescription {
    pub fn set_resolution(&mut self, resolution: ImageSize) {
        self.settings.resolution = resolution;
        self.camera_desc.resolution = resolution;
    }
}

impl Default for SceneDescription {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            camera_desc: PerspectiveCameraDescriptor::default(),
            materials: Vec::new(),
            shapes: Vec::new(),
            lights: Vec::new()
        }
    }
}


pub struct Scene {
    pub settings: Settings,
    pub camera: PerspectiveCamera,
    pub materials: Vec<Box<dyn BSDFInterface>>,
    pub geometry: Geometry,
    pub lights: Vec<Box<dyn LightInterface>>
}

impl From<SceneDescription> for Scene {
    fn from(desc: SceneDescription) -> Self {
        let mut materials = Vec::new();
        let mut mat_names = HashMap::new();
        for mat_desc in desc.materials.iter() {
            let mat = match mat_desc.create() {
                Ok(mat) => mat,
                Err(err) => panic!("{}", err)
            };
            mat_names.insert(mat_desc.name.clone(), materials.len());
            materials.push(mat);
        }
        let geometry = Geometry::from_shape_descriptions(&desc.shapes, &mat_names);
        let mut lights = Vec::new();
        for light_desc in desc.lights.iter() {
            let light = light_desc.create();
            lights.push(light);
        }
        Self {
            settings: desc.settings,
            camera: desc.camera_desc.create(),
            materials,
            geometry,
            lights
        }
    }
}
