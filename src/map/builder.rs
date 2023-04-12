use macroquad::prelude::*;
use crate::map;

pub struct GridBuilder {
    pub arr: Option<Vec<f32>>, //one long vector that reperesents the 2d field
    pub cell_amount: Option<( usize, usize )>,
    pub cell_size: Option<usize>,
    pub line_thickness: Option<f32>,
}

impl GridBuilder {
    #[allow(dead_code)]
    pub fn cell_amount( &mut self, x: usize, y: usize ) -> &mut Self {
        self.cell_amount = Some( ( x, y ) );
        self
    }
    #[allow(dead_code)]
    pub fn cell_size( &mut self, size: usize ) -> &mut Self {
        self.cell_size = Some( size );
        self
    }
    #[allow(dead_code)]
    pub fn line_thickness( &mut self, thickness: f32 ) -> &mut Self {
        self.line_thickness = Some( thickness );
        self
    }

    pub fn spawn( &mut self ) -> map::Grid {
        let c_a = self.cell_amount.unwrap_or( ( 0, 0 ) );
        let g: Vec<f32> = vec![0.0; c_a.0 * c_a.1]; // Create the grid

        map::Grid {  
            arr: g,
            cell_amount: c_a,
            cell_size: self.cell_size.unwrap_or(5),
            line_thickness: self.line_thickness.unwrap_or(1.0),
        }
    }
}