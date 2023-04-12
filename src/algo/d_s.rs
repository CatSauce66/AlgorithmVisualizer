use rand::{Rng, rngs::ThreadRng};

use crate::map; // This crate is used to make a grid and use its functions to mutate it
use super::GenerationAlgorithm;

// This struct will set the setting for the grid and generation
#[derive(Clone)]
pub struct DiamondSquareSettings {
    pub grid_n: u32, // This defines the grid size as 2^n+1

    pub height_range: std::ops::RangeInclusive<f32>,
    pub offset_range: std::ops::RangeInclusive<f32>,

    pub chunck_size: i32,
    pub rng: ThreadRng,
}



impl GenerationAlgorithm for DiamondSquareSettings { 
    fn make_grid(&mut self) -> map::Grid {

        // Let the grid size be 2^n+1 on both the x and y axis
        let n = usize::pow(2, self.grid_n) + 1;
        self.chunck_size = n as i32 - 1; 

        // Generate the grid
        return map::Grid::new()
            .cell_amount( n, n )
            .cell_size( 3 )
            .line_thickness( 0.0 )
            .spawn();
    }

    
    fn setup(&mut self, grid: &mut map::Grid) {
        // Generate random values for the four corners
        let c1 = self.rng.gen_range( self.height_range.clone() );
        let c2 = self.rng.gen_range( self.height_range.clone() );
        let c3 = self.rng.gen_range( self.height_range.clone() );
        let c4 = self.rng.gen_range( self.height_range.clone() );
        
        let (cell_amount_x, cell_amount_y) = (grid.get_cell_amount().0 as i32, grid.get_cell_amount().1 as i32);

        // Set the four corners to some value
        grid.change( grid.index( 0, 0 ), c1 );
        grid.change( grid.index( 0, cell_amount_y - 1), c2 );
        grid.change( grid.index( cell_amount_x - 1, 0 ), c3 );
        grid.change( grid.index( cell_amount_x - 1, cell_amount_y - 1 ), c4 );
    }

    fn run(&mut self, grid: &mut map::Grid) {

        // While chunck size is bigger than 1 square run algorithm again
        let mut offset_range_clone = self.offset_range.clone();

        while self.chunck_size > 1 {

            let half = self.chunck_size / 2;

            // Square step
            for y in ( 0..i32::pow( 2, self.grid_n ) ).step_by( self.chunck_size as usize ) {
                for x in ( 0..i32::pow( 2, self.grid_n ) ).step_by( self.chunck_size as usize) {

                    // Get the four corners of the Square
                    // NOTE: Unwarppign because these values will always exist
                    let square_index_list = [
                        grid.get( grid.index( x, y ) ).unwrap(),
                        grid.get( grid.index( x + self.chunck_size, y ) ).unwrap(),
                        grid.get( grid.index( x, y + self.chunck_size ) ).unwrap(),
                        grid.get( grid.index( x + self.chunck_size, y + self.chunck_size ) ).unwrap(),
                    ];

                    // Get Sum of all the corners
                    let mut sum = 0.0;
                    for cell in square_index_list {
                        sum += cell;
                    }

                    // Set random offset
                    let r = self.rng.gen_range( offset_range_clone.clone() );

                    // Set the center of the square
                    grid.change( grid.index( x + half, y + half), 
                        (( sum / 4.0 ) + r).round()
                    );

                }
            }

            
            // Diamond step
            for y in ( 0..i32::pow( 2, self.grid_n ) + 1 ).step_by( half as usize ) {
                for x in ( (y + half ) % self.chunck_size..i32::pow( 2, self.grid_n ) + 1 ).step_by( self.chunck_size as usize ) {

                    // Get the four corners of the Diamond
                    let diamond_corner_list = [
                        grid.get( grid.index( x, y - half ) ),
                        grid.get( grid.index( x - half, y ) ),
                        grid.get( grid.index( x + half, y ) ),
                        grid.get( grid.index( x, y + half ) ),
                    ];  
                    

                    // For each cell with value and to sum and add count
                    let ( mut count, mut sum ) = ( 0.0, 0.0 );
                    for cell in diamond_corner_list {
                        if let Some( val ) = cell {
                            sum += val;
                            count += 1.0;
                        } 
                    }     


                    // Add random offset
                    let r = self.rng.gen_range(offset_range_clone.clone());               

                    // Change the center index to the average of the diamond_index_list + random offsett
                    grid.change( grid.index(x, y), ( (sum / count) + r ).round() );

                }
            }
            
            self.chunck_size = self.chunck_size / 2;
            offset_range_clone = (offset_range_clone.start() / 2.0)..=(offset_range_clone.end() / 2.0);

        }
    }
}



impl DiamondSquareSettings {
    pub fn default() -> Self {
        Self {
            grid_n: 2,
            height_range: 0.0..=20.0,
            offset_range: -5.0..=5.0,
                
            chunck_size: 0,
            rng: rand::thread_rng(), 
        }
    }
}
