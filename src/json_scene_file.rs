use std::error::Error;
use std::fs;
use serde_json::Value;
use std::path::Path;

use crate::rgb::ImageSize;
use crate::color::{TMOType, RGB};
use crate::vec::{Point3, Vec3};
use crate::materials::{MaterialDescription, MaterialType};
use crate::shapes::{ShapeDescription, ShapeType};
use crate::lights::{LightDescription, LightType};
use crate::scene::{SceneDescription, RenderingAlgorithm};
use crate::transformations::Transformation;


pub fn load_scene_description_from_json<P: AsRef<Path>>(path: P) -> Result<SceneDescription, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let val: Value = serde_json::from_str(&contents)?;

    let mut scene_desc = SceneDescription::default();

    let global = &val["global"];
    if !global.is_null() {
        parse_global(&mut scene_desc, global)?;
    }
    let camera = &val["camera"];
    if !camera.is_null() {
        parse_camera(&mut scene_desc, camera)?;
    }
    let materials = &val["materials"];
    if !materials.is_null() {
        let mat_descs = parse_materials(materials)?;
        scene_desc.materials.extend(mat_descs)
    }
    let shapes = &val["shapes"];
    if !shapes.is_null() {
        let shape_descs = parse_shapes(shapes)?;
        scene_desc.shapes.extend(shape_descs);
    }
    let lights = &val["lights"];
    if !lights.is_null() {
        let light_descs = parse_lights(lights)?;
        scene_desc.lights.extend(light_descs);
    }

    Ok(scene_desc)
}


fn parse_global(scene_desc: &mut SceneDescription, section: &Value) -> Result<(), Box<dyn Error>> {

    if !section["resolution"].is_null() {
        let resolution = parse_resolution(&section["resolution"])?;
        scene_desc.set_resolution(resolution);
    }
    if !section["spp"].is_null() {
        let spp = parse_usize(&section["spp"], "spp")?;
        scene_desc.settings.spp = spp;
    }
    if !section["rendering"].is_null() {
        let alg = parse_string(&section["rendering"], "rendering")?;
        let algorithm = match alg.as_str() {
            "ambient" => RenderingAlgorithm::AmbientOcclusion,
            "direct_lighting" => RenderingAlgorithm::DirectLighting,
            "path" => RenderingAlgorithm::PathTracer,
            _ => return Err(format!("Unknown rendering algorithm: {}", alg).into())
        };
        scene_desc.settings.rendering_algorithm = algorithm;
    }
    if !section["tonemap"].is_null() {
        let tmo = parse_string(&section["tonemap"], "tonemap")?;
        let tmo_type = match tmo.as_str() {
            "linear" => TMOType::Linear,
            "gamma" => TMOType::Gamma,
            "reinhard" => TMOType::Reinhard,
            _ => return Err(format!("Unknown tone mapping operator: {}", tmo).into())
        };
        scene_desc.settings.tonemap = tmo_type;
    }
    if !section["output"].is_null() {
        let output = parse_string(&section["output"], "output")?;
        scene_desc.settings.output_fname = output;
    }
    if !section["nthreads"].is_null() {
        let nthreads = parse_usize(&section["nthreads"], "nthreads")?;
        scene_desc.settings.nthreads = nthreads;
    }

    Ok(())
}

fn parse_camera(scene_desc: &mut SceneDescription, section: &Value) -> Result<(), Box<dyn Error>> {
    if !section["eye"].is_null() {
        let eye = parse_point3(&section["eye"], "camera->eye")?;
        scene_desc.camera_desc.position = eye;
    }
    if !section["lookat"].is_null() {
        let look_at = parse_point3(&section["lookat"], "camera->lookat")?;
        scene_desc.camera_desc.look_at = look_at;
    }
    if !section["fov"].is_null() {
        let fov = parse_f32(&section["fov"], "camera->fov")?;
        scene_desc.camera_desc.fov = fov;
    }
    if !section["up"].is_null() {
        let up = parse_vec3(&section["up"], "camera->up")?;
        scene_desc.camera_desc.up = Some(up);
    }
    Ok(())
}

fn parse_materials(section: &Value) -> Result<Vec<MaterialDescription>, Box<dyn Error>> {
    let mtrs = match section.as_array() {
        Some(mtrs) => mtrs,
        None => return Err("List of materials expected.".into())
    };
    let mut materials = Vec::new();
    for mat in mtrs.iter() {
        let name = parse_string(&mat["name"], "material->name")?;
        let material_desc = parse_material(mat, &name)?;
        materials.push(material_desc);
    }
    Ok(materials)
}

fn parse_material(section: &Value, name: &str) -> Result<MaterialDescription, Box<dyn Error>> {
    let typ = parse_string(&section["type"], "material->type")?;
    let material_desc = match typ.as_str() {
        "matte" => parse_matte_material(section, name)?,
        // "matte_emissive" => parse_matte_emissive_material(scene_data, section, name)?,
        _ => return Err(format!("Unknown material type {}", typ).into())
    };
    Ok(material_desc)
}


fn parse_matte_material(section: &Value, name: &str) -> Result<MaterialDescription, Box<dyn Error>> {
    let diffuse = parse_rgb_color(&section["diffuse"], &format!("material:{}:diffuse", name))?;
    let mut desc = MaterialDescription::default();
    desc.name = name.to_string();
    desc.typ = MaterialType::Matte;
    desc.diffuse = diffuse;
    Ok(desc)
}

