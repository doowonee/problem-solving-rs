//! Find the minimum count of blocks to fit in given shape.
//! 
//! ```
//! block sizes are 1*1, 1*2, 1*3, 1*4, 2*2.
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

/// find the minimum count of blocks to fit in given shape.
fn solution(problem: &Vec<Vec<u8>>) -> usize {
    let mut pixels = 0 as usize;
    for row in problem {
        for col in row {
            if *col == 1 {
                pixels = pixels + 1;
            }
        }
    }
    // currently it returns pixels of shape
    pixels
}

mod tests {
    use super::*;
    
    #[test]
    fn count_pixel_of_shape() {
        // 1 is represent the shape
        let problem = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 1, 1],
            vec![0, 1, 0, 0]
        ];
        assert_eq!(solution(&problem), 7);
    }
}