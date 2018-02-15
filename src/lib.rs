extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate glutin_window;

use glutin_window::GlutinWindow as Window;
use piston::input::{UpdateEvent, RenderEvent, MouseCursorEvent};

pub type Color = graphics::types::Color;

pub struct Graphics<'a> {
    gl: &'a mut opengl_graphics::GlGraphics
}

const INVISIBLE: Color = [0.0; 4];

impl<'a> Graphics<'a> {
    pub fn rectangle(&mut self, state: DrawState, width: f64, height: f64) {
        if state.fill.is_none() && state.stroke.is_none() {
            return; // nothing to draw
        }
        let coords = [0.0, 0.0, width, height];
        let rect = graphics::rectangle::Rectangle {
            color: state.fill.unwrap_or(INVISIBLE),
            shape: graphics::rectangle::Shape::Square,
            border: state.stroke.and_then(|color| {
                Some(graphics::rectangle::Border {
                    color,
                    radius: state.stroke_width
                })
            })
        };
        rect.draw(coords, &Default::default(), state.transform, self.gl);
    }
}

pub struct DrawState {
    transform: graphics::math::Matrix2d,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: f64
}

impl DrawState {
    fn new(transform: graphics::math::Matrix2d) -> Self {
        DrawState {
            transform,
            fill: None,
            stroke: None,
            stroke_width: 1.0
        }
    }
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = Some(color);
        self
    }
    pub fn translate(mut self, x: f64, y: f64) -> Self {
        use graphics::Transformed;
        self.transform = self.transform.trans(x, y);
        self
    }
    pub fn rotate(mut self, angle: f64) -> Self {
        use graphics::Transformed;
        self.transform = self.transform.rot_rad(angle);
        self
    }
}

pub struct Context {
    args: piston::input::RenderArgs,
    ctx: graphics::context::Context,
    pub dt: f64,
    pub mouse_x: f64,
    pub mouse_y: f64
}

impl Context {
    pub fn width(&self) -> f64 {
        self.args.width.into()
    }
    pub fn height(&self) -> f64 {
        self.args.height.into()
    }
    pub fn state(&self) -> DrawState {
        DrawState::new(self.ctx.transform)
    }
}

pub struct WindowBuilder {
    settings: piston::window::WindowSettings,
    draw: Option<Box<FnMut(Graphics, Context) -> ()>>
}

const OPENGL_VERSION: opengl_graphics::OpenGL = opengl_graphics::OpenGL::V2_1;

impl WindowBuilder {
    pub fn run(mut self) {
        let mut window: Window = self.settings.build().unwrap();
        let mut gl = opengl_graphics::GlGraphics::new(OPENGL_VERSION);
        let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
        let mut render_dt = 0.0;
        let mut mouse_x = 0.0;
        let mut mouse_y = 0.0;
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                if let Some(ref mut draw) = self.draw {
                    gl.draw(r.viewport(), |c, glo| {
                        let ctx = Context {
                            args: r,
                            ctx: c,
                            dt: render_dt,
                            mouse_x,
                            mouse_y
                        };
                        render_dt = 0.0;
                        let graphics = Graphics {
                            gl: glo
                        };
                        draw(graphics, ctx);
                    });
                }
            }
            if let Some(u) = e.update_args() {
                render_dt += u.dt;
            }
            if let Some(m) = e.mouse_cursor_args() {
                mouse_x = m[0];
                mouse_y = m[1];
            }
        }
    }
    pub fn draw<F: 'static + FnMut(Graphics, Context) -> ()>
        (mut self, draw: F) -> Self {
        self.draw = Some(Box::new(draw));
        self
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
