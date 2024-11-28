use crate::rng::{PCGRng, Rng};
use crate::vec::{Vec3, Normal};
use crate::color::{RGB, RGBPixelSample, AccumlationBuffer};
use crate::shapes::Geometry;
use crate::frame::Frame;
use crate::scene::Scene;
use crate::rgb::RGB8uffer;
use crate::vec::Point3;
use crate::tile::Tile;
use crate::ray::{Ray, spawn_new_ray};
use crate::scene::RenderingAlgorithm;
use crate::scene::AmbientOcclusionSettings;
use crate::samplings::{sample_cos_hemisphere, sample_uniform_hemisphere};


pub fn ambient_occlusion_integrator(scene: &Scene, ao_settings: &AmbientOcclusionSettings) -> RGB8uffer {
    let spp = scene.settings.spp;
    let resolution = scene.settings.resolution;
    let camera = &scene.camera;
    let geometry = &scene.geometry;
    let mut rng = PCGRng::new(15236, 31337);
    let tile = Tile::new(0, 0, resolution.width, resolution.height);
    let mut accum = AccumlationBuffer::<RGBPixelSample>::new(tile);
    let cossample = ao_settings.cossample;
    let maxdistance = ao_settings.maxdistance;

    for _ in 0..spp {
        for (x, y) in tile {
            let px = x as f32 + rng.rand_f32();
            let py = y as f32 + rng.rand_f32();
            let ray = camera.generate_ray(px, py);
            let rgb = ambient_occlusion(&ray, &geometry, &mut rng, cossample, maxdistance);
            let sample = RGBPixelSample::new(rgb, 1.0);
            
            accum.add(x, y, &sample);
        } 
    }
    accum.to_rgb8_buffer(&scene.settings.tonemap)
}

pub fn ambient_occlusion(ray: &Ray, shapes: &Geometry, rng: &mut PCGRng,
                         cossample: bool, maxdistance: f32) -> RGB {
    
    let result = shapes.intersect(&ray);
    let si = match result {
        Some(si) => si,
        None => return RGB::new(1.0, 1.0, 1.0)
    };

    let sample_dir = if cossample {
        sample_cos_hemisphere(rng.rand_f32(), rng.rand_f32())
    } else {
        sample_uniform_hemisphere(rng.rand_f32(), rng.rand_f32())
    };
    if sample_dir.pdfw == 0.0 {
        return RGB::zero();
    }

    let new_direction = Frame::from(si.normal).to_world(sample_dir.direction).normalize();

    let shadow_ray = spawn_new_ray(si.hit_point, si.normal, new_direction);
    let shadow_result = shapes.intersect(&shadow_ray);

    #[inline(always)]
    fn calc_result(direction: Vec3, normal: Normal, pdfw: f32) -> RGB {
        // Divide by pi so that fully visible is one.
        let cosa = (direction * normal).abs();
        let denom = pdfw * std::f32::consts::PI;
        RGB::new(1.0, 1.0, 1.0) * (cosa * denom.recip())
    }

    match shadow_result {
        Some(res) => {
            if res.t < maxdistance {
                return RGB::zero();
            }
            calc_result(new_direction, si.normal, sample_dir.pdfw)
        },
        None => calc_result(new_direction, si.normal, sample_dir.pdfw)
    }
}


fn visible(p1: Point3, normal: Normal, p2: Point3, shapes: &Geometry) -> bool {
    let new_direction = (p2 - p1).normalize();
    let shadow_ray = crate::ray::spawn_new_ray(p1, normal, new_direction);
    let result = shapes.intersect(&shadow_ray);
    let distance = shadow_ray.origin.distance(p2);
    match result {
        Some(si) => si.t > distance,
        None => true
    }
}

pub fn pdfw_to_a(pdfw: f32, dist: f32, cos_there: f32) -> f32 {
    pdfw * cos_there.abs() / (dist * dist)
}

