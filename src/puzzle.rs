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

/// vertex of block which is coordinated system.
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
    let block_pool: [Block; 5] = [
        Block { vertices: vec![Vertex(0,0)] },
        Block { vertices: vec![Vertex(0,0), Vertex(1,0)] },
        Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(2,0)] },
        Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(2,0), Vertex(3,0)] },
        Block { vertices: vec![Vertex(0,0), Vertex(1,0), Vertex(1,1), Vertex(0,1)] },
    ];
    
    println!("given shape {:?}", problem);
    for block in &block_pool {
        println!("your have: {:?}", block);
    }
    // currently it returns wrong number
    0
}

mod tests {
    use super::*;
    
    #[test]
    fn count_pixel_of_shape() {
        let problem = vec![ 
            Vertex(0,0), 
            Vertex(1,0), Vertex(1,1), Vertex(1,2), Vertex(1,3),
            Vertex(2,2), 
            Vertex(3,2),
        ];
        assert_eq!(solution(&problem), 7);
    }
}