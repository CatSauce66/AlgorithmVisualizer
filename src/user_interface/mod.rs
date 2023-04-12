use egui_macroquad::egui;

use crate::algo;
//use crate::algo::GenerationAlgorithm;

use crate::algo::GenerationAlgorithm;
use crate::map;


pub fn handle_ui( settings: &mut Option<algo::Algorithms>, field: &mut Option<map::Grid> ) {

    egui_macroquad::ui( |egui_ctx| {
        egui::SidePanel::right("settings").show( egui_ctx, |ui| {

            match settings {
                Some( algo::Algorithms::DiamondSquare( ds_settings ) ) => {

                    let mut offset_range_start = *ds_settings.offset_range.start() as f32;
                    let mut offset_range_end = *ds_settings.offset_range.end() as f32;

                    ui.add( egui::Slider::new( &mut ds_settings.grid_n, 2..=10 )
                        .text( "Grid Size" )
                        .suffix( "x" ),
                    );

                    ui.add( egui::Slider::new( &mut offset_range_start, 0.0..=-10.0 ).text( "Offset Range End" ) );
                    ui.add( egui::Slider::new( &mut offset_range_end, -0.0..=10.0 ).text( "Offset Range End" ) );

                    ds_settings.offset_range = ( offset_range_start )..=( offset_range_end );
                    *settings = Some( algo::Algorithms::DiamondSquare( ds_settings.clone() ) )   
                },
                _ => {
                    *settings = None
                }
            };

            
            
            if ui.button( "Generate map" ).clicked() {
                
                match settings {
                    Some( algo::Algorithms::DiamondSquare( ds_settings ) ) => {
                        let mut g = ds_settings.make_grid(); //maje a mutable copie of the grid TODO: dont have copie
                        ds_settings.setup( &mut g );
                        *field = Some( g ); // Save the grid
                    }
                    None => {}
                }

                

                if let Some( grid ) = field.as_mut() {
                    match settings {
                        Some( algo::Algorithms::DiamondSquare( ds_settings ) ) => {
                            algo::DiamondSquareSettings::run( ds_settings, grid);
                        }
                        _ => {}
                    }
                } 
            } 
            
        });
    });
}

pub fn draw_ui(){
    egui_macroquad::draw();
}