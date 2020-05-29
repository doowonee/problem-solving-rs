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

// 블럭 종류를 const로 포인트의 리스트를 하나의 블록이라 하고 블록 종류는 리스트로 못만드나?
// tuple의 tuple 어캐 하누

/// find the minimum count of blocks to fit in given shape.
fn solution(problem: &Vec<Vec<u8>>) -> usize {
    // let mut pixels = 0 as usize;
    for row in problem {
        for col in row {
            for block in &BLOCKS {
                println!("{:?} {:?}", col, block)
            }
        }
    }
    // currently it returns pixels of shape
    0
}

mod tests {
    use super::*;
    
    #[test]
    fn count_pixel_of_shape() {
        let problem = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 1, 1],
            vec![0, 1, 0, 0]
        ];
        assert_eq!(solution(&problem), 3);
    }
}