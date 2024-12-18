use crate::color::RGB;
use crate::vec::{Point3, Vec3, Normal, Point2};
use std::path::PathBuf;
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::scene::SceneDescription;
use std::collections::HashSet;
use crate::pbrt_v4_tokenizer::PBRTTokenizer;
use crate::transformations::Transformation;
use crate::scene::RenderingAlgorithm;
use std::str::FromStr;
use std::fmt::Display;
use crate::rgb::ImageSize;
use crate::materials::MaterialDescription;
use crate::materials::MaterialType;
use crate::lights::LightDescription;
use crate::lights::LightType;
use crate::shapes::ShapeDescription;
use crate::scene::{AmbientOcclusionProperties, RandomWalkProperties};
use crate::matrix::Matrix4x4;
use crate::scene::{Sampler, RandomSamplerSettings, StratifiedSamplerSettings};
use crate::shapes::{MeshDescription, SphereDescription};
use crate::filter::{FilterDescriptor, FilterType};


struct ParseState {
    transformations: Vec<Transformation>,
    materials: Vec<String>,
    area_lights: Vec<String>,
    current_path: PathBuf,
    directives: HashSet<&'static str>,
}

impl ParseState {
    pub fn new() -> Self {
        let transformations = vec![Transformation::identity()];
        let materials = Vec::new();
        let area_lights = Vec::new();
        let current_path = PathBuf::new();
        let directives: HashSet<_> = vec!["LookAt", "Camera", "Sampler", "Integrator", "Film", "PixelFilter",
        "WorldBegin", "AttributeBegin", "AttributeEnd", "LightSource", "AreaLightSource", "Texture",
        "Material", "MakeNamedMaterial", "NamedMaterial", "Include", "Accelerator", "Shape",
        "Scale", "Translate", "Rotate", "Identity", "Transform", "ConcatTransform"].into_iter().collect();
        Self {
            transformations,
            materials,
            area_lights,
            current_path,
            directives
        }
    }

    pub fn push_state(&mut self) {
        self.transformations.push(self.current_transformation());
        self.materials.push(self.current_material());
        if !self.area_lights.is_empty() {
            self.area_lights.push(self.area_lights.last().expect("No area light exist!").clone());
        }
    }

    pub fn pop_state(&mut self) {
        self.transformations.pop();
        self.materials.pop();
        self.area_lights.pop();
    }

    pub fn current_transformation(&self) -> Transformation {
        self.transformations[self.transformations.len() - 1]
    }

    pub fn current_material(&self) -> String {
        self.materials.last().expect("No material exist!").clone()
    }

    pub fn set_transformation(&mut self, transformation: Transformation) {
        if let Some(last) = self.transformations.last_mut() {
            *last = transformation;
        }
    }

    pub fn set_material(&mut self, material: String) {
        if self.materials.is_empty() {
            self.materials.push(material);
        } else {
            let index = self.materials.len() - 1;
            self.materials[index] = material;
        }
    }

    pub fn set_area_light(&mut self, material: String) {
        if self.area_lights.is_empty() {
            self.area_lights.push(material);
        } else {
            let index = self.area_lights.len() - 1;
            self.area_lights[index] = material;
        }
    }

    pub fn is_directive(&self, directive: &str) -> bool {
        self.directives.contains(directive)
    }
}

pub fn parse_pbrt_v4_input_file<P: AsRef<Path>>(path: P) -> Result<SceneDescription, Box<dyn Error>> {
    let mut state = ParseState::new();
    state.current_path = path.as_ref().to_path_buf();
    let contents = fs::read_to_string(path)?;
    let mut scene = SceneDescription::default();
    parse_input_string(&contents, &mut scene, &mut state)?;
    Ok(scene)
}

