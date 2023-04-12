use macroquad::prelude::*;

mod camera_controller;



const MAX_SCROLL: f32 = 0.00025;

pub fn get_input(camera: &mut Camera2D, prev_mouse_world_pos: &mut Vec2) {

    if mouse_wheel().1 != 0.0 { 
        camera_controller::scroll(mouse_wheel().1, prev_mouse_world_pos, camera);
    }

    if camera.zoom.x < MAX_SCROLL { 
        camera_controller::keep_zoom(0.00025, prev_mouse_world_pos, camera);
    }

    // ------------------------------------------------
                
    if is_mouse_button_down(MouseButton::Right) {    
        camera_controller::keep(prev_mouse_world_pos, camera);
    }


    *prev_mouse_world_pos = camera.screen_to_world(mouse_position().into()); // update so the next frame the prev_world_pos is correct
}  
