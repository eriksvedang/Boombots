extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;
extern crate rand;

use opengl_graphics::{ GlGraphics, OpenGL };
use std::rc::Rc;
use std::cell::RefCell;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston::input::{ Button, Key };
use piston::event::*;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as Window;

mod bot;
use bot::Bot;
use bot::Owner;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(opengl, WindowSettings::new("Boombots", [600, 600]).exit_on_esc(true));

    let window = Rc::new(RefCell::new(window));
    let ref mut gl = GlGraphics::new(opengl);

    let mut player = Bot::new(150.0, 150.0);
    player.owner = Owner::Player;
    
    let mut bots = vec![player,
                        Bot::new(230.0, 200.0),
                        Bot::new(330.0, 200.0),
                        Bot::new(130.0, 300.0),
                        Bot::new(430.0, 200.0)];

    for e in window.clone().events() {
        
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                graphics::clear([1.0, 0.8, 0.7, 1.0], graphics);
                for bot in &bots {
                    bot.draw(&context, graphics);
                }
            });
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A => bots[0].turn_left = true,
                Key::D => bots[0].turn_right = true,
                _ => ()
            }
        };

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::A => bots[0].turn_left = false,
                Key::D => bots[0].turn_right = false,
                _ => ()
            }
        };
        
        e.update(|args| {
            for bot in &mut bots {
                bot.tick(args.dt);
            }
        });
    }
}
