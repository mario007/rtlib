//! Ray tracing library
//!
//! This low-level library contains all that you need to develop all kinds off ray tracers.
//! It has random number generator, 3D vector math library. 

pub mod rng;
pub mod math;
pub mod vec;
pub mod hash;
pub mod matrix;
pub mod isect;
pub mod rgb;
pub mod frame;
pub mod transformations;
pub mod camera;
pub mod ray;
pub mod tile;
pub mod color;
pub mod shapes;
pub mod samplings;
pub mod lights;
pub mod materials;
pub mod json;
pub mod scene;
pub mod pbrt_v4_tokenizer;
pub mod pbrt_v4;
pub mod integrators;
pub mod samplers;
pub mod filter;
pub mod bvh;
pub mod bbox;

pub use crate::color::{RGBPixelSample, AccumlationBuffer};
pub use crate::rgb::ImageSize;
pub use crate::camera::{PerspectiveCameraDescriptor, PerspectiveCamera};
pub use crate::ray::Ray;
pub use crate::tile::Tile;
pub use crate::json::load_scene_description_from_json;
pub use crate::pbrt_v4::parse_pbrt_v4_input_file;
