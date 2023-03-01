use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;
use speedy2d::color::Color;
mod star;


const DEFAULT_WINDOW_HEIGHT: u32 = 480;
const DEFAULT_WINDOW_WIDTH: u32 = 640;

struct MyWindowHandler {
    star_field: star::StarField,
    velocity_multiplier: f32,
}

impl MyWindowHandler {
    fn new() -> Self {
        MyWindowHandler {
            star_field: star::StarField::new(100, DEFAULT_ as f32, DEFAULT_WINDOW_HEIGHT as f32),
            velocity_multiplier: 1.0,
        }
    }

    fn update_screen_size(&mut self, width: u32, height: u32) {
        self.star_field.max_x = width as f32;
        self.star_field.max_y = height as f32;
    }
}


impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::BLACK);
        self.star_field.draw(graphics);
        self.star_field.update(self.velocity_multiplier);


        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_mouse_wheel_scroll(
            &mut self,
            _: &mut WindowHelper<()>,
            distance: speedy2d::window::MouseScrollDistance
        ) {
        
        if let speedy2d::window::MouseScrollDistance::Lines{x:_, y, z:_} = distance {
            self.velocity_multiplier += y as f32;
        }
        println!("Changing Velocity Multiplier to: {distance_speed_multiplier_max}", distance_speed_multiplier_max = self.velocity_multiplier);
        
    }

    fn on_resize(&mut self, _: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {
        println!("Window resized to: {width}x{height}", width = size_pixels.x, height = size_pixels.y);
        self.update_screen_size(size_pixels.x, size_pixels.y);
        
    }
}


fn main() {
    let window = Window::new_centered("Starfield", (DEFAULT_, DEFAULT_WINDOW_HEIGHT)).unwrap();
    window.run_loop(MyWindowHandler::new())
}
