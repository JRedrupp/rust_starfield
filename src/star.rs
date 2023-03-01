use rand::Rng;
use speedy2d::{Graphics2D, color::Color};

pub struct Star {
    x: f32,
    y: f32,
    size: f32,
    velocity: f32,

    px: Option<f32>,
    py: Option<f32>
}

impl Star {
    const MAX_START_SIZE: f32 = 5.0;
    const MIN_STARTING_VEL : f32 = 1.0;
    const MAX_STARTING_VEL : f32 = 5.0;
    const PATH_THICKNESS: f32 = 3.0;


    pub fn new(max_x: f32, max_y: f32) -> Self {
        
        let mut rng = rand::thread_rng();
        Star {
            x: rng.gen_range(0.0..max_x),
            y: rng.gen_range(0.0..max_y),
            size: rng.gen_range(0.0..Star::MAX_START_SIZE),
            velocity: rng.gen_range(Star::MIN_STARTING_VEL..Star::MAX_STARTING_VEL),
            px: None,
            py: None,
        }
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle((self.x, self.y), self.size, Color::WHITE);

        if self.px.is_some() && self.py.is_some() {
            graphics.draw_line((self.x, self.y), (self.px.unwrap(), self.py.unwrap()), Star::PATH_THICKNESS,Color::WHITE);
        }
    }

    fn update(&mut self, distance_speed_multiplier_max: f32, max_x: f32, max_y: f32) {
        // Store the previous position of the star
        self.px = Some(self.x);
        self.py = Some(self.y);

        // Take the vector between the star and the center of the screen and normalize it
        let mut direction = (self.x - max_x / 2.0 , self.y - max_y / 2.0);
        let magnitude = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
        direction.0 /= magnitude;
        direction.1 /= magnitude;

        // Get a velovity multiplier based on the distance from the center of the screen and the star velocity
        let distance_multiplier = magnitude / max_x; // 0.0 - 1.0
        let velocity_multiplier = 1.0 + (distance_multiplier * distance_speed_multiplier_max - 1.0); // 1.0 - DISTANCE_SPEED_MULTIPLIER

        // Move the star away from the center of the screen
        self.x += direction.0 * (self.velocity + velocity_multiplier);
        self.y += direction.1 * (self.velocity + velocity_multiplier);
        

        // If the star has moved off the screen, reset the star with a new object
        if self.x > max_x|| self.y > max_y || self.x < 0.0 || self.y < 0.0{
            *self = Star::new(max_x, max_y);
        }
    }
}

pub struct StarField {
    stars: Vec<Star>,
    pub max_x: f32,
    pub max_y: f32,
}

impl StarField {

    pub fn new(num_stars: usize, max_x: f32, max_y: f32) -> Self {
        let mut stars = Vec::new();
        for _ in 0..num_stars {
            stars.push(Star::new(max_x, max_y));
        }
        StarField {
            stars,
            max_x,
            max_y,
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D) {
        for star in &self.stars {
            star.draw(graphics);
        }
    }

    pub fn update(&mut self, distance_speed_multiplier_max: f32) {
        for star in &mut self.stars {
            star.update(distance_speed_multiplier_max, self.max_x, self.max_y);
        }
    }
}

