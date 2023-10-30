const WINDOW_WIDTH: f32 = 1600.0;
const WINDOW_HEIGHT: f32 = 1200.0;

use ggez::{
    conf::*,
    event,
    glam::*,
    graphics::{self, Mesh, DrawParam},
    Context, GameResult,
};

struct MainState {
    needs_redraw: bool
}

fn get_value(x: f32, y: f32) -> f32 {
    (x + y) * 0.01 * std::f32::consts::PI * 2.0
}

fn draw_point(x: f32, y: f32, value: f32, mb: &mut graphics::MeshBuilder) {
    let amp = 10.0;
    let xd = amp * value.cos();
    let yd = amp * value.sin();
    _ = mb.line(&[vec2(x, y), vec2(x+xd, y+yd)], 1.0, graphics::Color::from_rgb(255, 255, 0));
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { needs_redraw: true })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if !self.needs_redraw {
            return Ok(());
        }
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let mut mb = graphics::MeshBuilder::new();

        let res = 10.0;
        let mut x = 0.0;
        while x < WINDOW_WIDTH {
            let mut y = 0.0;
            while y < WINDOW_HEIGHT {
                let angle = get_value(x, y);
                draw_point(x, y, angle, &mut mb);
                y += res;
            }
            x += res;
        }
        
        let mesh = mb.build();
        canvas.draw(&Mesh::from_data(ctx, mesh), DrawParam::new());
        canvas.finish(ctx)?;
        self.needs_redraw = false;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default()
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .maximized(false)
        .fullscreen_type(FullscreenType::Windowed)
        .borderless(false)
        .min_dimensions(0.0, 0.0)
        .max_dimensions(0.0, 0.0)
        .resizable(false));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}