fn parse_lights(section: &Value) -> Result<Vec<LightDescription>, Box<dyn Error>> {
    let lights = match section.as_array() {
        Some(lights) => lights,
        None => return Err("List of lights expected!".into())
    };
    let mut light_descs = Vec::new();
    for light in lights.iter() {
        let light_desc = parse_light(light)?;
        light_descs.push(light_desc);
    }
    Ok(light_descs)
}

fn parse_light(section: &Value) -> Result<LightDescription, Box<dyn Error>> {
    let typ = parse_string(&section["type"], "light->type")?;
    let light_desc = match typ.as_str() {
        "point" => parse_point_light(section)?,
        _ => return Err(format!("Unknown light type {}", typ).into())
    };
    Ok(light_desc)
}

fn parse_point_light(section: &Value) -> Result<LightDescription, Box<dyn Error>> {
    let intensity = parse_rgb_color(&section["intensity"], "light->intensity")?;
    let position = parse_point3(&section["position"], "light->position")?;
    let mut desc = LightDescription::default();
    desc.typ = LightType::Point;
    desc.intensity = intensity;
    desc.position = position;
    Ok(desc)
}


fn parse_shapes(section: &Value) -> Result<Vec<ShapeDescription>, Box<dyn Error>> {
    let shapes = match section.as_array() {
        Some(shapes) => shapes,
        None => return Err("List of shapes expected!".into())
    };
    let mut shape_descs = Vec::new();
    for shape in shapes.iter() {
        let shape_desc = parse_shape(shape)?;
        shape_descs.push(shape_desc);
    }
    Ok(shape_descs)
}

fn parse_shape(section: &Value) -> Result<ShapeDescription, Box<dyn Error>> {
    let typ = parse_string(&section["type"], "shape->type")?;
    let shape_desc = match typ.as_str() {
        "sphere" => parse_sphere_shape(section)?,
        _ => return Err(format!("Unknown shape type {}", typ).into())
    };
    Ok(shape_desc)
}

fn parse_sphere_shape(section: &Value) -> Result<ShapeDescription, Box<dyn Error>> {
    let material = parse_string(&section["material"], "shape->material")?;
    let position = parse_point3(&section["position"], "shape->position")?;
    let radius = parse_f32(&section["radius"], "shape->radius")?;
    let mut desc = ShapeDescription::default();
    desc.typ = ShapeType::Sphere;
    desc.material = material;
    desc.position = position;
    desc.radius = radius;
    // if !section["transform"].is_null() {
    //     let transform = parse_transform(&section["transform"])?;
    // }
    Ok(desc)
}

// fn parse_transform(section: &Value) -> Result<Transformation, Box<dyn Error>> {
//     let t = Transformation::default();
    
//     let translation = parse_point3(&section["translation"], "transform->translation")?;
//     let rotation = parse_quat(&section["rotation"], "transform->rotation")?;
//     let scale = parse_vec3(&section["scale"], "transform->scale")?;
//     Ok(Transform::new(translation, rotation, scale))
// }


fn parse_rgb_color(section: &Value, field_name: &str) -> Result<RGB, Box<dyn Error>> {
    let r = parse_f32(&section[0], field_name)?;
    let g = parse_f32(&section[1], field_name)?;
    let b = parse_f32(&section[2], field_name)?;
    if !&section[3].is_null() {
        return Err(format!("Field: {} - Exactly 3 values expected!", field_name).into())
    }
    Ok(RGB{r, g, b})

}

fn parse_resolution(section: &Value) -> Result<ImageSize, Box<dyn Error>> {
    let width = parse_usize(&section[0], "resolution width")?;
    let height = parse_usize(&section[1], "resolution height")?;
    Ok(ImageSize::new(width, height))
}

fn parse_usize(section: &Value, field_name: &str) -> Result<usize, Box<dyn Error>> {
    let val = match section.as_u64() {
        Some(val) => val as usize,
        None => return Err(format!("Field: {}", field_name).into())
    };
    Ok(val)
}

fn parse_string(section: &Value, field_name: &str) -> Result<String, Box<dyn Error>> {
    let val = match section.as_str() {
        Some(val) => val,
        None => return Err(format!("Field: {}", field_name).into())
    };
    Ok(val.to_string())
}

fn parse_point3(section: &Value, field_name: &str) -> Result<Point3, Box<dyn Error>> {
    let val1 = parse_f32(&section[0], field_name)?;
    let val2 = parse_f32(&section[1], field_name)?;
    let val3 = parse_f32(&section[2], field_name)?;
    if !&section[3].is_null() {
        return Err(format!("Field: {} - Exactly 3 values expected!", field_name).into())
    }
    Ok(Point3::new(val1, val2, val3))
}

fn parse_vec3(section: &Value, field_name: &str) -> Result<Vec3, Box<dyn Error>> {
    let point = parse_point3(section, field_name)?;
    Ok(Vec3::from(point))
}

fn parse_f32(section: &Value, field_name: &str) -> Result<f32, Box<dyn Error>> {
    let val = match section.as_f64() {
        Some(val) => val as f32,
        None => return Err(format!("Field: {}", field_name).into())
    };
    Ok(val)
}
