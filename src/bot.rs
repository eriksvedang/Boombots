use graphics::{ Context, Graphics };
use graphics;
use rand;
use rand::Rng;
use graphics::math::{translate, multiply, rotate_radians};

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

    fn calculate_transform(&self, c: &Context) -> [[f64; 3]; 2] {
        let translation_matrix = translate([self.x, self.y]);
        let rotation_matrix = rotate_radians(self.angle);
        let transform = multiply(multiply(c.transform, translation_matrix), rotation_matrix);
        return transform
    }
}

pub struct Bot {
    moving: MovingObject,
    pub hp: f32,
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
            moving: MovingObject {
                x: x,
                y: y,
                speed: 20.0,
                angle: rng.gen::<f64>() * 3.14 * 2.0,
            },
            hp: 100.0,
            owner: Owner::Enemy,
            turn_left: false,
            turn_right: false,
        }
    }

    pub fn shoot(&self) -> Shot {
        Shot::new(self.moving.x, self.moving.y, self.moving.angle)
    }

    pub fn tick(&mut self, dt: f64) {
        self.moving.angle += dt * match (self.turn_left, self.turn_right) {
            (true, true) => 0.0,
            (true, false) => -5.0,
            (false, true) => 5.0,
            _ => 0.0
        };
        if self.hp > 0.0 {
            self.moving.tick(dt);
        }
    }
    
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        if self.hp > 0.0 {
            let transform = self.moving.calculate_transform(c);
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
        } else {
            let transform = self.moving.calculate_transform(c);
            let smoke = graphics::Ellipse::new([0.2, 0.2, 0.2, 0.5]);
            smoke.draw([-13.0, -13.0, 26.0, 26.0], &c.draw_state, transform, g);
        }
    }
}

impl Shot {
    pub fn new(x: f64, y: f64, angle: f64) -> Shot {
        Shot {
            moving: MovingObject {
                x: x,
                y: y,
                speed: 250.0,
                angle: angle
            },
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.moving.tick(dt);
    }
    
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        let transform = self.moving.calculate_transform(c);
        let rect = graphics::Ellipse::new([1.0, 0.0, 0.0, 1.0]);
        rect.draw([-3.0, -3.0, 6.0, 6.0], &c.draw_state, transform, g);
    }
}

