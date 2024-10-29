use crate::vec::{Point3, Normal};
use crate::transformations::Transformation;
use crate::ray::Ray;

pub trait Intersect {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<f32>;
}

pub trait CalculateNormal {
    fn normal(&self, ray: &Ray, t: f32) -> Normal;
}


pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<f32> {
        crate::isect::isect_ray_sphere(ray, self.center, self.radius, tmin, tmax)
    }
}

impl CalculateNormal for Sphere {
    fn normal(&self, ray: &Ray, t: f32) -> Normal {
        Normal::from((ray.point_at(t) - self.center).normalize())
    }
}

pub struct TransformedShape<T> {
    object_to_world: Transformation,
    shape: T,
}

impl<T: Intersect> Intersect for TransformedShape<T> {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<f32> {
        // TODO: Transform ray
        self.shape.intersect(&ray, tmin, tmax)
    }
}

impl<T: CalculateNormal> CalculateNormal for TransformedShape<T> {
    fn normal(&self, ray: &Ray, t: f32) -> Normal {
        // TODO: Transform normal
        self.shape.normal(&ray, t)
    }
}

pub struct ShapeIntersection {
    t: f32,
    shape_id: usize,
    transformed: bool,
}

pub struct Primitives<T> {
    shapes: Vec<T>,
    transformed_shapes: Vec<TransformedShape<T>>,
    material_ids: Vec<u32>,
    transformed_material_ids: Vec<u32>,
}

impl<T: Intersect + CalculateNormal> Primitives<T> {
    pub fn new() -> Self {
        Self { shapes: Vec::new(), transformed_shapes: Vec::new(),
            material_ids: Vec::new(), transformed_material_ids: Vec::new() }
    }

    pub fn add(&mut self, shape: T, material_id: u32) {
        self.shapes.push(shape);
        self.material_ids.push(material_id);
    }

    pub fn add_transformed(&mut self, shape: T, object_to_world: Transformation, material_id: u32) {
        let transformed_shape = TransformedShape { shape, object_to_world };
        self.transformed_shapes.push(transformed_shape);
        self.transformed_material_ids.push(material_id);
    }

    pub fn normal(&self, ray: &Ray, isect: &ShapeIntersection) -> Normal {
        if isect.transformed {  
            self.transformed_shapes[isect.shape_id].normal(ray, isect.t)
        } else {
            self.shapes[isect.shape_id].normal(ray, isect.t)
        }
    }

    pub fn material(&self, isect: &ShapeIntersection) -> u32 {
        if isect.transformed {
            self.transformed_material_ids[isect.shape_id]
        } else {
            self.material_ids[isect.shape_id]
        }
    }

    pub fn intersect(&self, ray: &Ray, tmin: f32, mut tmax: f32) -> Option<ShapeIntersection> {
        let mut shape_id = 0;
        let mut transformed = false;
    
        for (idx, shape) in self.shapes.iter().enumerate() {
            let result = shape.intersect(ray, tmin, tmax);
            if let Some(t) = result {
                if t < tmax {
                    tmax = t;
                    shape_id = idx;
                    transformed = false;
                }
            }
        }

        for (idx, shape) in self.transformed_shapes.iter().enumerate() {
            let result = shape.intersect(ray, tmin, tmax);
            if let Some(t) = result {
                if t < tmax {
                    tmax = t;
                    shape_id = idx;
                    transformed = true;
                }
            }
        }
        if tmax < f32::INFINITY {
            Some(ShapeIntersection { t: tmax, shape_id, transformed })
        } else {
            None
        }
    }
}

pub struct Geometry {
    spheres: Primitives<Sphere>,
}

pub enum GeometryIntersection {
    Sphere(ShapeIntersection),
    None
}

pub struct SurfaceInteraction {
    pub t: f32,
    pub hit_point: Point3,
    pub normal: Normal,
    pub material_id: u32,
}

impl Geometry {
    pub fn new() -> Self {
        Self { spheres: Primitives::new() }
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material_id: u32) {
        self.spheres.add(sphere, material_id);
    }

    pub fn add_transformed_sphere(&mut self, sphere: Sphere, object_to_world: Transformation, material_id: u32) {
        self.spheres.add_transformed(sphere, object_to_world, material_id);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let res = self.spheres.intersect(ray, 0.0, f32::INFINITY);
        let isect = if let Some(shape_intersection) = res {
            GeometryIntersection::Sphere(shape_intersection)
        } else {
            GeometryIntersection::None
        };
        self.surface_interaction(&ray, &isect)
    }

    pub fn surface_interaction(&self, ray: &Ray, isect: &GeometryIntersection) -> Option<SurfaceInteraction> {
        match isect {
            GeometryIntersection::Sphere(shape_intersection) => {
                let hit_point = ray.point_at(shape_intersection.t);
                let normal = self.spheres.normal(&ray, &shape_intersection);
                let material_id = self.spheres.material(&shape_intersection);
                Some(SurfaceInteraction { t: shape_intersection.t, hit_point, normal, material_id })
            }
            GeometryIntersection::None => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec::Point3;

    #[test]
    fn test_sphere_creation() {
        let center = Point3::new(1.0, 2.0, 3.0);
        let radius = 5.0;
        let sphere = Sphere::new(center, radius);

        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn test_shapes_add() {
        let mut primitives = Primitives::<Sphere>::new();
        let sphere1 = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let sphere2 = Sphere::new(Point3::new(1.0, 1.0, 1.0), 2.0);

        primitives.add(sphere1, 0);
        primitives.add(sphere2, 0);

        assert_eq!(primitives.shapes.len(), 2);
        assert_eq!(primitives.shapes[0].center, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(primitives.shapes[0].radius, 1.0);
        assert_eq!(primitives.shapes[1].center, Point3::new(1.0, 1.0, 1.0));
        assert_eq!(primitives.shapes[1].radius, 2.0);
    }
}
