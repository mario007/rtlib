use crate::color::RGB;
use crate::vec::{Point3, Vec3};
use std::path::PathBuf;
use std::collections::HashMap;
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
use crate::shapes::ShapeType;

struct AreaLightInfo {
    radiance: RGB,
    type_name: String,
}

struct ParseState {
    transformations: Vec<Transformation>,
    general_section: bool,
    materials: Vec<String>,
    area_lights_infos: Vec<AreaLightInfo>,
    named_materials: Vec<HashMap<String, u32>>,
    current_path: PathBuf,
    directives: HashSet<&'static str>,
}

impl ParseState {
    pub fn new() -> Self {
        let transformations = vec![Transformation::default()];
        let materials = Vec::new();
        let info = AreaLightInfo{type_name: "".to_string(), radiance: RGB::zero()};
        let area_lights_infos = vec![info];
        let named_materials = vec![HashMap::new()];
        let current_path = PathBuf::new();
        let directives: HashSet<_> = vec!["LookAt", "Camera", "Sampler", "Integrator", "Film", "PixelFilter",
        "WorldBegin", "AttributeBegin", "AttributeEnd", "LightSource", "AreaLightSource", "Texture",
        "Material", "MakeNamedMaterial", "NamedMaterial", "Include", "Accelerator", "Shape",
        "Scale", "Translate", "Rotate", "Identity", "Transform", "ConcatTransform"].into_iter().collect();
        Self {
            transformations,
            general_section: true,
            materials,
            area_lights_infos,
            named_materials,
            current_path,
            directives
        }
    }

    pub fn push_state(&mut self) {
        self.transformations.push(self.current_transformation());
        self.materials.push(self.current_material());
        let info = AreaLightInfo{type_name: "".to_string(), radiance: RGB::zero()};
        self.area_lights_infos.push(info);

        // TODO - copy on write approach for performances reasons
        let index = self.named_materials.len() - 1;
        let map = self.named_materials[index].clone();
        self.named_materials.push(map);
    }

    pub fn current_transformation(&self) -> Transformation {
        self.transformations[self.transformations.len() - 1]
    }

    pub fn current_material(&self) -> String {
        //self.materials_ids[self.materials_ids.len() - 1]
        self.materials.last().expect("No material exist!").clone()
    }

    pub fn cur_area_light_type(&self) -> &str {
        &self.area_lights_infos[self.area_lights_infos.len() - 1].type_name
    }

    pub fn cur_area_light_radiance(&self) -> RGB {
        self.area_lights_infos[self.area_lights_infos.len() - 1].radiance
    }

    pub fn set_transformation(&mut self, transformation: Transformation) {
        //let index = self.matrices.len() - 1;
        //self.matrices[index] = matrix;
        if let Some(last) = self.transformations.last_mut() {
            *last = transformation;
        }
    }

    pub fn set_material(&mut self, material: String) {
        if self.materials.len() == 0 {
            self.materials.push(material);
        } else {
            let index = self.materials.len() - 1;
            self.materials[index] = material;
        }
    }

    pub fn set_area_light_info(&mut self, type_name: String, radiance: RGB) {
        let index = self.area_lights_infos.len() - 1;
        self.area_lights_infos[index].type_name = type_name;
        self.area_lights_infos[index].radiance = radiance;
    }

    pub fn pop_state(&mut self) {
        self.transformations.pop();
        self.materials.pop();
        self.area_lights_infos.pop();
        self.named_materials.pop();
    }

    pub fn set_in_general_section(&mut self, value: bool) {
        self.general_section = value;
    }

    pub fn add_named_material(&mut self, name: String, material_id: u32) {
        let index = self.named_materials.len() - 1;
        let map = &mut self.named_materials[index];
        // Todo - logger - if material allready exist it will be redefined
        map.insert(name, material_id);
    }

    pub fn get_named_material(&self, name: &str) -> u32 {
        let index = self.named_materials.len() - 1;
        let map = &self.named_materials[index];
        *map.get(name).unwrap_or_else(|| panic!("Material {} doesn't exist!", name))
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
            // "PixelFilter" => process_pixel_filter(tokens, scene, state)?,
            "WorldBegin" => process_world_begin(&mut ct, scene, state)?,
            // "AttributeBegin" => process_attribute_begin(tokens, scene, state)?,
            // "AttributeEnd" => process_attribute_end(tokens, scene, state)?,
            "LightSource" => process_light(&mut ct, scene, state)?,
            // "AreaLightSource" => process_area_light_source(tokens, scene, state)?,
            // "Texture" => process_texture(tokens, scene, state)?,
            "Material" => process_material(&mut ct, scene, state)?,
            "Shape" => process_shape(&mut ct, scene, state)?,
            // "MakeNamedMaterial" => process_make_named_material(tokens, scene, state)?,
            // "NamedMaterial" => process_named_material(tokens, scene, state)?,
            // "Accelerator" => process_accelerator(tokens, scene, state)?,
            // "Scale" => process_scale_transform(tokens, scene, state)?,
            // "Translate" => process_translate_transform(tokens, scene, state)?,
            // "Rotate" => process_rotate_transform(tokens, scene, state)?,
            // "Identity" => process_identity_transform(tokens, scene, state)?,
            // "Transform" => process_transform(tokens, scene, state)?,
            // "ConcatTransform" => process_concat_transform(tokens, scene, state)?,
            _=> return Err(format!("Unsupported directive to process: {}", cur_directive).into())
        };
        match new_directive {
            Some(directive) => cur_directive = directive.clone(),
            None => return Ok(())
        }
    }
}

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

