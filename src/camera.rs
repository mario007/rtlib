use crate::vec::{Vec3, Point3};
use crate::transformations::Transformation;
use crate::ray::Ray;
use crate::rgb::ImageSize;

pub fn create_raster_to_ndc_transformation(resolution_x: usize, resolution_y: usize) -> Transformation {
    let ndc_to_raster = Transformation::scale(resolution_x as f32, -(resolution_y as f32), 1.0);
    return ndc_to_raster.inverse();
}

pub fn create_ndc_to_screen_transformation(resolution_x: usize, resolution_y: usize) -> Transformation {
    let aspect_ratio = resolution_x as f32 / resolution_y as f32;
    // screen, shorter axis [-1, 1]
    let (pmin_x, pmin_y, pmax_x, pmax_y) = if aspect_ratio > 1.0 {
        (-aspect_ratio, -1.0, aspect_ratio, 1.0)
    } else {
        (-1.0, -1.0 / aspect_ratio, 1.0, 1.0 / aspect_ratio)
    };

    let scale_x = (pmax_x - pmin_x).recip();
    let scale_y = (pmax_y - pmin_y).recip();
    let screen_to_ndc =
    Transformation::scale(scale_x, scale_y, 1.0) * 
    Transformation::translate(&Vec3::new(-pmin_x, -pmax_y, 0.0));
    return screen_to_ndc.inverse();
}

pub fn create_screen_to_perspective_transformation(fov: f32, z_near: f32, z_far: f32) -> Transformation {
    Transformation::perspective(fov, z_near, z_far).inverse()
}

pub fn create_raster_to_perspective_transformation(
    resolution_x: usize, resolution_y: usize, fov: f32, z_near: f32, z_far: f32) -> Transformation {
    let raster_to_ndc = create_raster_to_ndc_transformation(resolution_x, resolution_y);
    let ndc_to_screen = create_ndc_to_screen_transformation(resolution_x, resolution_y);
    let screen_to_camera = create_screen_to_perspective_transformation(fov, z_near, z_far);
    screen_to_camera * ndc_to_screen * raster_to_ndc
}

pub struct PerspectiveCamera {
    raster_to_camera: Transformation,
    camera_to_world: Transformation,
}

impl PerspectiveCamera {
    fn new(size: ImageSize, fov: f32, near_plane: f32, far_plane: f32, camera_to_world: Transformation) -> PerspectiveCamera {
        let raster_to_camera = create_raster_to_perspective_transformation(size.width, size.height, fov, near_plane, far_plane);
        PerspectiveCamera { raster_to_camera, camera_to_world }
    }

    pub fn generate_ray(&self, x: f32, y: f32) -> Ray {
        let local_origin = Point3::new(0.0, 0.0, 0.0);
        let point_on_camera = Point3::new(x, y, 0.0) * self.raster_to_camera;
        let local_direction = Vec3::from(point_on_camera);
        Ray::new(local_origin, local_direction) * self.camera_to_world
    }
}

pub struct PerspectiveCameraDescriptor {
    pub resolution: ImageSize,
    pub fov: f32,
    pub position: Point3,
    pub look_at: Point3,
    pub up: Option<Vec3>,
    pub near_plane: Option<f32>,
    pub far_plane: Option<f32>,
    pub camera_to_world: Option<Transformation>,
}

impl PerspectiveCameraDescriptor {
    pub fn create(&self) -> PerspectiveCamera {
        let near_plane = self.near_plane.unwrap_or(0.01);
        let far_plane = self.far_plane.unwrap_or(1000.0);
        let up = self.up.unwrap_or(Vec3::new(0.0, 1.0, 0.0));
        let camera_to_world = self.camera_to_world.unwrap_or(Transformation::look_at(self.position, self.look_at, up));
        PerspectiveCamera::new(self.resolution, self.fov, near_plane, far_plane, camera_to_world)
    }
}

impl Default for PerspectiveCameraDescriptor {
    fn default() -> Self {
        Self { 
            resolution: ImageSize::new(256, 256),
            fov: 45.0,
            position: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            up: None,
            near_plane: None,
            far_plane: None,
            camera_to_world: None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec::Point3;

    #[test]
    fn test_create_matrix() {
        let resolution_x = 800;
        let resolution_y = 600;

        let matrix = create_raster_to_ndc_transformation(resolution_x, resolution_y);

        let point = Point3::new(400.0, 100.0, 0.0);
        let result = point * matrix;
        println!("result: {:?}", result);

        // Assert that the matrix is correctly created
        //assert_eq!(matrix, Transformation::scale(800.0, -600.0, 1.0));
    }
}
