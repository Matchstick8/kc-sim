extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct Sim {
   gl: GlGraphics,
   slice: Slice
}

struct Slice {
    x: i32,
    y: i32
}

impl Sim {
    fn render (&mut self, arg: &RenderArgs){
        let gray: [f32;4] = [0.1, 0.1, 0.1, 0.1];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(gray, gl);
        });
        
        self.slice.render(&mut self.gl, arg);
    }

    fn update (&mut self){
        self.slice.update();
    }
}

impl Slice {
    fn render (&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let light_gray: [f32; 4] = [0.8, 0.8, 0.8, 0.8];

        let square = graphics::rectangle::square(self.x as f64, self.y as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(light_gray, square, transform, gl)
        });
    }

    fn update (&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

fn main () {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "kc-sim", 
        [1440, 720]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        
    let mut sim = Sim { 
        gl: GlGraphics::new(opengl),
        slice: Slice {x: 0, y: 0}
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            sim.render(&r);
        }

        if let Some(u) = e.render_args() {
            sim.update();
        }
    }
}