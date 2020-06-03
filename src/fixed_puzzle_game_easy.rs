//! You have 4 types of blocks such as 1x1, 1x2, 1x3 and 1x4.
//! Find the minimum amount of blocks to fit in the given shape which is a problem.
//! 
//! Given shape and blocks are represeted as coordiate point see https://www.mathworks.com/help/images/image-coordinate-systems.html
//! 
//! If given problem is like
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
//!     [1, 2, 0, 0],
//!     [0, 2, 0, 0],
//!     [0, 2, 3, 3],
//!     [0, 2, 0, 0]
//! ]
//! 
//! [
//!     [1, 1, 0, 0],
//!     [0, 2, 0, 0],
//!     [0, 2, 3, 3],
//!     [0, 2, 0, 0]
//! ]
//! 
//! this is 4 so it's not an answer.
//! 
//! [
//!     [1, 1, 0, 0],
//!     [0, 2, 0, 0],
//!     [0, 3, 3, 3],
//!     [0, 4, 0, 0]
//! ]
//! ```
//! 

use std::collections::HashSet;

/// maximum block size is 1x4
const MAX_CAPACITY: usize = 4;

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
    
    // given problem is vector but it is not guaranteed to be sorted.
    // so problem could be possible to be [(0,0) (2,0) (1,0)].
    let mut sorted_problem = problem.clone();
    // @TODO should it be sorted near by 0,0 ?
    // currently it sorted by x axis number asc
    sorted_problem.sort();
    println!("#@ sorted_problem: {:?}", sorted_problem);
    
    // hashsets are copied from vector so vector is still accessible.
    let set_of_problem: HashSet<Vertex> = problem.iter().cloned().collect();
    let mut caculated_points: HashSet<Vertex> = HashSet::new();
    let mut count_of_blocks = 0;

    // iterate sorted point due to check right and below not left or above.
    for point in &sorted_problem {
        if !caculated_points.contains(point) {
            let (horizontal_block, vertical_block) = scan(point, &set_of_problem, &caculated_points);

            // if horizontal block can be fit larger than vertically. 
            if horizontal_block.len() >= vertical_block.len() {
                for point in &horizontal_block {
                    // mark point as cacaulated so prevent to use dupilicated
                    caculated_points.insert(*point);
                }
                count_of_blocks +=1;
                println!("{:?}th block of points are {:?}", count_of_blocks, horizontal_block)
            } else {
                for point in &vertical_block {
                    // mark point as cacaulated so prevent to use dupilicated
                    caculated_points.insert(*point);
                }
                count_of_blocks +=1;
                println!("{:?}th block of points are {:?}", count_of_blocks, vertical_block)
            }
        }
    }

    count_of_blocks
}

/// scan both direction and returns list of points can be fitted.
fn scan(current_point: &Vertex, all_of_points: &HashSet<Vertex>, caculated_points: &HashSet<Vertex>) -> (Vec<Vertex>, Vec<Vertex>) {
    // scan horizontal direction first 
    let mut next_point = Vertex(current_point.0  + 1, current_point.1);
    let mut horizontal_possible_block: Vec<Vertex> = vec![*current_point];
    let mut vertical_possible_block: Vec<Vertex> = vec![*current_point];

    // if next right point is exisits and lower than max block size without already fit by another blocks.
    while all_of_points.contains(&next_point) && horizontal_possible_block.len() < MAX_CAPACITY && !caculated_points.contains(&next_point) {
        horizontal_possible_block.push(next_point);
        next_point = Vertex(next_point.0 + 1, next_point.1);
    }

    next_point = Vertex(current_point.0, current_point.1 + 1);
    // if next bleow point is exisits and lower than max block size without already fit by another blocks.
    while all_of_points.contains(&next_point) && vertical_possible_block.len() < MAX_CAPACITY && !caculated_points.contains(&next_point) {
        vertical_possible_block.push(next_point);
        next_point = Vertex(next_point.0, next_point.1 + 1);
    }

    (horizontal_possible_block, vertical_possible_block)
}

mod tests {
    use super::*;
    
    /// ```
    /// [
    ///     [1, 1, 0, 0],
    ///     [0, 1, 0, 0],
    ///     [0, 1, 1, 1],
    ///     [0, 1, 0, 0]
    /// ]
    /// ```
    #[test]
    fn sorted_problem() {
        let problem = vec![
            Vertex(0,0),
            Vertex(1,0), Vertex(1,1), Vertex(1,2), Vertex(1,3),
            Vertex(2,2),
            Vertex(3,2),
        ];

        assert_eq!(solution(&problem), 3);
    }
    
    /// ```
    /// [
    ///     [1, 1, 0, 0],
    ///     [0, 1, 0, 0],
    ///     [0, 1, 1, 1],
    ///     [0, 1, 0, 0]
    /// ]
    /// ```
    #[test]
    fn unsorted_problem() {
        let problem = vec![
            Vertex(2,2),
            Vertex(0,0),
            Vertex(3,2),
            Vertex(1,0), Vertex(1,1), Vertex(1,2), Vertex(1,3),
        ];

        assert_eq!(solution(&problem), 3);
    }

    /// ```
    /// [
    ///     [1, 1, 0, 0],
    ///     [1, 1, 1, 0],
    ///     [0, 1, 1, 1],
    ///     [0, 1, 0, 0]
    /// ]
    /// ```
    #[test]
    fn unsorted_problem_2() {
        let problem = vec![
            Vertex(2,2), Vertex(2,1),
            Vertex(0,0), Vertex(0,1),
            Vertex(3,2),
            Vertex(1,0), Vertex(1,1), Vertex(1,2), Vertex(1,3),
        ];

        assert_eq!(solution(&problem), 4);
    }

    /// ```
    /// [
    ///     [0, 0, 1, 1, 1, 1],
    ///     [0, 1, 1, 0, 0, 1],
    ///     [1, 1, 0, 0, 0, 1],
    ///     [0, 1, 1, 0, 0, 1],
    ///     [1, 1, 1, 1, 1, 1],
    ///     [0, 1, 0, 1, 0, 0],
    ///     [1, 1, 0, 0, 0, 0],
    /// ]
    /// ```
    #[test]
    fn unsorted_larger_than_4() {
        let problem = vec![
            Vertex(2,0), Vertex(2,1), Vertex(2,3), Vertex(2,4),
            Vertex(0,2), Vertex(0,4), Vertex(0,6),
            Vertex(3,0), Vertex(3,4), Vertex(3,5),
            Vertex(4,0), Vertex(4,4),
            Vertex(1,1), Vertex(1,2), Vertex(1,3), Vertex(1,4), Vertex(1,5), Vertex(1,6),
            Vertex(5,0), Vertex(5,1), Vertex(5,2), Vertex(5,3), Vertex(5,4),
        ];

        assert_eq!(solution(&problem), 10);
    }
}