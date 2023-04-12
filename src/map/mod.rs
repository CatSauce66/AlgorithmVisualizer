pub mod builder;
pub mod draw;

pub struct Grid {
    arr: Vec<f32>, //one long vector that reperesents the 2d field
    cell_amount: (usize, usize),
    pub cell_size: usize,
    line_thickness: f32,
}

impl Grid {

    // Make a new empty Grid
    pub fn new() -> builder::GridBuilder {
        builder::GridBuilder { arr: None, cell_amount: None, cell_size: None, line_thickness: None }
    }

    // ----------------------

    // Private function to get the index
    pub fn index( &self, x: i32, y: i32 ) -> Option<usize> {
        // Checking if x and y are not under 0 or above cell_amount
        if x >= self.cell_amount.0 as i32|| y < 0 || y >= self.cell_amount.1 as i32 || x < 0 {
            return None
        }

        Some( y as usize * self.cell_amount.0 + x as usize )
    }

    pub fn change( &mut self, index: Option<usize>, swap: f32 ) {
        // Check if the input parameter is valid
        if index == None {
            println!("Setter index doesn't exist");
            return;
        }

        self.arr[ index.unwrap() ] = swap;
    }

    pub fn get( &mut self, index: Option<usize> ) -> Option<f32> {
        // Check if the input parameter is valid
        if index == None {
            return None;
        }

        Some( self.arr[ index.unwrap() ] )
    }

    pub fn get_cell_amount( &self ) -> (usize, usize) { self.cell_amount } 
}
