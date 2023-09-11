use gamekit::app::App;
use gamekit::gfx::{
    BindGroup, BlendMode, Buffer, Color, DrawFrame, Gfx, IndexFormat, RenderPipeline,
    TextureBinding, VertexFormat, VertexLayout,
};
use gamekit::prelude::*;

use gamekit::time::Time;

// language=wgsl
const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position.x, model.position.y * -1.0, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var t_texture: texture_2d<f32>;
@group(0) @binding(1)
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.tex_coords);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
    ebo: Buffer,
    bind_group: BindGroup,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/bunny.png"))
            .build()?;

        let sampler = gfx.create_sampler().build()?;

        let bind_group = gfx
            .create_bind_group()
            .with_texture(
                TextureBinding::new()
                    .with_texture(0, &texture)
                    .with_sampler(1, &sampler)
                    .with_fragment_visibility(true),
            )
            .build()?;

        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x2),
            )
            .with_index_format(IndexFormat::UInt16)
            .with_bind_group(&bind_group)
            .with_blend_mode(BlendMode::NORMAL)
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            //pos           //coords
             0.5,  0.5,     1.0, 1.0,
             0.5, -0.5,     1.0, 0.0,
            -0.5, -0.5,     0.0, 0.0,
            -0.5,  0.5,     0.0, 1.0
        ];
        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 3,
            1, 2, 3,
        ];
        let ebo = gfx.create_index_buffer(indices).build()?;

        Ok(State {
            pip,
            vbo,
            ebo,
            bind_group,
        })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &DrawFrame, gfx: &mut Gfx, state: &mut State) {
    let mut renderer = evt.create_renderer();
    renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
    renderer.apply_pipeline(&state.pip);
    renderer.apply_buffers(&[&state.vbo, &state.ebo]);
    renderer.apply_bindings(&[&state.bind_group]);
    renderer.draw(0..6);
    gfx.render(&renderer).unwrap();
}
