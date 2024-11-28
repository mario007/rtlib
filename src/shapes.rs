use crate::vec::{Point3, Normal, Vec3};
use crate::transformations::Transformation;
use crate::ray::Ray;
use std::ops::Mul;
use std::collections::HashMap;

pub trait Intersect {
    fn intersect(&self, ray: &Ray, tmin: f32) -> Option<f32>;
}

pub trait CalculateNormal {
    fn normal(&self, ray: &Ray, hit_point: Point3) -> Normal;
}

pub trait BoundingBox {
    fn bounding_box(&self) -> AABB;
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn intersect(&self, ray_origin: Point3, ray_inv_direction: Vec3) -> bool {
        crate::isect::isect_ray_bbox(ray_origin, ray_inv_direction, self.min, self.max)
    }
}

impl Mul<Transformation> for AABB {
    type Output = Self;
    fn mul(self, rhs: Transformation) -> Self::Output {
        let delta = self.max - self.min;
        let p1 = rhs * self.min;
        let p2 = rhs * self.max;
        let p3 = rhs * (self.min + Vec3::new(delta.x, 0.0, 0.0));
        let p4 = rhs * (self.min + Vec3::new(0.0, delta.y, 0.0));
        let p5 = rhs * (self.min + Vec3::new(delta.x, delta.y, 0.0));
        let p6 = rhs * (self.max + Vec3::new(delta.x, 0.0, 0.0));
        let p7 = rhs * (self.max + Vec3::new(0.0, delta.y, 0.0));
        let p8 = rhs * (self.max + Vec3::new(delta.x, delta.y, 0.0));
        let min_p = p1.min(p2).min(p3).min(p4).min(p5).min(p6).min(p7).min(p8);
        let max_p = p1.max(p2).max(p3).max(p4).max(p5).max(p6).max(p7).max(p8);
        AABB::new(min_p, max_p)
    }
}

pub struct LinearIntersector {
    bboxes: Vec<AABB>,
}

impl LinearIntersector {
    pub fn new() -> Self {
        Self { bboxes: Vec::new() }
    }

    pub fn prepare_for_rendering(&mut self, n_primitives: usize,
        calculate_bbox_fn: &dyn Fn(usize) -> AABB) {
        self.bboxes.clear();
        for i in 0..n_primitives {
            self.bboxes.push(calculate_bbox_fn(i));
        }
    }

    pub fn intersect(&self, ray: &Ray,
    isect_fn: &dyn Fn(usize, &Ray) -> Option<f32>) -> Option<ShapeIntersection> {
        let mut primitive_id = 0;
        const BIG_NUMBER: f32 = 1e38;
        let mut current_t = BIG_NUMBER;
        let rd = ray.direction;
        let inv_rd = Vec3::new(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);
    
        for (idx, bbox) in self.bboxes.iter().enumerate() {
            // Note: ray-bbox to return t and used that information to improve performance
            if bbox.intersect(ray.origin, inv_rd) {
                let result = isect_fn(idx, ray);
                if let Some(t) = result {
                    if t < current_t {
                        current_t = t;
                        primitive_id = idx;
                    }
                }
            }
        }
        if current_t < BIG_NUMBER {
            Some(ShapeIntersection { t: current_t, shape_id: primitive_id})
        } else {
            None
        }
    }
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
    fn intersect(&self, ray: &Ray, tmin: f32) -> Option<f32> {
        crate::isect::isect_ray_sphere(ray, self.center, self.radius, tmin, 1e38)
    }
}

impl CalculateNormal for Sphere {
    fn normal(&self, _ray: &Ray, hit_point: Point3) -> Normal {
        Normal::from((hit_point - self.center).normalize())
    }
}

impl BoundingBox for Sphere {
    fn bounding_box(&self) -> AABB {
        let min = self.center + Vec3::new(-self.radius, -self.radius, -self.radius);
        let max = self.center + Vec3::new(self.radius, self.radius, self.radius);
        AABB::new(min, max)
    }
}

pub struct TransformedShape<T> {
    shape: T,
    obj_to_world: Option<Transformation>,
}

