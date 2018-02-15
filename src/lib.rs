extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;

use glutin_window::GlutinWindow as Window;
use piston::input::UpdateEvent;
use piston::input::RenderEvent;

pub type Color = graphics::types::Color;

pub struct Graphics<'a> {
    gl: &'a mut opengl_graphics::GlGraphics
}

impl<'a> Graphics<'a> {
    pub fn rectangle(&mut self, state: DrawState, width: f64, height: f64) {
        if let Some(fill) = state.fill {
            graphics::rectangle(fill, [0.0, 0.0, width, height], state.transform, self.gl);
        }
    }
}

pub struct DrawState {
    transform: graphics::math::Matrix2d,
    fill: Option<Color>,
    stroke: Option<Color>
}

pub struct Context {
    args: piston::input::RenderArgs,
    ctx: graphics::context::Context
}

pub struct WindowBuilder {
    settings: piston::window::WindowSettings,
    draw: Option<Box<Fn(Graphics, Context) -> ()>>
}

const OPENGL_VERSION: opengl_graphics::OpenGL = opengl_graphics::OpenGL::V2_1;

impl WindowBuilder {
    pub fn run(&self) {
        let mut window: Window = self.settings.build().unwrap();
        println!("got window");
        let mut gl = opengl_graphics::GlGraphics::new(OPENGL_VERSION);
        let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                println!("{:?}", r);
                if let Some(ref draw) = self.draw {
                    gl.draw(r.viewport(), |c, glo| {
                        let ctx = Context {
                            args: r,
                            ctx: c
                        };
                        let graphics = Graphics {
                            gl: glo
                        };
                        draw(graphics, ctx);
                    });
                }
            }
            if let Some(u) = e.update_args() {
            }
        }
    }
}

pub fn create_window(title: &str, width: u32, height: u32) -> WindowBuilder {
    WindowBuilder {
        settings: piston::window::WindowSettings::new(
                      title, [width, height])
            .opengl(OPENGL_VERSION)
            .srgb(false),
        draw: None
    }
}
