extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod game_state;

pub struct App {
    gl: GlGraphics, //OpenGL drawing backend
    state: game_state::GameState
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.state.player.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            //clear screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x,y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            //draw box rotating around middle of screen
            rectangle(RED, square, transform, gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.state.player.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_3;
    let viewport = [640, 480];

    //create Glutin window
    let mut window: Window = WindowSettings::new(
            "Hello Piston!",
            viewport
        )
        .opengl(OpenGL::V3_3)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: game_state::GameState::new(0.0)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}