impl<T> TransformedShape<T> {
    pub fn new(shape: T, obj_to_world: Option<Transformation>) -> Self {
        Self { shape, obj_to_world }
    }
}

impl<T: Intersect> Intersect for TransformedShape<T> {
    fn intersect(&self, ray: &Ray, tmin: f32) -> Option<f32> {
        match self.obj_to_world {
            Some(transformation) => {   
                let local_ray = *ray * transformation.inverse();
                let result = self.shape.intersect(&local_ray, tmin);
                if let Some(t) = result {
                    let local_point = local_ray.point_at(t);
                    let world_point = transformation * local_point;
                    Some(world_point.distance(ray.origin))
                } else {
                    None
                }
            }
            None => self.shape.intersect(ray, tmin)
        }
    }
}

impl<T: BoundingBox> BoundingBox for TransformedShape<T> {
    fn bounding_box(&self) -> AABB {
        let bounding_box = self.shape.bounding_box();
        match self.obj_to_world {
            Some(transformation) => bounding_box * transformation,
            None => bounding_box
        }
    }
}

impl<T: CalculateNormal> CalculateNormal for TransformedShape<T> {
    fn normal(&self, ray: &Ray, hit_point: Point3) -> Normal {
        match self.obj_to_world {
            Some(transformation) => {
                let world_to_object = transformation.inverse();
                let local_ray = *ray * world_to_object;
                let local_point = hit_point * world_to_object;
                let local_normal = self.shape.normal(&local_ray, local_point);
                (transformation * local_normal).normalize()
            }
            None => self.shape.normal(ray, hit_point)
        }
    }
}


pub struct ShapeIntersection {
    t: f32,
    shape_id: usize,
}

pub struct Primitives<T> {
    shapes: Vec<TransformedShape<T>>,
    material_ids: Vec<u32>,
    linear_intersector: LinearIntersector,
}

impl<T: Intersect + CalculateNormal + BoundingBox> Primitives<T> {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            material_ids: Vec::new(),
            linear_intersector: LinearIntersector::new(),
        }
    }

    pub fn prepare_for_rendering(&mut self) {
        let calculate_bbox_fn = |idx: usize| self.shapes[idx].bounding_box();
        self.linear_intersector.prepare_for_rendering(self.shapes.len(), &calculate_bbox_fn);
    }

    pub fn add(&mut self, shape: T, object_to_world: Option<Transformation>, material_id: u32) {
        self.shapes.push(TransformedShape::new(shape, object_to_world));
        self.material_ids.push(material_id);
    }

    pub fn normal(&self, ray: &Ray, isect: &ShapeIntersection) -> Normal {
        self.shapes[isect.shape_id].normal(ray, ray.point_at(isect.t))
    }

    pub fn material(&self, isect: &ShapeIntersection) -> u32 {
        self.material_ids[isect.shape_id]
    }

    pub fn intersect(&self, ray: &Ray) -> Option<ShapeIntersection> {
        let isect_fn = |idx: usize, ray: &Ray| self.shapes[idx].intersect(ray, 0.0);
        self.linear_intersector.intersect(ray, &isect_fn)
    }
}

pub struct Mesh {
    vertices: Vec<Point3>,
    indices: Vec<u32>,
}

impl From<(Vec<Point3>, Vec<u32>)> for Mesh {
    fn from(descriptor: (Vec<Point3>, Vec<u32>)) -> Self {
        if descriptor.1.len() % 3 != 0 {
            panic!("Invalid mesh descriptor: indices length must be a multiple of 3");
        }
        Self {
            vertices: descriptor.0,
            indices: descriptor.1,
        }
    }
}

impl Mesh {
    pub fn bounding_box(&self, triangle_id: usize) -> AABB {
        let vertices = triangle_id * 3;
        let v0 = self.vertices[self.indices[vertices] as usize];
        let v1 = self.vertices[self.indices[vertices + 1] as usize];
        let v2 = self.vertices[self.indices[vertices + 2] as usize];
        let min_p = v0.min(v1).min(v2);
        let max_p = v0.max(v1).max(v2);
        AABB::new(min_p, max_p)
    }

