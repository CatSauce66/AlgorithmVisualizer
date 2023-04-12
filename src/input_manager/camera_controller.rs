use macroquad::prelude::*;

// Function that handles zooming in/out
pub fn scroll(scroll_delta: f32, prev_world_pos: &mut Vec2, camera: &mut Camera2D){
    let sign = scroll_delta.signum(); // Get the signum (-1.0, 0.0, 1.0) of the scroll delta 

    camera.zoom.x += sign * 0.1 / screen_width(); // Devide by screen_size to not get distortion
    camera.zoom.y -= sign * 0.1 / screen_height();
    
    keep_mouse_world_pos(camera, prev_world_pos); // Keep the mouse in the same world coordinates
}

// Limits the zoom so that you can't zoom indefenetly
// TODO: some things
pub fn keep_zoom(max_zoom: f32, prev_world_pos: &mut Vec2, camera: &mut Camera2D) {
    camera.zoom.x = max_zoom;
    camera.zoom.y = camera.zoom.x * screen_width() / screen_height();

    keep_mouse_world_pos(camera, prev_world_pos);
}


pub fn keep(prev_world_pos: &mut Vec2, camera: &mut Camera2D) {
    keep_mouse_world_pos(camera, prev_world_pos);
}

// Keeps the mouse at the same World coordinates as it was
fn keep_mouse_world_pos(camera: &mut Camera2D, prev_mouse_world_pos: &mut Vec2){
    let mouse_world_pos = camera.screen_to_world(mouse_position().into()); // current mouse position on the world
    let delta = Vec2 { x: mouse_world_pos.x - prev_mouse_world_pos.x, y: mouse_world_pos.y - prev_mouse_world_pos.y };
        
    camera.target.x += -delta.x;
    camera.target.y += -delta.y;
}