//! You have various types of blocks following below.
//! 1x1, 1x2, 1x3, 1x4, 1x6, 1x10, 2x2, 2x3, 2x4, 2x6, 2x8 and 2x10
//! Find the minimum amount of blocks to fit in the given shape which is a problem.
//! 
//! Given shape and blocks are represeted as coordiate point see https://www.mathworks.com/help/images/image-coordinate-systems.html
//! 
//! If given problem is like
//! 
//! [
//!     [0, 1, 1, 1],
//!     [1, 1, 1, 0],
//!     [1, 1, 1, 0],
//!     [1, 1, 1, 0],
//! ]
//! 
//! As corodinate system it's ((1,0), (2,0), (3,0), (0,1), (0,2), (0,3), (1,1), (2,1), (1,2), (1,3), (2,2), (2,3))
//! 
//! then answer is 3 like below
//! 
//! [
//!     [0, 1, 1, 1],
//!     [2, 2, 2, 0],
//!     [3, 3, 3, 0],
//!     [3, 3, 3, 0]
//! ]
//! 
//! or 
//! 
//! [
//!     [0, 2, 2, 3],
//!     [1, 2, 2, 0],
//!     [1, 2, 2, 0],
//!     [1, 2, 2, 0]
//! ]
//! 
//! or
//! 
//! [
//!     [0, 1, 1, 1],
//!     [2, 2, 3, 0],
//!     [2, 2, 3, 0],
//!     [2, 2, 3, 0]
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

/// 1x1, 1x2, 1x3, 1x4, 1x6, 1x10, 2x2, 2x3, 2x4, 2x6, 2x8 and 2x10
const BLOCK_SIZE_POOL: [usize; 7] = [1, 2, 3, 4, 6, 8, 10];

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

/// find the minimum count of blocks to fit in given shape. It returns used blocks via points
fn solution(problem: &Shape) -> Vec<Vec<Vertex>> {
    let mut result: Vec<Vec<Vertex>> = Vec::new();

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
                result.push(horizontal_block);
                println!("{:?}th block is horizontal and points are {:?}", result.len(), result.last())
            } else {
                for point in &vertical_block {
                    // mark point as cacaulated so prevent to use dupilicated
                    caculated_points.insert(*point);
                }
                result.push(vertical_block);
                println!("{:?}th block is vertical and points are {:?}", result.len(), result.last())
            }
        }
    }

    result
}

/// scan both direction and returns list of points can be fitted.
fn scan(current_point: &Vertex, all_of_points: &HashSet<Vertex>, caculated_points: &HashSet<Vertex>) -> (Vec<Vertex>, Vec<Vertex>) {
    // scan horizontal direction first 
    let mut next_point = Vertex(current_point.0 + 1, current_point.1);
    let mut horizontal_possible_block: Vec<Vertex> = vec![*current_point];
    let mut vertical_possible_block: Vec<Vertex> = vec![*current_point];

    // if next right point is exisits without already used by another blocks.
    while all_of_points.contains(&next_point) && !caculated_points.contains(&next_point) {
        horizontal_possible_block.push(next_point);
        next_point = Vertex(next_point.0 + 1, next_point.1);
    }

    // check horizontal_possible_block is given block size
    for (i, size) in BLOCK_SIZE_POOL.iter().enumerate() {
        // if horizontal_possible_block exisists then use it. 
        if *size == horizontal_possible_block.len() {
            break;
        }
        // if there is no size to current horizontal_possible_block
        // then use closest smaller size to horizontal_possible_block
        if *size > horizontal_possible_block.len() {
            horizontal_possible_block.truncate(BLOCK_SIZE_POOL[i-1]);
            break;
        }
    }
    
    // scan vertically
    next_point = Vertex(current_point.0, current_point.1 + 1);
    // if next below point is exisits without already used by another blocks.
    while all_of_points.contains(&next_point) && !caculated_points.contains(&next_point) {
        vertical_possible_block.push(next_point);
        next_point = Vertex(next_point.0, next_point.1 + 1);
    }

    // check vertical_possible_block is given block size
    for (i, size) in BLOCK_SIZE_POOL.iter().enumerate() {
        // if vertical_possible_block exisists then use it. 
        if *size == vertical_possible_block.len() {
            break;
        }
        // if there is no size to current vertical_possible_block
        // then use closest smaller size to vertical_possible_block
        if *size > vertical_possible_block.len() {
            vertical_possible_block.truncate(BLOCK_SIZE_POOL[i-1]);
            break;
        }
    }
    
    // check if it can use double size of block
    // it is horizontally maximun so check if it can use duplicated below
    let mut can_use_double = true;
    let mut temp_list: Vec<Vertex> = Vec::new();

    for block in &horizontal_possible_block {
        let next_point = Vertex(block.0, block.1 + 1);
        // if point was already used or doesn't exisit in the shape than can not use double.
        if !all_of_points.contains(&next_point) || caculated_points.contains(&next_point) {
            can_use_double = false;
            break;
        } else {
            temp_list.push(next_point);
        }
    }
    
    // if we can use double than add points to list
    if can_use_double {
        horizontal_possible_block.append(&mut temp_list);
    }

    // if 2x8 is impossible then change single block to 6
    // @TODO currently it's hardcoding so It would be better
    if !can_use_double && horizontal_possible_block.len() == 8 {
        horizontal_possible_block.truncate(6);
    }
    
    // remove all of temp list
    // use temp_list again for vertical way
    temp_list.clear();
    can_use_double = true;

    for block in &vertical_possible_block {
        let next_point = Vertex(block.0 + 1, block.1);
        // if point was already used or doesn't exisit in the shape than can not use double.
        if !all_of_points.contains(&next_point) || caculated_points.contains(&next_point) {
            can_use_double = false;
            break;
        } else {
            temp_list.push(next_point);
        }
    }
    
    // if we can use double than add points to list
    if can_use_double {
        vertical_possible_block.append(&mut temp_list);
    }

    // if 2x8 is impossible then change single block to 6
    // @TODO currently it's hardcoding so It would be better
    if !can_use_double && vertical_possible_block.len() == 8 {
        vertical_possible_block.truncate(6);
    }

    (horizontal_possible_block, vertical_possible_block)
}