    pub fn normal(&self, triangle_id: usize) -> Normal {
        let vertices = triangle_id * 3;
        let v0 = self.vertices[self.indices[vertices] as usize];
        let v1 = self.vertices[self.indices[vertices + 1] as usize];
        let v2 = self.vertices[self.indices[vertices + 2] as usize];
        Normal::from((v1 - v0).cross(v2 - v0).normalize())
    }

    pub fn intersect(&self, triangle_id: usize, ray: &Ray, tmin: f32) -> Option<f32> {
        let vertices = triangle_id * 3;
        let v0 = self.vertices[self.indices[vertices] as usize];
        let v1 = self.vertices[self.indices[vertices + 1] as usize];
        let v2 = self.vertices[self.indices[vertices + 2] as usize];
        crate::isect::isect_ray_triangle(ray, v0, v1, v2, tmin)
    }
}

pub struct Triangle {
    mesh_id: u32,
    triangle_id: u32,
}

pub struct Triangles {
    meshes: Vec<Mesh>,
    obj_to_world: Vec<Transformation>,
    material_ids: Vec<u32>,

    triangles: Vec<Triangle>,
    linear_intersector: LinearIntersector,
}

impl Triangles {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            obj_to_world: Vec::new(),
            material_ids: Vec::new(),
            triangles: Vec::new(),
            linear_intersector: LinearIntersector::new(),
        }
    }

    pub fn prepare_for_rendering(&mut self) {
        let calculate_bbox_fn = |idx: usize| {
            let triangle = &self.triangles[idx];
            let mesh = &self.meshes[triangle.mesh_id as usize];
            mesh.bounding_box(triangle.triangle_id as usize)
        };
        self.linear_intersector.prepare_for_rendering(self.triangles.len(), &calculate_bbox_fn);
    }

    pub fn add(&mut self, mut mesh: Mesh, object_to_world: Option<Transformation>, material_id: u32) {
        let transformation = object_to_world.unwrap_or_default();
        self.obj_to_world.push(transformation);
        self.material_ids.push(material_id);
        let triangle_count = mesh.indices.len() / 3;
        if object_to_world.is_some() {
            for vertex in mesh.vertices.iter_mut() {
                *vertex = *vertex * transformation;
            }
        }
        let mesh_id = self.meshes.len() as u32;
        for i in 0..triangle_count {
            let triangle_id = i as u32;
            let triangle = Triangle { mesh_id, triangle_id};
            self.triangles.push(triangle);
        }
        self.meshes.push(mesh);
    }

    pub fn normal(&self, _ray: &Ray, isect: &ShapeIntersection) -> Normal {
        let triangle = &self.triangles[isect.shape_id];
        let mesh = &self.meshes[triangle.mesh_id as usize];
        mesh.normal(triangle.triangle_id as usize)
    }

    pub fn material(&self, isect: &ShapeIntersection) -> u32 {
        let triangle = &self.triangles[isect.shape_id];
        self.material_ids[triangle.mesh_id as usize]
    }

    pub fn intersect(&self, ray: &Ray) -> Option<ShapeIntersection> {
        let isect_fn = |idx: usize, ray: &Ray| {
            let triangle = &self.triangles[idx];
            let mesh = &self.meshes[triangle.mesh_id as usize];
            mesh.intersect(triangle.triangle_id as usize, ray, 0.0)
        };
        self.linear_intersector.intersect(ray, &isect_fn)
    }

}

impl Default for Triangles {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Geometry {
    spheres: Primitives<Sphere>,
    triangles: Triangles,
}

pub enum GeometryIntersection {
    Sphere(ShapeIntersection),
    Triangle(ShapeIntersection),
    None
}

pub struct SurfaceInteraction {
    pub t: f32,
    pub hit_point: Point3,
    pub normal: Normal,
    pub material_id: u32,
    pub back_face: bool,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            spheres: Primitives::new(),
            triangles: Triangles::new()
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere, object_to_world: Option<Transformation>, material_id: u32) {
        self.spheres.add(sphere, object_to_world, material_id);
    }

