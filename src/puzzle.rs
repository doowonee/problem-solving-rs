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
            vec![1, 1 ,0, 0],
            vec![0, 1 ,0, 0],
            vec![0, 1 ,1, 1],
            vec![0, 1 ,0, 0]
        ];
        assert_eq!(solution(&problem), 7);
    }
}