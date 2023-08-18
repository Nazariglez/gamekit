use gamekit::app::event;
use gamekit::gfx::{Buffer, Color, Gfx, RenderPipeline, Renderer, VertexFormat, VertexLayout};
use gamekit::platform::Platform;
use gamekit::prelude::*;
use gk_gfx::{ColorMask, CompareMode, Stencil, StencilAction};

// TODO https://webglfundamentals.org/webgl/lessons/webgl-qna-how-to-use-the-stencil-buffer.html
// https://github.com/Nazariglez/notan/blob/0815528fd42e96fd1d2299871c3e49251cf684bf/crates/notan_draw/src/manager.rs#L202
// https://maxammann.org/posts/2022/01/wgpu-stencil-testing/
// https://stackoverflow.com/questions/76240723/why-webgpu-stencil-buffer-2d-clipping-result-invisible-when-antialias-enabled
// https://learnopengl.com/Advanced-OpenGL/Stencil-testing
// https://carmencincotti.com/2022-06-13/webgpu-the-depth-texture/

// language=wgsl
const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.position = vec4<f32>(model.position - 0.5, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
    mask_pip: RenderPipeline,
    mask_vbo: Buffer,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x3),
            )
            .with_stencil(Stencil {
                stencil_fail: StencilAction::Keep,
                depth_fail: StencilAction::Keep,
                pass: StencilAction::Replace,
                compare: CompareMode::Always,
                read_mask: 0xff,
                write_mask: 0xff,
                reference: 1,
            })
            .with_color_mask(ColorMask::NONE)
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            0.5, 1.0,   1.0, 0.0, 0.0,
            0.0, 0.0,   0.0, 1.0, 0.0,
            1.0, 0.0,   0.0, 0.0, 1.0,
        ];

        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        let mask_pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x3),
            )
            .with_depth_stencil(CompareMode::Never, true)
            .with_stencil(Stencil {
                stencil_fail: StencilAction::Keep,
                depth_fail: StencilAction::Keep,
                pass: StencilAction::Replace,
                compare: CompareMode::Equal,
                read_mask: 0xff,
                write_mask: 0x00,
                reference: 1,
            })
            .with_color_mask(ColorMask::ALL)
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            0.5, 0.1,   0.0, 0.0, 0.0,
            0.65, 0.5,   0.0, 0.0, 0.0,
            0.35, 0.5,   0.0, 0.0, 0.0,
        ];

        let mask_vbo = gfx.create_vertex_buffer(vertices).build()?;

        Ok(State {
            pip,
            vbo,
            mask_pip,
            mask_vbo,
        })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(Platform::config())?
        .add_config(Gfx::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &event::Draw, gfx: &mut Gfx, state: &mut State) {
    let mut renderer = Renderer::new();
    renderer.begin(1600, 1200);
    renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, Some(0));
    renderer.apply_pipeline(&state.pip);
    renderer.apply_buffers(&[&state.vbo]);
    renderer.draw(0..3);
    // gfx.render(evt.window_id, &renderer).unwrap();

    // let mut renderer = Renderer::new();
    renderer.begin(1600, 1200);
    // renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
    renderer.apply_pipeline(&state.mask_pip);
    renderer.apply_buffers(&[&state.mask_vbo]);
    renderer.draw(0..3);

    gfx.render(evt.window_id, &renderer).unwrap();
}