    pub fn add_mesh(&mut self, mesh: Mesh, object_to_world: Option<Transformation>, material_id: u32) {
        self.triangles.add(mesh, object_to_world, material_id);
    }

    pub fn prepare_for_rendering(&mut self) {
        self.spheres.prepare_for_rendering();
        self.triangles.prepare_for_rendering();
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let sphere_isect = self.spheres.intersect(ray);
        let triangle_isect = self.triangles.intersect(ray);
        let sphere_isect = sphere_isect.unwrap_or(ShapeIntersection { t: -1.0, shape_id: 0 });
        let triangle_isect = triangle_isect.unwrap_or(ShapeIntersection { t: -1.0, shape_id: 0 });

        let mut current_t = 1e38;
        let mut type_id = -1;
        if sphere_isect.t > 0.0 && sphere_isect.t < current_t {
            current_t = sphere_isect.t;
            type_id = 0;
        }
        if triangle_isect.t > 0.0 && triangle_isect.t < current_t {
            type_id = 1;
        }

        match type_id {
            0 => self.surface_interaction(ray, &GeometryIntersection::Sphere(sphere_isect)),
            1 => self.surface_interaction(ray, &GeometryIntersection::Triangle(triangle_isect)),
            _ => None
        }
    }

    pub fn surface_interaction(&self, ray: &Ray, isect: &GeometryIntersection) -> Option<SurfaceInteraction> {
        match isect {
            GeometryIntersection::Sphere(shape_intersection) => {
                let hit_point = ray.point_at(shape_intersection.t);
                let mut normal = self.spheres.normal(ray, shape_intersection);
                let mut back_face = false;
                if (-ray.direction) * normal < 0.0 {
                    normal = -normal;
                    back_face = true;
                }
                let material_id = self.spheres.material(shape_intersection);
                Some(SurfaceInteraction { t: shape_intersection.t, hit_point, normal, material_id, back_face })
            }
            GeometryIntersection::Triangle(shape_intersection) => {
                let hit_point = ray.point_at(shape_intersection.t);
                let mut normal = self.triangles.normal(ray, shape_intersection);
                let mut back_face = false;
                if (-ray.direction) * normal < 0.0 {
                    normal = -normal;
                    back_face = true;
                }
                let material_id = self.triangles.material(shape_intersection);
                Some(SurfaceInteraction { t: shape_intersection.t, hit_point, normal, material_id, back_face })
            }
            GeometryIntersection::None => None
        }
    }

    pub fn from_shape_descriptions(descs: &[ShapeDescription], mat_names: &HashMap<String, usize>) -> Self {
        let mut geometry = Self::new();
        for desc in descs.iter() {
            let material_id = mat_names[&desc.material] as u32;
            match desc.typ {
                ShapeType::Sphere => {
                    let sphere = Sphere::new(desc.position, desc.radius);
                    geometry.add_sphere(sphere, desc.transform, material_id);
                }
                ShapeType::Mesh => {
                    panic!("Meshes are not supported yet");
                    // geometry.add_mesh(Mesh::from((desc.vertices, desc.indices)), None, material_id);
                }
            }
        }
        geometry.prepare_for_rendering();
        geometry
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Self::new()
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

        primitives.add(sphere1, None, 0);
        primitives.add(sphere2, None, 0);

        assert_eq!(primitives.shapes.len(), 2);
        assert_eq!(primitives.shapes[0].shape.center, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(primitives.shapes[0].shape.radius, 1.0);
        assert_eq!(primitives.shapes[1].shape.center, Point3::new(1.0, 1.0, 1.0));
        assert_eq!(primitives.shapes[1].shape.radius, 2.0);
    }
}

pub enum ShapeType {
    Sphere,
    Mesh
}

pub struct ShapeDescription {
    pub typ: ShapeType,
    pub material: String,
    pub position: Point3,
    pub radius: f32,
    pub transform: Option<Transformation>
}

impl Default for ShapeDescription {
    fn default() -> Self {
        Self {
            typ: ShapeType::Sphere,
            material: String::new(),
            position: Point3::new(0.0, 0.0, 0.0),
            radius: 0.0,
            transform: None
        }
    }
}
