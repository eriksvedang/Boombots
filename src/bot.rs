use graphics::{ Context, Graphics };
use graphics;
use rand;
use rand::Rng;

pub enum Owner {
    Player,
    Enemy
}

struct MovingObject {
    x: f64,
    y: f64,
    speed: f64,
    angle: f64,
}

impl MovingObject {
    fn tick(&mut self, dt: f64) {
        self.x += self.angle.cos() * self.speed * dt;
        self.y += self.angle.sin() * self.speed * dt;
    }
}

pub struct Bot {
    //hp: f32,
    moving: MovingObject,
    pub owner: Owner,
    pub turn_left: bool,
    pub turn_right: bool,
}

pub struct Shot {
    moving: MovingObject
}

impl Bot {
    pub fn new(x: f64, y: f64) -> Bot {
        let mut rng = rand::thread_rng();
        Bot {
            //hp: 100.0,
            moving: MovingObject {
                x: x,
                y: y,
                speed: 10.0,
                angle: rng.gen::<f64>() * 3.14 * 2.0,
            },
            owner: Owner::Enemy,
            turn_left: false,
            turn_right: false,
        }
    }

    pub fn shoot(&self, shots: &mut Vec<Shot>) {
        shots.push(Shot::new(self.moving.x, self.moving.y, self.moving.angle));
    }

    pub fn tick(&mut self, dt: f64) {
        self.moving.angle += dt * match (self.turn_left, self.turn_right) {
            (true, true) => 0.0,
            (true, false) => -5.0,
            (false, true) => 5.0,
            _ => 0.0
        };
        self.moving.tick(dt);
    }
    
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::math::{translate, multiply, rotate_radians};
        let translation_matrix = translate([self.moving.x, self.moving.y]);
        let rotation_matrix = rotate_radians(self.moving.angle);
        let transform = multiply(multiply(c.transform, translation_matrix), rotation_matrix);
        let col = match self.owner {
            Owner::Player => [0.0, 0.2, 0.4, 1.0],
            Owner::Enemy => [0.8, 0.5, 0.8, 1.0]
        };
        let rect = graphics::Rectangle::new(col);
        rect.draw([-10.0, -10.0, 20.0, 20.0], &c.draw_state, transform, g);
        let canon_col = match self.owner {
            Owner::Player => [0.0, 0.8, 0.8, 1.0],
            Owner::Enemy => [0.2, 0.2, 0.4, 1.0]
        };
        let canon = graphics::Rectangle::new(canon_col);
        canon.draw([-5.0, -5.0, 30.0, 10.0], &c.draw_state, transform, g);
    }
}

impl Shot {
    pub fn new(x: f64, y: f64, angle: f64) -> Shot {
        Shot {
            moving: MovingObject {
                x: x,
                y: y,
                speed: 150.0,
                angle: angle
            },
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.moving.tick(dt);
    }
    
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::math::{translate, multiply, rotate_radians};
        let translation_matrix = translate([self.moving.x, self.moving.y]);
        let rotation_matrix = rotate_radians(self.moving.angle);
        let transform = multiply(multiply(c.transform, translation_matrix), rotation_matrix);
        let rect = graphics::Rectangle::new([1.0, 0.0, 0.0, 1.0]);
        rect.draw([-3.0, -3.0, 6.0, 6.0], &c.draw_state, transform, g);
    }
}