fn parse_input_string(text: &str, scene: &mut SceneDescription, state: &mut ParseState) -> Result<(), Box<dyn Error>> {

    let mut ct = PBRTTokenizer::new(text);

    let mut cur_directive = match ct.next() {
        Some(token) => token.to_string(),
        None => return Ok(())
    };

    loop {
        let new_directive: Option<String> = match cur_directive.as_str() {
            "LookAt" => process_look_at(&mut ct, scene, state)?,
            "Camera" => process_camera(&mut ct, scene, state)?,
            "Integrator" => process_integrator(&mut ct, scene, state)?,
            "Film" => process_film(&mut ct, scene, state)?,
            "Sampler" => process_sampler(&mut ct, scene, state)?,
            "PixelFilter" => process_filter(&mut ct, scene, state)?,
            "WorldBegin" => process_world_begin(&mut ct, scene, state)?,
            "AttributeBegin" => process_attribute_begin(&mut ct, scene, state)?,
            "AttributeEnd" => process_attribute_end(&mut ct, scene, state)?,
            "LightSource" => process_light(&mut ct, scene, state)?,
            "AreaLightSource" => process_area_light_source(&mut ct, scene, state)?,
            // "Texture" => process_texture(tokens, scene, state)?,
            "Material" => process_material(&mut ct, scene, state)?,
            "Shape" => process_shape(&mut ct, scene, state)?,
            "MakeNamedMaterial" => process_make_named_material(&mut ct, scene, state)?,
            "NamedMaterial" => process_named_material(&mut ct, scene, state)?,
            // "Accelerator" => process_accelerator(tokens, scene, state)?,
            "Scale" => process_scale_transform(&mut ct, scene, state)?,
            "Translate" => process_translate_transform(&mut ct, scene, state)?,
            // "Rotate" => process_rotate_transform(tokens, scene, state)?,
            "Identity" => process_identity_transform(&mut ct, scene, state)?,
            "Transform" => process_transform(&mut ct, scene, state)?,
            "ConcatTransform" => process_concat_transform(&mut ct, scene, state)?,
            _=> return Err(format!("Unsupported directive to process: {}", cur_directive).into())
        };
        match new_directive {
            Some(directive) => cur_directive = directive.clone(),
            None => return Ok(())
        }
    }
}

#[allow(clippy::manual_map)]
fn next_directive(tokenizer: &mut PBRTTokenizer) -> Option<String> {
    match tokenizer.next() {
        Some(token) => Some(token.to_string()),
        None => None
    }
}

fn process_look_at(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
                   state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let v0 = parse_f32(tokenizer, "LookAt:eye x ")?;
    let v1 = parse_f32(tokenizer, "LookAt:eye y ")?;
    let v2 = parse_f32(tokenizer, "LookAt:eye z ")?;
    let eye = Point3::new(v0, v1, v2);
    let v0 = parse_f32(tokenizer, "LookAt:look_at x ")?;
    let v1 = parse_f32(tokenizer, "LookAt:look_at y ")?;
    let v2 = parse_f32(tokenizer, "LookAt:look_at z ")?;
    let look_at = Point3::new(v0, v1, v2);
    let v0 = parse_f32(tokenizer, "LookAt:up x ")?;
    let v1 = parse_f32(tokenizer, "LookAt:up y ")?;
    let v2 = parse_f32(tokenizer, "LookAt:up z ")?;
    let up = Vec3::new(v0, v1, v2);
    let tranformation = state.current_transformation() * Transformation::look_at(eye, look_at, up);
    state.set_transformation(tranformation);
    Ok(next_directive(tokenizer))
}

fn process_translate_transform(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
    state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let v0 = parse_f32(tokenizer, "Translate: x ")?;
    let v1 = parse_f32(tokenizer, "Translate: y ")?;
    let v2 = parse_f32(tokenizer, "Translate: z ")?;
    let delta = Vec3::new(v0, v1, v2);
    let transformation = state.current_transformation() * Transformation::translate(&delta);
    state.set_transformation(transformation);
    Ok(next_directive(tokenizer))
}

fn process_scale_transform(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
    state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let v0 = parse_f32(tokenizer, "Scale: x ")?;
    let v1 = parse_f32(tokenizer, "Scale: y ")?;
    let v2 = parse_f32(tokenizer, "Scale: z ")?;
    let transformation = state.current_transformation() * Transformation::scale(v0, v1, v2);
    state.set_transformation(transformation);
    Ok(next_directive(tokenizer))
}

