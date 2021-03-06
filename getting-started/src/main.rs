extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;

use sdl2_window::Sdl2Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_3_2;

use piston::{
    Window,
    Render,
    RenderArgs,
    Update,
    UpdateArgs
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64 // Rotation for the square.
}

impl App {
    fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.rgba(0.0,1.0,0.0,1.0).draw(&mut self.gl);

        // Draw a box rotating around the middle of the screen.
        context
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .rect(0.0, 0.0, 50.0, 50.0)
            .rgba(1.0, 0.0, 0.0,1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
    }

    fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let mut window = Sdl2Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::WindowSettings::default()
    );

    // Some settings for how the game should be run.
    let event_settings = piston::EventSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(OpenGL_3_2), rotation: 0.0 };

    // TODO: Change this back to a for loop after rust is fixed.
    let mut event_iter = piston::EventIterator::new(&mut window, &event_settings);
    loop {
        let e = match event_iter.next() {
            Some(e) => e,
            None => { break; }
        };
        match e {
            Render(_args) => app.render(event_iter.window, &_args),
            Update(_args) => app.update(event_iter.window, &_args),
            _ => {  }
        }
    }

    /*
     * This is broken due to a bug in rustc.
     * For more information, please read:
     * https://github.com/PistonDevelopers/piston/issues/641

    for e in event_iter {
        match e {
            Render(_args) => app.render(event_iter.window, &_args),
            Update(_args) => app.update(event_iter.window, &_args),
            _ => {  }
        }
    }*/
}
