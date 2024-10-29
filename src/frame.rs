use std::convert::From;

use crate::vec::{Vec3, Normal};


pub struct Frame {
    u: Vec3,
    v: Vec3,
    w: Vec3
}

impl Frame {
    pub fn to_local(&self, vec: Vec3) -> Vec3 {
        Vec3::new(self.u * vec, self.v * vec, self.w * vec)
    }
    pub fn to_world(&self, vec: Vec3) -> Vec3 {
        self.u * vec.x + self.v * vec.y + self.w * vec.z
    }
}

impl From<Vec3> for Frame {
    fn from(normal: Vec3) -> Self {
        let sign = 1.0f32.copysign(normal.z);
        let a = -1.0 / (sign + normal.z);
        let b = normal.x * normal.y * a;
        let b1 = Vec3::new(1.0 + sign * normal.x * normal.x * a, sign * b, -sign * normal.x);
        let b2 = Vec3::new(b, sign + normal.y * normal.y * a, -normal.y);
        Frame {
            u: b1,
            v: b2,
            w: normal
        }
    }
}

impl From<Normal> for Frame {
    fn from(normal: Normal) -> Self {
        Frame::from(Vec3::from(normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_local() {
        let frame = Frame {
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 0.0, 1.0),
        };

        let vec = Vec3::new(2.0, 3.0, 4.0);
        let local_vec = frame.to_local(vec);

        assert_eq!(local_vec, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_to_world() {
        let frame = Frame {
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 0.0, 1.0),
        };

        let vec = Vec3::new(2.0, 3.0, 4.0);
        let world_vec = frame.to_world(vec);

        assert_eq!(world_vec, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_angle() {
        let normal = Vec3::new(1.0, 2.0, 3.0).normalize();
        let vec = Vec3::new(2.0, 2.0, 2.0).normalize();
        let frame = Frame::from(normal);
        let angle = normal.angle_between(vec);
        let local_vec = frame.to_local(vec);

        assert_eq!((local_vec.z.acos() - angle).abs() < 0.000001, true);
    }
}
