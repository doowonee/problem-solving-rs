//! Find the minimum count of blocks to fit in given shape.
//! 
//! Gien shape an blocks are represeted as coordiate point see https://www.mathworks.com/help/images/image-coordinate-systems.html
//! 
//! ```
//! block types are .
//! 
//! 1x1 (0,0)
//! 1x2 (0,0), (1,0)
//! 1x3 (0,0), (1,0), (2,0)
//! 1x4 (0,0), (1,0), (2,0) (3,0)
//! 2x2 (0,0), (1,0), (1,1) (0,1)
//! 
//! If given problem is
//! 
//! [
//!     [1, 1, 0, 0],
//!     [0, 1, 0, 0],
//!     [0, 1, 1, 1],
//!     [0, 1, 0, 0]
//! ]
//! 
//! As corodinate system it's ((0,0), (1,0), (1.1), (1,2), (1,3), (2,2), (3,2))
//! 
//! then minumum is 3 like below
//! 
//! [
//!     [1, 4, 0, 0],
//!     [0, 4, 0, 0],
//!     [0, 4, 2, 2],
//!     [0, 4, 0, 0]
//! ]
//! 
//! [
//!     [2, 2, 0, 0],
//!     [0, 3, 0, 0],
//!     [0, 3, 2, 2],
//!     [0, 3, 0, 0]
//! ]
//! 
//! this is 4 so it's not an answer.
//! 
//! [
//!     [2, 2, 0, 0],
//!     [0, 1, 0, 0],
//!     [0, 3, 3, 3],
//!     [0, 1, 0, 0]
//! ]
//! ```
//! 

use std::collections::HashSet;

/// vertex of block which is coordinated system.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Vertex(u32, u32);

/// show only coordinate point without sturct name
impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
        .field(&self.0)
        .field(&self.1)
        .finish()
    }
}

/// list of vertex which means shape of the block.
struct Block {
    vertices: Vec<Vertex>,
}

/// basically Shape is list of vertices but It's not a Block
type Shape = Vec<Vertex>;

/// show only list of vertex without sturct name
impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.vertices).finish()
    }
}

/// find the minimum count of blocks to fit in given shape.
fn solution(problem: &Shape) -> usize {
    // Blocks which can fit in the given shape.
    // let block_pool: [Block; 5] = [
    //     Block { vertices: vec![Vertex(0,0)] },
    //     Block { vertices: vec![Vertex(0,0), Vertex(1,0)] },
    //     Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(2,0)] },
    //     Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0)] },
    //     Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(1,1), Vertex(0,1)] },
    // ];
    
    println!("#@ given vector of points:  {:?}", problem);
    
    // given problem is vector but it is not guaranteed to be sorted.
    // so problem could be possible to be [(0,0) (2,0) (1,0)].
    let mut sorted_problem = problem.clone();
    sorted_problem.sort();
    println!("#@ sorted_problem: {:?}", sorted_problem);
    
    // hashsets are copied from vector so vector is still accessible.
    let set_of_problem: HashSet<Vertex> = problem.iter().cloned().collect();
    let mut caculated_points: HashSet<Vertex> = HashSet::new();
    for point in &sorted_problem {
        if !caculated_points.contains(point) {
            // skip iterate to scan toward right
            scan_right(point, &set_of_problem, &mut caculated_points);
        }
    }

    // currently it returns wrong number
    0
}

fn scan_right(current_point: &Vertex, all_of_points: &HashSet<Vertex>, caculated_points: &mut HashSet<Vertex>) {
    println!("#@ scan from {:?} to right", current_point);
    // check to right from start point
    let mut next_point = Vertex(current_point.0 + 1, current_point.1);
    let mut max_size = 1;
    while all_of_points.contains(&next_point) {
        // if next point can be a part of block then record it.
        caculated_points.insert(next_point);
        max_size += 1;
        next_point = Vertex(next_point.0 + 1, next_point.1);
    }
    println!("width is {:?}", max_size)
}

mod tests {
    use super::*;
    
    #[test]
    fn first_shape() {
        // points are un sorted vector.
        let problem = vec![
            Vertex(3,2),
            Vertex(2,2),
            Vertex(1,0), Vertex(1,1), Vertex(1,2), Vertex(1,3),
            Vertex(0,0),
        ];
        assert_eq!(solution(&problem), 3);
    }
}