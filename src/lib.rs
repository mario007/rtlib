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

pub use crate::color::{RGBPixelSample, AccumlationBuffer};
pub use crate::rgb::ImageSize;
pub use crate::camera::{PerspectiveCameraDescriptor, PerspectiveCamera};
pub use crate::ray::Ray;
pub use crate::tile::Tile;