pub fn pdfa_to_w(pdfa: f32, dist: f32, cos_there: f32) -> f32 {
    pdfa * (dist * dist) / cos_there.abs()
}

pub fn direct_lgt_integrator(scene: &Scene) -> RGB8uffer {
    let spp = scene.settings.spp;
    let resolution = scene.settings.resolution;
    let camera = &scene.camera;
    let mut rng = PCGRng::new(15236, 31337);
    let tile = Tile::new(0, 0, resolution.width, resolution.height);
    let mut accum = AccumlationBuffer::<RGBPixelSample>::new(tile);

    for _ in 0..spp {
        for (x, y) in tile {
            let px = x as f32 + rng.rand_f32();
            let py = y as f32 + rng.rand_f32();
            let ray = camera.generate_ray(px, py);
            let rgb = radiance_direct_lgt(&ray, scene, &mut rng);
            let sample = RGBPixelSample::new(rgb, 1.0);
            
            accum.add(x, y, &sample);
        } 
    }
    accum.to_rgb8_buffer(&scene.settings.tonemap)
}

pub fn radiance_direct_lgt (ray: &Ray, scene: &Scene, rng: &mut PCGRng) -> RGB {
    let isect_p = match scene.geometry.intersect(ray) {
        Some(isect_p) => isect_p,
        None => return RGB::zero()
    };

    let wo = -ray.direction;
    let mut acum = RGB::zero();

    for light in scene.lights.iter() {
        let ls = light.illuminate(isect_p.hit_point, rng);
        let ls = match ls {
            Some(ls) => ls,
            None => continue
        };
        if visible(isect_p.hit_point, isect_p.normal, ls.position, &scene.geometry) {
            let material = &scene.materials[isect_p.material_id as usize];
            let result = material.eval(wo, isect_p.normal, ls.wi);
            let (mat_spectrum, _pdfw) = match result {
                Some(result) => (result.color, result.pdfw),
                None => continue
            };
            let cosa = (ls.wi * isect_p.normal).abs();
            let dist = isect_p.hit_point.distance(ls.position);
            let pdf = pdfa_to_w(ls.pdfa, dist, ls.cos_theta);
            if ls.wi * isect_p.normal > 0.0 && wo * isect_p.normal > 0.0 {
                acum = acum + (mat_spectrum * ls.intensity) * (cosa / pdf);
            }
        }
    }
    acum
}

fn render_scene(scene: &Scene) -> RGB8uffer {
    match scene.settings.rendering_algorithm {
        RenderingAlgorithm::AmbientOcclusion(ao_settings) => {
            ambient_occlusion_integrator(scene, &ao_settings)
        }
        RenderingAlgorithm::DirectLighting => {
            direct_lgt_integrator(scene)
        }
        _ => {
            panic!("Unsupported algorithm");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use crate::pbrt_v4::parse_pbrt_v4_input_file;
    use crate::json::load_scene_description_from_json;

    #[test]
    fn test_render_scene() {
        // let path = "D://rtlib_scenes//sphere//sphere.json";
        // let path = "D://rtlib_scenes//spheres//spheres.json";
        // let path = "D://rtlib_scenes//spheres_trans//spheres.json";
        // let scene_descripton = load_scene_description_from_json(path);

        // let path = "D://rtlib_scenes//sphere//sphere.pbrt";
        let path = "D://rtlib_scenes//spheres//spheres.pbrt";
        // let path = "D://rtlib_scenes//spheres_trans//spheres.pbrt";
        let scene_descripton = parse_pbrt_v4_input_file(path);

        let scene_description = match scene_descripton {    
            Ok(scene_descripton) => {
                scene_descripton
            }
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        };
        let scene = Scene::from(scene_description);
        let total_time = Instant::now();
        let image = render_scene(&scene);
        let total_duration = total_time.elapsed();
        println!("Rendering time: {:?}", total_duration);
        let _res = image.save(scene.settings.output_fname);
    }
}