fn process_transform(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
    state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let values = parse_f32_array(tokenizer, "Transform: ")?;
    if values.len() != 16 {
        return Err("Transform: Exactly 16 values expected!".to_string().into())
    }
    let m = [
        [values[0], values[4], values[8],  values[12]],
        [values[1], values[5], values[9],  values[13]],
        [values[2], values[6], values[10], values[14]],
        [values[3], values[7], values[11], values[15]],
    ];
    let transform = Transformation::from(Matrix4x4::new(m));
    state.set_transformation(transform);
    Ok(next_directive(tokenizer))
}

fn process_concat_transform(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
    state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let values = parse_f32_array(tokenizer, "ConcatTransform: ")?;
    if values.len() != 16 {
        return Err("ConcatTransform: Exactly 16 values expected!".to_string().into())
    }
    let m = [
        [values[0], values[4], values[8],  values[12]],
        [values[1], values[5], values[9],  values[13]],
        [values[2], values[6], values[10], values[14]],
        [values[3], values[7], values[11], values[15]],
    ];
    let transform = Transformation::from(Matrix4x4::new(m));
    state.set_transformation(state.current_transformation() * transform);
    Ok(next_directive(tokenizer))
}

fn process_identity_transform(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
    state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    state.set_transformation(Transformation::identity());
    Ok(next_directive(tokenizer))
}

fn process_camera(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                  state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let camera_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Missing camera type token!".to_string().into())
    };

    match camera_type {
        "perspective" => process_perspective_camera(tokenizer, scene, state),
        _ => Err(format!("Camera: Unsupported camera type - {}", camera_type).into())
    }
}


fn process_perspective_camera(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut fov: f32 = 90.0;                           
    let result = loop {
        let token = match tokenizer.next() {
            Some(token) => token.trim(),
            None => break None
        };
        if state.is_directive(token) {
            break Some(token.to_string());
        }
        match token {
            "float fov" => fov = extract_value(tokenizer, "Perspective Camera::fov - ")?,
            _ => return Err(format!("Unsupported parameter in Perspective Camera: {}", token).into())
        }

    };
    scene.camera_desc.fov = fov;
    scene.camera_desc.camera_to_world = Some(state.current_transformation().inverse());
    Ok(result)
}

fn process_integrator(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Integrator: Type of integrator not specified!".into())
    };
    match token {
        "direct_lighting" => direct_lighting_integrator(tokenizer, scene, state),
        "ambientocclusion" => ambientocclusion_integrator(tokenizer, scene, state),
        "randomwalk" => randomwalk_integrator(tokenizer, scene, state),
        _=> Err(format!("Unsupported integrator type {}", token).into())
    }
}

fn direct_lighting_integrator(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                                      _state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    scene.settings.rendering_algorithm = RenderingAlgorithm::DirectLighting;                                   
    Ok(next_directive(tokenizer))
}

