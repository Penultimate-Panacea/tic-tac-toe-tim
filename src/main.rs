use ndarray::*;

pub const BOARD_SIZE: usize = 3;


#[derive(Clone)]
struct Board{
    board_data: ndarray::ArrayD<char>,
    dimension: usize,
}

impl Board {
    fn new() -> Self {  
        let shape = IxDyn();
        let data = Array::<char, _>::from_elem(shape.clone(), ' ');

        
    }
}

fn main() {
    unimplemented!()
}
