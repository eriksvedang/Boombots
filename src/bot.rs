use graphics::{ Context, Graphics };
use graphics;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as Window;
use rand;
use rand::Rng;

enum Owner {
    Player,
    Enemy
}

pub struct Bot {
    hp: f32,
    x: f64,
    y: f64,
    speed: f64,
    angle: f64,
    owner: Owner
}

struct Shot {
    owner: Owner
}

impl Bot {
    pub fn new(x: f64, y: f64) -> Bot {
        let mut rng = rand::thread_rng();
        Bot {
            hp: 100.0,
            x: x,
            y: y,
            speed: 10.0,
            angle: rng.gen::<f64>() * 3.14 * 2.0,
            owner: Owner::Enemy
        }
    }

    pub fn tick(&mut self, dt: f64) {
        use std::f64;
        self.x += self.angle.cos() * self.speed * dt;
        self.y += self.angle.sin() * self.speed * dt;
    }
    
    pub fn draw<G: Graphics>(&self, window: &Window, c: &Context, g: &mut G) {
        use graphics::math::{Vec2d, Matrix2d};
        use graphics::math::{translate, multiply, rotate_radians};
        let translationMatrix = translate([self.x, self.y]);
        let rotationMatrix = rotate_radians(self.angle);
        let transform = multiply(multiply(c.transform, translationMatrix), rotationMatrix);
        let rect = graphics::Rectangle::new([0.0, 0.0, 0.0, 1.0]);
        rect.draw([0.0, 0.0, 20.0, 20.0], &c.draw_state, transform, g);
        let canon = graphics::Rectangle::new([0.2, 0.2, 0.4, 1.0]);
        canon.draw([5.0, 5.0, 30.0, 10.0], &c.draw_state, transform, g);
    }
}