fn ambientocclusion_integrator(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut settings = AmbientOcclusionProperties::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "bool cossample" => settings.cossample = extract_value(tokenizer, "Ambientocclusion::cossample - ")?,
            "float maxdistance" => settings.maxdistance = extract_value(tokenizer, "Ambientocclusion::maxdistance - ")?,
            _ => return Err(format!("Unsupported parameter in ambient occlusion integrator: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.settings.rendering_algorithm = RenderingAlgorithm::AmbientOcclusion(settings);                                                                      
    Ok(result)
}

fn randomwalk_integrator(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                         state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut settings = RandomWalkProperties::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "integer maxdepth" => settings.maxdepth = extract_value(tokenizer, "Randomwalk::maxdepth - ")?,
            _ => return Err(format!("Unsupported parameter in random walk integrator: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.settings.rendering_algorithm = RenderingAlgorithm::RandomWalk(settings);                                                                      
    Ok(result)
}


fn process_attributes(tokenizer: &mut PBRTTokenizer,
                      state: &mut ParseState,
                      process_attribute: &mut dyn FnMut(&mut PBRTTokenizer, &str)-> Result<(), Box<dyn Error>>) -> Result<Option<String>, Box<dyn Error>> {
    let result = loop {
        let token = match tokenizer.next() {
            Some(token) => token.trim(),
            None => break None
        };
        if state.is_directive(token) {
            break Some(token.to_string());
        }
        process_attribute(tokenizer, token)?;
    };
    Ok(result)
}

fn process_film(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                  state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let film_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Missing film type token!".to_string().into())
    };

    match film_type {
        "rgb" => process_rgb_film(tokenizer, scene, state),
        _ => Err(format!("Camera: Unsupported film type - {}", film_type).into())
    }
}

fn process_rgb_film(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                    state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut xresolution: usize = 1280;
    let mut yresolution: usize = 720;
    let mut filename: String = "".to_string();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "integer xresolution" => xresolution = extract_value(tokenizer, "Film::xresolution - ")?,
            "integer yresolution" => yresolution = extract_value(tokenizer, "Film::yresolution - ")?,
            "string filename" => filename = extract_value(tokenizer, "Film::filename - ")?,
            _ => return Err(format!("Unsupported parameter in Rgb film: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.set_resolution(ImageSize::new(xresolution, yresolution));
    scene.settings.output_fname = filename;
    Ok(result)
}

fn process_sampler(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                  state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let sampler_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Missing sampler type token!".to_string().into())
    };

    match sampler_type {
        "independent" => process_independent_sampler(tokenizer, scene, state),
        "halton" => process_halton_sampler(tokenizer, scene, state),
        "paddedsobol" => process_independent_sampler(tokenizer, scene, state),
        "sobol" => process_independent_sampler(tokenizer, scene, state),
        "stratified" => process_stratified_sampler(tokenizer, scene, state),
        "zsobol" => process_independent_sampler(tokenizer, scene, state),
        _ => Err(format!("Sampler: Unsupported sampler type - {}", sampler_type).into())
    }
}

fn process_independent_sampler(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut pixelsamples: usize = 1;
    let mut settings = RandomSamplerSettings::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "integer seed" => settings.seed = extract_value(tokenizer, "Sampler::seed - ")?,
            "integer pixelsamples" => pixelsamples = extract_value(tokenizer, "Sampler::pixelsamples - ")?,
            _ => return Err(format!("Unsupported parameter in independent sampler: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.sampler = Some(Sampler::Random(settings));
    scene.settings.spp = pixelsamples;
    Ok(result)
}

fn process_stratified_sampler(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut settings = StratifiedSamplerSettings::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "integer seed" => settings.seed = extract_value(tokenizer, "Sampler::seed - ")?,
            "integer xsamples" => settings.xsamples = extract_value(tokenizer, "Sampler::xsamples - ")?,
            "integer ysamples" => settings.ysamples = extract_value(tokenizer, "Sampler::ysamples - ")?,
            "bool jitter" => settings.jitter = extract_value(tokenizer, "Sampler::jitter - ")?,
            _ => return Err(format!("Unsupported parameter in stratified sampler: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.settings.spp = (settings.xsamples * settings.ysamples) as usize;
    scene.sampler = Some(Sampler::Stratified(settings));
    Ok(result)
}

fn process_halton_sampler(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut _seed: u64 = 0;
    let mut pixelsamples: usize = 1;
    let mut _randomization: String = "none".to_string();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "integer seed" => _seed = extract_value(tokenizer, "Sampler::seed - ")?,
            "integer pixelsamples" => pixelsamples = extract_value(tokenizer, "Sampler::pixelsamples - ")?,
            "string randomization" => _randomization = extract_value(tokenizer, "Sampler::randomization - ")?,
            _ => return Err(format!("Unsupported parameter in halton sampler: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;
    scene.settings.spp = pixelsamples;
    Ok(result)
}

fn process_filter(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                  state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let filter_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Missing filter type token!".to_string().into())
    };

    match filter_type {
        "box" => process_filter_data(tokenizer, scene, state, FilterType::Box),
        "gaussian" => process_filter_data(tokenizer, scene, state, FilterType::Gaussian),
        "mitchell" => process_filter_data(tokenizer, scene, state, FilterType::Mitchell),
        "sinc" => process_filter_data(tokenizer, scene, state, FilterType::LanczosSinc),
        "triangle" => process_filter_data(tokenizer, scene, state, FilterType::Triangle),
        _ => Err(format!("Sampler: Unsupported filter type - {}", filter_type).into())
    }
}

fn process_filter_data(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                       state: &mut ParseState, filter_type: FilterType) -> Result<Option<String>, Box<dyn Error>> {

    let mut desc = FilterDescriptor::default();
    desc.filter_type = filter_type;
    desc.set_default_radius();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "float xradius" => desc.xradius = extract_value(tokenizer, "Filter::xradius - ")?,
            "float yradius" => desc.yradius = extract_value(tokenizer, "Filter::yradius - ")?,
            "float sigma" => desc.alpha = extract_value(tokenizer, "Filter::sigma - ")?,
            "float tau" => desc.tau = extract_value(tokenizer, "Filter::tau - ")?,
            "float B" => desc.b = extract_value(tokenizer, "Filter::B - ")?,
            "float C" => desc.c = extract_value(tokenizer, "Filter::C - ")?,
            _ => return Err(format!("Unsupported parameter in filter: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.filter = Some(desc);
    Ok(result)
}

fn process_material(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let material_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Material: Type of material not specified!".into())
    };
    // TODO improve this - use unique name
    let name = format!("Material_generated_name_17654_{}", scene.materials.len());
    let result = match material_type {
        "diffuse" => process_diffuse_material(tokenizer, scene, state, &name),
        _=> Err(format!("Unsupported material type {}", material_type).into())
    };
    state.set_material(name);
    result
}

fn process_make_named_material(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let name = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Make Named Material: Name of material not specified!".into())
    };

    let string_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Make Named Material: Data type 'string type' of material not specified!".into())
    };

    if string_type != "string type" {
        return Err(format!("Make Named Material: 'string type' is expected! - {}", string_type).into());
    }

    let material_type: String = extract_value(tokenizer, "Make Named Material: Type of material - ")?;

    match material_type.as_str() {
        "diffuse" => process_diffuse_material(tokenizer, scene, state, name),
        _=> Err(format!("Make Named Material: Unsupported material type {}", material_type).into())
    }
}

fn process_named_material(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
                          state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let name = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Named Material: Name of material not specified!".into())
    };
    state.set_material(name.to_string());
    Ok(next_directive(tokenizer))
}

fn process_diffuse_material(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState, name: &str) -> Result<Option<String>, Box<dyn Error>> {

    let mut desc = MaterialDescription::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "rgb reflectance" => desc.diffuse = parse_rgb(tokenizer, "Material:rgb ")?,
            _ => return Err(format!("Unsupported parameter in diffuse material: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    desc.name = name.to_string();
    desc.typ = MaterialType::Matte;
    scene.materials.push(desc); 
    Ok(result)
}

fn process_light(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Light: Type of light not specified!".into())
    };
    match token {
        "point" => process_point_light(tokenizer, scene, state),
        _=> Err(format!("Unsupported light type {}", token).into())
    }
}

fn process_point_light(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut desc = LightDescription::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "rgb I" => desc.intensity = parse_rgb(tokenizer, "PointLight:rgb ")?,
            "point3 from" => desc.position = parse_point3(tokenizer, "PointLight:point from ")?,
            _ => return Err(format!("Unsupported parameter in point light: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    if !state.current_transformation().is_identity() {
        let t = Transformation::translate(&Vec3::from(desc.position)) * state.current_transformation();
        desc.position = Point3::new(0.0, 0.0, 0.0) * t;
    }

    desc.typ = LightType::Point;
    scene.lights.push(desc);
    Ok(result)
}

fn process_area_light_source(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                             state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("AreaLightSource: Type of light not specified!".into())
    };
    match token {
        "diffuse" => process_area_diffuse_light(tokenizer, scene, state),
        _=> Err(format!("Unsupported area light type {}", token).into())
    }
}

fn process_area_diffuse_light(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut desc = MaterialDescription::default();
    desc.diffuse = RGB::new(0.0, 0.0, 0.0);

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "rgb reflectance" => desc.diffuse = parse_rgb(tokenizer, "Material:rgb ")?,
            "rgb L" => desc.emission = parse_rgb(tokenizer, "Material:emission ")?,
            _ => return Err(format!("Unsupported parameter in emissive diffuse material: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    // TODO improve this - use unique name
    let name = format!("Material_generated_name_emmisive_17654_{}", scene.materials.len());

    desc.name = name.to_string();
    desc.typ = MaterialType::EmissiveMatte;
    scene.materials.push(desc);
    state.set_area_light(name.to_string());
    Ok(result)
}


fn process_shape(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Shape: Type of shape not specified!".into())
    };
    match token {
        "sphere" => process_sphere_shape(tokenizer, scene, state),
        "trianglemesh" => process_trianglemesh_shape(tokenizer, scene, state),
        _=> Err(format!("Unsupported shape type {}", token).into())
    }
}

fn process_sphere_shape(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut desc = SphereDescription::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "float radius" => desc.radius = extract_value(tokenizer, "Sphere:radius - ")?,
            "point3 position" => desc.position = parse_point3(tokenizer, "Sphere:position - ")?,
            _ => return Err(format!("Unsupported parameter in sphere shape: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    desc.material = match state.area_lights.last() {
        Some(name) => name.clone(),
        None => state.current_material().clone()
    };

    if !state.current_transformation().is_identity() {
        desc.transform = Some(state.current_transformation());
    }
    let shape = ShapeDescription::Sphere(desc);
    scene.shapes.push(shape);
    Ok(result)
}

fn process_trianglemesh_shape(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut desc = MeshDescription::default();

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "point2 uv" => desc.uvs = Some(parse_point2_array(tokenizer, "Mesh:uvs - ")?),
            "normal N" => desc.normals = Some(parse_normal_array(tokenizer, "Mesh:normals - ")?),
            "point3 P" => desc.vertices = Some(parse_point3_array(tokenizer, "Mesh:positions - ")?),
            "integer indices" => desc.indices = Some(parse_u32_array(tokenizer, "Mesh:indices - ")?),
            _ => return Err(format!("Unsupported parameter in sphere shape: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    desc.material = match state.area_lights.last() {
        Some(name) => name.clone(),
        None => state.current_material().clone()
    };

    if !state.current_transformation().is_identity() {
        desc.transform = Some(state.current_transformation());
    }

    // NOTE: special case for one triangle
    if desc.indices.is_none() {
        match &desc.vertices {
            Some(vertices) => {
                if vertices.len() == 3 {
                    desc.indices = Some(vec![0, 1, 2]);
                }
            }
            None => {}
        }
    }
    let shape = ShapeDescription::Mesh(desc);
    scene.shapes.push(shape);
    Ok(result)
}

fn process_world_begin(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    state.set_transformation(Transformation::identity());
    Ok(next_directive(tokenizer))
}

fn process_attribute_begin(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
                           state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    state.push_state();
    Ok(next_directive(tokenizer))
}

fn process_attribute_end(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
                           state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    state.pop_state();
    Ok(next_directive(tokenizer))
}

// TODO - test this function
fn create_path(state: &ParseState, filename: &str) -> String {
    if Path::new(filename).is_absolute() {
        return filename.to_string();
    }
    let full_path = match state.current_path.parent() {
        Some(dir) => dir.join(filename),
        None => PathBuf::new(),
    };
    return full_path.to_str().expect("Path conversion faild!").to_string();
}

fn parse_rgb(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<RGB,  Box<dyn Error>> {
    let (v0, v1, v2) = parse_f32x3(tokenizer, err_msg)?;
    Ok(RGB::new(v0, v1, v2))
}

fn parse_point3(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Point3,  Box<dyn Error>> {
    let (v0, v1, v2) = parse_f32x3(tokenizer, err_msg)?;
    Ok(Point3::new(v0, v1, v2))
}

fn parse_vec3(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Vec3,  Box<dyn Error>> {
    let (v0, v1, v2) = parse_f32x3(tokenizer, err_msg)?;
    Ok(Vec3::new(v0, v1, v2))
}

fn parse_f32x3(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<(f32, f32, f32),  Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    if token != "[" {
        return Err(format!("{} - Expected '[' token!", err_msg).into());
    }
    let v0 = parse_f32(tokenizer, err_msg)?;
    let v1 = parse_f32(tokenizer, err_msg)?;
    let v2 = parse_f32(tokenizer, err_msg)?;
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    if token != "]" {
        return Err(format!("{} - Expected ']' token!", err_msg).into());
    }
    Ok((v0, v1, v2))
}

fn parse_f32_array(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    if token != "[" {
        return Err(format!("{} - Expected '[' token!", err_msg).into());
    }
    let mut values = Vec::<f32>::new();
    loop {
        let token = match tokenizer.next() {
            Some(token) => token.trim(),
            None => return Err(format!("{} - Missing ']' token!", err_msg).into())
        };
        if token == "]" {
            break;
        }
        let val: f32 = match token.parse() {
            Err(e) => return Err(format!("{} - Parsing '{}':{}", err_msg, token, e).into()),
            Ok(val) => val
        };
        values.push(val);
    }
    Ok(values)
}


fn parse_point3_array(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Vec<Point3>, Box<dyn Error>> {
    let values = parse_f32_array(tokenizer, err_msg)?;
    let mut result = Vec::<Point3>::new();
    let chunks = values.chunks_exact(3);
    let rest = chunks.remainder();
    for chunk in chunks {
        result.push(Point3::new(chunk[0], chunk[1], chunk[2]));
    }
    if !rest.is_empty() {
        return Err(format!("{} - Expected 3 values per point!", err_msg).into());
    }
    Ok(result)
}

fn parse_point2_array(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Vec<Point2>, Box<dyn Error>> {
    let values = parse_f32_array(tokenizer, err_msg)?;
    let mut result = Vec::<Point2>::new();
    let chunks = values.chunks_exact(2);
    let rest = chunks.remainder();
    for chunk in chunks {
        result.push(Point2::new(chunk[0], chunk[1]));
    }
    if !rest.is_empty() {
        return Err(format!("{} - Expected 2 values per point!", err_msg).into());
    }
    Ok(result)
}

fn parse_normal_array(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Vec<Normal>, Box<dyn Error>> {
    let values = parse_f32_array(tokenizer, err_msg)?;
    let mut result = Vec::<Normal>::new();
    let chunks = values.chunks_exact(3);
    let rest = chunks.remainder();
    for chunk in chunks {
        result.push(Normal::new(chunk[0], chunk[1], chunk[2]));
    }
    if !rest.is_empty() {
        return Err(format!("{} - Expected 3 values per normal!", err_msg).into());
    }
    Ok(result)
}


fn parse_u32_array(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    if token != "[" {
        return Err(format!("{} - Expected '[' token!", err_msg).into());
    }
    let mut values = Vec::<u32>::new();
    loop {
        let token = match tokenizer.next() {
            Some(token) => token.trim(),
            None => return Err(format!("{} - Missing ']' token!", err_msg).into())
        };
        if token == "]" {
            break;
        }
        let val: u32 = match token.parse() {
            Err(e) => return Err(format!("{} - Parsing '{}':{}", err_msg, token, e).into()),
            Ok(val) => val
        };
        values.push(val);
    }
    Ok(values)
}

fn extract_value<T>(tokenizer: &mut PBRTTokenizer, err_msg: &str) -> Result<T,  Box<dyn Error>>
where T: FromStr, <T as FromStr>::Err: Display
{
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    if token != "[" {
        let val = match token.parse::<T>() {
            Err(e) => return Err(format!("{} - Parsing '{}':{}", err_msg, token, e).into()),
            Ok(val) => val
        };
        return Ok(val);
    }
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    let val = match token.parse::<T>() {
        Err(e) => return Err(format!("{} - Parsing '{}':{}", err_msg, token, e).into()),
        Ok(val) => val
    };
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    if token != "]" {
        return Err(format!("{} - Expected ']' token!", err_msg).into());
    }
    Ok(val)
}


fn parse_f32(tokenizer: &mut PBRTTokenizer, err_msg: &str) ->Result<f32,  Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("{} - Missing token!", err_msg).into())
    };
    let val: f32 = match token.parse() {
        Err(e) => return Err(format!("{} - Parsing '{}':{}", err_msg, token, e).into()),
        Ok(val) => val
    };
    Ok(val)   
}

