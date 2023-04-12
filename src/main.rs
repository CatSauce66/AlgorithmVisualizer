use macroquad::prelude::*;

mod user_interface;
mod input_manager;
mod map;

mod algo;



fn window_conf() -> Conf {
    Conf {
        window_title: "Visualizer".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}



#[macroquad::main(window_conf)]
async fn main() {

    // Setup of camera and mous_to_world positioning
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
    let mut mouse_pos_world = camera.screen_to_world(mouse_position().into()); 

    // Select a generation algorithm
    let mut alg: Option<algo::Algorithms> = Some( algo::Algorithms::DiamondSquare( algo::DiamondSquareSettings::default() ) );
    let mut field: Option<map::Grid> = None; // Set the grid to None

    
    loop {
        // Handle user-input
        input_manager::get_input(&mut camera, &mut mouse_pos_world);
        user_interface::handle_ui(&mut alg, &mut field);


        // ----------------------------------------------------------------------------
        // Update Game State
        if let Some(_) = &mut field {
            println!("{}", field.as_ref().unwrap().cell_size );
        }
        
        // ----------------------------------------------------------------------------
        // Rendering
        clear_background(Color::new(0.1, 0.2, 0.3, 1.0));

        // Set camera as active camera
        set_camera(&camera);
            // If a grid is note None, draw it
            if let Some(grid) = &mut field {
                grid.draw_grid();
            } 
        set_default_camera();

        user_interface::draw_ui();

        // ----------------------------------------------------------------------------

        next_frame().await;
    }
}
