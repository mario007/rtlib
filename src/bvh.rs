use crate::bbox::AABB;
use crate::ray::Ray;
use crate::shapes::ShapeIntersection;
use crate::vec::Vec3;
use std::collections::HashSet;



#[derive(Clone, Copy, Debug)]
struct BVHNode {
    bbox: AABB,
    left_node: u32,
    first_prim: u32,
    num_prims: u32,
}

impl BVHNode {
    fn is_leaf(&self) -> bool {
        self.num_prims > 0
    }
}

pub struct BVH {
    nodes: Vec<BVHNode>,
    primitive_indices: Vec<u32>,
}

impl BVH {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            primitive_indices: Vec::new(),
        }
    }

    pub fn build(&mut self, n_primitives: usize,
                 calculate_bbox_fn: &dyn Fn(usize) -> AABB) {

        self.primitive_indices = Vec::from_iter(0..n_primitives as u32);
        self.nodes.clear();
        self.nodes.reserve(n_primitives);
        if n_primitives == 0 {
            return;
        }

        let root_node = BVHNode {
            bbox: self.calculate_bbox(0, n_primitives, calculate_bbox_fn),
            left_node: 0,
            first_prim: 0,
            num_prims: n_primitives as u32,
        };
        self.nodes.push(root_node);

        let mut stack_nodes: [usize; 64] = [0; 64];
        let mut stack_ptr = 0;
        stack_nodes[stack_ptr] = 0;
        stack_ptr += 1;

        while stack_ptr > 0 {
            stack_ptr -= 1;
            let node_idx = stack_nodes[stack_ptr];

            // Get node data without holding a reference
            let (bbox, first_prim, num_prims) = {
                let node = &self.nodes[node_idx];
                if node.num_prims <= 2 {
                    continue;
                }
                (node.bbox, node.first_prim, node.num_prims)
            };

            let (mut i, _j) = self.partition_primitives(&bbox, first_prim, num_prims, calculate_bbox_fn);

            let mut left_count = i - first_prim as i32;
            // if all primitives are on one side of the split, split in the middle
            if left_count == 0 || left_count == num_prims as i32 {
                i = first_prim as i32 + num_prims as i32 / 2;
                left_count = i - first_prim as i32;
            }
            let right_count = num_prims - left_count as u32;
            let left_node = BVHNode {
                bbox: self.calculate_bbox(first_prim as usize, left_count as usize, calculate_bbox_fn),
                left_node: 0,
                first_prim: first_prim,
                num_prims: left_count as u32,
            };
            let right_node = BVHNode {
                bbox: self.calculate_bbox(i as usize, right_count as usize, calculate_bbox_fn),
                left_node: 0,
                first_prim: i as u32,
                num_prims: right_count,
            };
            // Update parent and push children
            let left_idx = self.nodes.len();
            self.nodes[node_idx].left_node = left_idx as u32;
            self.nodes[node_idx].num_prims = 0;
            
            self.nodes.push(left_node);
            self.nodes.push(right_node);
            
            stack_nodes[stack_ptr] = left_idx;
            stack_ptr += 1;
            stack_nodes[stack_ptr] = left_idx + 1;
            stack_ptr += 1;
        }
        // println!("Number of nodes: {} {}", self.nodes.len(), self.primitive_indices.len());
        // for node in self.nodes.iter() {
        //     println!("Node: {} {} {} {}", node.is_leaf(), node.left_node, node.first_prim,  node.num_prims);
        // }

    }

    fn calculate_axis_and_split_pos(&self, bbox: &AABB) -> (usize, f32) {
        let extent = bbox.max - bbox.min;
        let axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };
        (axis, bbox.min[axis] + extent[axis] * 0.5)
    }

    fn calculate_bbox(&self, start: usize, n_primitives: usize, calculate_bbox_fn: &dyn Fn(usize) -> AABB) -> AABB {
        let mut bbox = calculate_bbox_fn(self.primitive_indices[start] as usize);
        for i in start + 1..start + n_primitives {
            bbox = bbox.union(&calculate_bbox_fn(self.primitive_indices[i] as usize));
        }
        bbox
    }

    fn partition_primitives(&mut self, bbox: &AABB, first_prim: u32, num_prims: u32,
        calculate_bbox_fn: &dyn Fn(usize) -> AABB) -> (i32, i32) {

        let (axis, split_pos) = self.calculate_axis_and_split_pos(&bbox);

        // partition primitives into two halves
        let mut i: i32 = first_prim as i32;
        let mut j: i32 = i + num_prims as i32 - 1;
        while i <= j {
            let prim = self.primitive_indices[i as usize];
            let c = calculate_bbox_fn(prim as usize).centroid()[axis];
            if c < split_pos {
                i += 1;
            }
            else {
                let tmp = self.primitive_indices[j as usize];
                self.primitive_indices[j as usize] = self.primitive_indices[i as usize];
                self.primitive_indices[i as usize] = tmp;
                j -= 1; 
            }
        }
        (i, j)
    }

    pub fn intersect(&self, ray: &Ray,
        isect_fn: &dyn Fn(usize, &Ray) -> Option<f32>) -> Option<ShapeIntersection> {
        
        let mut primitive_id = 0;
        const BIG_NUMBER: f32 = 1e30;
        let mut current_t = BIG_NUMBER;
        let rd = ray.direction;
        let inv_rd = Vec3::new(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);

        let mut stack_nodes: [usize; 64] = [0; 64];
        let mut stack_ptr = 0;
        stack_nodes[stack_ptr] = 0;
        stack_ptr += 1;

        while stack_ptr > 0 {
            stack_ptr -= 1;
            let node_idx = stack_nodes[stack_ptr];
            let node = self.nodes[node_idx];

            let first_prim = node.first_prim as usize;
            let num_prims = node.num_prims as usize;
            if node.bbox.intersect(ray.origin, inv_rd)
            {
                if node.is_leaf() {
                    for i in first_prim..first_prim + num_prims {
                        let idx = self.primitive_indices[i] as usize;
                        let result = isect_fn(idx, ray);
                        if let Some(t) = result {
                            if t < current_t {
                                current_t = t;
                                primitive_id = idx;
                            }
                        }
                    }
                } else {
                    stack_nodes[stack_ptr] = node.left_node as usize;
                    stack_ptr += 1;
                    stack_nodes[stack_ptr] = (node.left_node + 1) as usize;
                    stack_ptr += 1;
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

#[derive(Clone, Copy, Debug)]
struct BVHUpNode {
    bbox: AABB,
    is_leaf: bool,
    left_node: u32,
    right_node: u32,
    primitive_id: u32,
}


pub struct BVHUp {
    nodes: Vec<BVHUpNode>,
}

impl BVHUp {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
    pub fn build(&mut self, n_primitives: usize,
        calculate_bbox_fn: &dyn Fn(usize) -> AABB) {
        
        self.nodes.clear();
        self.nodes.reserve_exact(n_primitives * 2);

        let mut active_nodes = HashSet::new();
        for i in 0..n_primitives {
            let bbox = calculate_bbox_fn(i);
            let node = BVHUpNode {
                bbox,
                is_leaf: true,
                left_node: 0,
                right_node: 0,
                primitive_id: i as u32,
            };
            active_nodes.insert(self.nodes.len());
            self.nodes.push(node);
        }

        loop {
            if active_nodes.len() == 1 {
                break
            }
            let mut min_area = f32::INFINITY;
            let mut l1 = 0;
            let mut l2 = 0;
            for n1 in active_nodes.iter() {
                for n2 in active_nodes.iter() {
                    if n1 != n2 {
                        let area = &self.nodes[*n1].bbox.union(&self.nodes[*n2].bbox).area();
                        if *area < min_area {
                            l1 = *n1;
                            l2 = *n2;
                            min_area = *area;
                        }
                    }
                }
            }
    
            if l1 == l2 {
                break
            }
    
            let bbox = self.nodes[l1].bbox.union(&self.nodes[l2].bbox);
            let node = BVHUpNode {
                bbox: bbox,
                is_leaf: false,
                left_node: l1 as u32,
                right_node: l2 as u32,
                primitive_id: 0,
            };
            active_nodes.remove(&l1);
            active_nodes.remove(&l2);
            active_nodes.insert(self.nodes.len());
            self.nodes.push(node);
        }
    }

    pub fn intersect(&self, ray: &Ray,
        isect_fn: &dyn Fn(usize, &Ray) -> Option<f32>) -> Option<ShapeIntersection> {
        
        let mut primitive_id = 0;
        const BIG_NUMBER: f32 = 1e30;
        let mut current_t = BIG_NUMBER;
        let rd = ray.direction;
        let inv_rd = Vec3::new(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);

        let mut stack_nodes: [usize; 64] = [0; 64];
        let mut stack_ptr = 0;
        stack_nodes[stack_ptr] = self.nodes.len() - 1;
        stack_ptr += 1;

        while stack_ptr > 0 {
            stack_ptr -= 1;
            let node_idx = stack_nodes[stack_ptr];
            let node = self.nodes[node_idx];

            if node.bbox.intersect(ray.origin, inv_rd)
            {
                if node.is_leaf {
                    let result = isect_fn(node.primitive_id as usize, ray);
                    if let Some(t) = result {
                        if t < current_t {
                            current_t = t;
                            primitive_id = node.primitive_id as usize;
                        }
                    }
                } else {
                    stack_nodes[stack_ptr] = node.left_node as usize;
                    stack_ptr += 1;
                    stack_nodes[stack_ptr] = node.right_node as usize;
                    stack_ptr += 1;
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



#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_bvh() {
        let val = mem::size_of::<BVHNode>();
        println!("{}", val);
    }
}
