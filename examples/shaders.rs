use tetra::{graphics::{self, mesh::{Mesh, ShapeStyle}, Color, Shader, Texture}, window};
use tetra::math::Vec2;
use tetra::{Context, State};
use std::time::Instant;

struct GameState {
    shader: Shader,
    texture: Texture,
    start_time: Instant,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Load the shader
        let shader = load_noise_shader(ctx)?;

        // Load a texture
        let texture = Texture::new(ctx, "./examples/resources/dot.png")?;

        Ok(GameState {
            shader,
            texture,
            start_time: Instant::now(),
        })
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        // Update logic if needed
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        use tetra::graphics::DrawParams;

        graphics::clear(ctx, Color::BLACK);

        let (width, height) = window::get_size(ctx);
        let resolution = [width as f32, height as f32];
        let time = self.start_time.elapsed().as_secs_f32();

        // Set uniforms
        self.shader.set_uniform(ctx, "iResolution", resolution);
        self.shader.set_uniform(ctx, "iTime", time);
/*         self.shader.set_uniform(ctx, "iChannel0", &self.texture); */

        // Bind the shader
        graphics::set_shader(ctx, &self.shader);

        // Draw fullscreen quad
     /*    let mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            graphics::Rectangle::new(0.0, 0.0, width as f32, height as f32),
        );

        mesh.draw(ctx, DrawParams::new());
 */
        self.texture.draw(ctx, DrawParams {
            position: Vec2::new(0.0, 0.0),
            scale: Vec2::new(width as f32, height as f32 /2.0),
            ..Default::default()
        }); 
        // Unbind the shader
        graphics::reset_shader(ctx);

        Ok(())
    }
}




fn load_noise_shader(ctx: &mut Context) -> tetra::Result<Shader> {
/*     let vertex_source = include_str!("./examples/resources/vertex_shader.glsl");
    let fragment_source = include_str!("./examples/resources/noise_shader.glsl");
 */
    Shader::from_fragment_file(ctx, "./examples/resources/noise_shader.frag")
}


use tetra::{ContextBuilder};

fn main() -> tetra::Result {
    ContextBuilder::new("Procedural Noise Shader", 800, 600)
        .build()?
        .run(GameState::new)
}