mod tests {
    use super::*;
    
    /// ```
    /// [
    ///     [0, 1, 1, 1],
    ///     [1, 1, 1, 0],
    ///     [1, 1, 1, 0],
    ///     [1, 1, 1, 0],
    /// ]
    /// ```
    #[test]
    fn should_use_thick_block() {
        let problem = vec![
            Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(0,1),
            Vertex(0,2), Vertex(0,3), Vertex(1,1), Vertex(2,1),
            Vertex(1,2), Vertex(1,3), Vertex(2,2), Vertex(2,3),
        ];

        assert_eq!(solution(&problem).len(), 3);
    }

    /// ```
    /// [
    ///     [1, 1, 1, 1, 1, 1, 1],
    ///     [1, 1, 1, 1, 1, 1, 1],
    ///     [1, 0, 0, 0, 0, 0, 0],
    ///     [1, 0, 0, 0, 0, 0, 0],
    ///     [1, 0, 0, 0, 0, 0, 0],
    /// ]
    /// ```
    #[test]
    fn should_use_thick_block2() {
        let problem = vec![
            Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(4,0), Vertex(5,0), Vertex(6,0),
            Vertex(0,1), Vertex(1,1), Vertex(2,1), Vertex(3,1), Vertex(4,1), Vertex(5,1), Vertex(6,1),
            Vertex(0,2), Vertex(0,3), Vertex(0,4),
        ];

        assert_eq!(solution(&problem).len(), 3);
    }

    /// ```
    /// [
    ///     [0, 1, 1, 1, 0],
    ///     [1, 1, 1, 1, 1],
    ///     [1, 1, 1, 0, 0],
    ///     [1, 1, 1, 0, 0],
    /// ]
    /// ```
    #[test]
    fn should_use_thick_block3() {
        let problem = vec![
            Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(0,1), Vertex(3,1), Vertex(4,1),
            Vertex(0,2), Vertex(0,3), Vertex(1,1), Vertex(2,1),
            Vertex(1,2), Vertex(1,3), Vertex(2,2), Vertex(2,3),
        ];

        assert_eq!(solution(&problem).len(), 4);
    }

    /// ```
    /// [
    ///     [1, 1, 1, 1, 1, 1, 1, 1],
    /// ]
    /// ```
    #[test]
    fn should_divided_6_and_2() {
        let problem = vec![
            Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(4,0), Vertex(5,0), Vertex(6,0), Vertex(7,0)
        ];

        let expected_result = vec![
            vec![Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(4,0), Vertex(5,0),],
            vec![Vertex(6,0), Vertex(7,0),],
        ];

        let result = solution(&problem);

        assert_eq!(result, expected_result);
    }

    /// ```
    /// [
    ///     [1],
    ///     [1],
    ///     [1],
    ///     [1],
    ///     [1],
    ///     [1],
    ///     [1],
    ///     [1],
    /// ]
    /// ```
    #[test]
    fn should_divided_6_and_2_vertically() {
        let problem = vec![
            Vertex(0,0), Vertex(0,1), Vertex(0,2), Vertex(0,3), Vertex(0,4), Vertex(0,5), Vertex(0,6), Vertex(0,7),
        ];

        let expected_result = vec![
            vec![Vertex(0,0), Vertex(0,1), Vertex(0,2), Vertex(0,3), Vertex(0,4), Vertex(0,5),],
            vec![Vertex(0,6), Vertex(0,7),],
        ];

        let result = solution(&problem);

        assert_eq!(result, expected_result);
    }

    /// ```
    /// [
    ///     [1, 1, 1, 1, 1, 1, 1, 1, 1],
    ///     [1, 1, 1, 1, 1, 1, 1, 1, 0],
    /// ]
    /// ```
    #[test]
    fn should_use_2x8() {
        let problem = vec![
            Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(4,0), Vertex(5,0), Vertex(6,0), Vertex(7,0), Vertex(8,0),
            Vertex(0,1), Vertex(1,1), Vertex(2,1), Vertex(3,1), Vertex(4,1), Vertex(5,1), Vertex(6,1), Vertex(7,1),
        ];
        
        let expected_result = vec![
            vec![
                Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0), Vertex(4,0), Vertex(5,0), Vertex(6,0), Vertex(7,0),
                Vertex(0,1), Vertex(1,1), Vertex(2,1), Vertex(3,1), Vertex(4,1), Vertex(5,1), Vertex(6,1), Vertex(7,1),
            ],
            vec![Vertex(8,0),],
        ];

        let result = solution(&problem);

        assert_eq!(result, expected_result);
    }
}