fn process_camera(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                  state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let camera_type = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err(format!("Missing camera type token!").into())
    };

    match camera_type {
        "perspective" => process_perspective_camera(tokenizer, scene, state),
        _ => return Err(format!("Camera: Unsupported camera type - {}", camera_type).into())
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
    scene.camera_desc.camera_to_world = Some(state.current_transformation());
    Ok(result)
}

fn process_integrator(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Integrator: Type of integrator not specified!".into())
    };
    match token {
        "direct_lighting" => process_integrator_direct_lighting(tokenizer, scene, state),
        // "intersector" => process_integrator_isect(tokens, scene, state)?,
        // "path" => process_integrator_path(tokens, scene, state)?,
        _=> return Err(format!("Unsupported integrator type {}", token).into())
    }
}

fn process_integrator_direct_lighting(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                                      _state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    scene.settings.rendering_algorithm = RenderingAlgorithm::DirectLighting;                                   
    Ok(next_directive(tokenizer))
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
        None => return Err(format!("Missing film type token!").into())
    };

    match film_type {
        "rgb" => process_rgb_film(tokenizer, scene, state),
        _ => return Err(format!("Camera: Unsupported film type - {}", film_type).into())
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
        None => return Err(format!("Missing sampler type token!").into())
    };

    match sampler_type {
        "independent" => process_independent_sampler(tokenizer, scene, state),
        "halton" => process_halton_sampler(tokenizer, scene, state),
        "paddedsobol" => process_independent_sampler(tokenizer, scene, state),
        "sobol" => process_independent_sampler(tokenizer, scene, state),
        "stratified" => process_independent_sampler(tokenizer, scene, state),
        "zsobol" => process_independent_sampler(tokenizer, scene, state),
        _ => return Err(format!("Sampler: Unsupported sampler type - {}", sampler_type).into())
    }
}

fn process_independent_sampler(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut _seed: u64 = 0;
    let mut pixelsamples: usize = 1;

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "integer seed" => _seed = extract_value(tokenizer, "Sampler::seed - ")?,
            "integer pixelsamples" => pixelsamples = extract_value(tokenizer, "Sampler::pixelsamples - ")?,
            _ => return Err(format!("Unsupported parameter in independent sampler: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    scene.settings.spp = pixelsamples;
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

fn process_material(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                      state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    let token = match tokenizer.next() {
        Some(token) => token.trim(),
        None => return Err("Material: Type of material not specified!".into())
    };
    match token {
        "diffuse" => process_diffuse_material(tokenizer, scene, state),
        _=> return Err(format!("Unsupported material type {}", token).into())
    }
}

fn process_diffuse_material(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                              state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut diffuse: RGB = RGB::new(0.5, 0.5, 0.5);

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "rgb reflectance" => diffuse = parse_rgb(tokenizer, "Material:rgb ")?,
            _ => return Err(format!("Unsupported parameter in diffuse material: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    let mut desc = MaterialDescription::default();
    // TODO improve this -- use unique name - MakeNamedMaterial?!
    let name = "Material_General_".to_string() + &scene.materials.len().to_string();
    desc.name = name.clone();
    desc.typ = MaterialType::Matte;
    desc.diffuse = diffuse;
    scene.materials.push(desc);
    state.set_material(name);
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
        _=> return Err(format!("Unsupported light type {}", token).into())
    }
}

fn process_point_light(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut intensity: RGB = RGB::new(1.0, 1.0, 1.0);
    let mut position: Point3 = Point3::new(0.0, 0.0, 0.0);

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "rgb I" => intensity = parse_rgb(tokenizer, "PointLight:rgb ")?,
            "point3 from" => position = parse_point3(tokenizer, "PointLight:point from ")?,
            _ => return Err(format!("Unsupported parameter in point light: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    let mut desc = LightDescription::default();
    desc.typ = LightType::Point;
    desc.intensity = intensity;
    desc.position = position;
    scene.lights.push(desc);
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
        _=> return Err(format!("Unsupported shape type {}", token).into())
    }
}

fn process_sphere_shape(tokenizer: &mut PBRTTokenizer, scene: &mut SceneDescription,
                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {

    let mut radius: f32 = 1.0;
    let mut position: Point3 = Point3::new(0.0, 0.0, 0.0);

    let mut process_attribute = |tokenizer: &mut PBRTTokenizer, token: &str| -> Result<(), Box<dyn Error>> {
        match token {
            "float radius" => radius = extract_value(tokenizer, "Sphere:radius - ")?,
            "point3 position" => position = parse_point3(tokenizer, "Sphere:position - ")?,
            _ => return Err(format!("Unsupported parameter in sphere shape: {}", token).into())
        }
        Ok(())
    };
    let result = process_attributes(tokenizer, state, &mut process_attribute)?;

    let mut desc = ShapeDescription::default();
    desc.typ = ShapeType::Sphere;
    desc.radius = radius;
    desc.position = position;
    desc.material = state.current_material().clone();
    scene.shapes.push(desc);
    Ok(result)
}

fn process_world_begin(tokenizer: &mut PBRTTokenizer, _scene: &mut SceneDescription,
                       state: &mut ParseState) -> Result<Option<String>, Box<dyn Error>> {
    state.set_in_general_section(false);
    state.set_transformation(Transformation::default());
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
    return Ok(val);
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

