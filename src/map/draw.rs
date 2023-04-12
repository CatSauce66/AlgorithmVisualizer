use macroquad::{prelude::*, color};

use crate::map;

impl map::Grid {
    pub fn draw_grid(&mut self) {

        let grid_background = color::BLACK;
        let mut cell_color;
    
        draw_rectangle( 0.0, 0.0,
                      ( self.cell_size * self.cell_amount.0 ) as f32 + ( self.line_thickness * ( self.cell_amount.0 + 1 ) as f32 ), 
                      ( self.cell_size * self.cell_amount.1 ) as f32 + ( self.line_thickness * ( self.cell_amount.1 + 1 ) as f32 ), grid_background );
        
        
        for y in 0..self.cell_amount.1 {
            for x in 0..self.cell_amount.0 {
                
                let index = self.index( x as i32, y as i32 );

                if self.get( index ).unwrap() <= 2.5 {
                    cell_color = Color::from_rgba(33, 19, 100, 255); // Close to black
    
                }else if self.get( index ).unwrap() <= 4.0 {
                    cell_color = color::DARKBLUE;    
    
                } else if self.get( index ).unwrap() <= 5.0 {
                    cell_color = color::BLUE;
    
                } else if self.get( index ).unwrap() <= 6.0 {
                    cell_color = Color::from_rgba(125, 175, 190, 255); // Light blue
    
                } else if self.get( index ).unwrap() <= 7.0 {
                    cell_color = Color::from_rgba(255, 214, 10, 255); // Yellow
    
                } else if self.get( index ).unwrap() <= 8.0 {
                    cell_color = Color::from_rgba(255, 195, 0, 255); // Darker yellow
    
                } else if self.get( index ).unwrap() <= 10.0 {
                    cell_color = color::GREEN;
    
                } else if self.get( index ).unwrap() <= 13.0 {
                    cell_color = color::DARKGREEN;
    
                } else if self.get( index).unwrap() <= 16.0 {
                    cell_color = color::GRAY;
    
                } else if self.get( index ).unwrap() <= 18.0 {
                    cell_color = color::LIGHTGRAY;
    
                } else if self.get( index ).unwrap() <= 20.0 {
                    cell_color = color::WHITE;

                } else {
                    cell_color = color::PINK;
                }
    
                draw_rectangle( ( self.line_thickness * ( x + 1 ) as f32 ) + ( x * self.cell_size ) as f32,
                                ( self.line_thickness * ( y + 1 ) as f32 ) + ( y * self.cell_size ) as f32,
                                self.cell_size as f32, self.cell_size as f32, cell_color );
            }
        }
    }
}
