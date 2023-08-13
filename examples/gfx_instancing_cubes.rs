use gamekit::app::event;
use gamekit::gfx::{
    BindGroup, Buffer, Color, CullMode, Gfx, IndexFormat, RenderPipeline, Renderer, UniformBinding,
    VertexFormat, VertexLayout,
};
use gamekit::math::{Mat4, Vec3};
use gamekit::platform::Platform;
use gamekit::prelude::*;
use gamekit::random;
use gamekit::time::Time;
use gk_gfx::VertexStepMode;

const INSTANCES: u32 = 120;

// language=wgsl
const SHADER: &str = r#"
struct Transform {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> transform: Transform;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @builtin(instance_index) instance_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let i = f32(model.instance_index);
    let n = select(i, 0.0, i % 2.0 == 0.0);
    let j = select(i, 0.0, i % 3.0 == 0.0);
    let pos = vec4<f32>(model.position.x - i * 2.0, model.position.y - n * 2.0, model.position.z - j * 2.0, 1.0);

    var out: VertexOutput;
    out.color = model.color;
    out.position = transform.mvp * pos;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    pos_vbo: Buffer,
    color_vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    bind_group: BindGroup,
    angle: f32,
    mvp: Mat4,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        #[rustfmt::skip]
            let vertices: &[f32] = &[
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,

            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0,
            1.0, -1.0, -1.0,

            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,
        ];
        let pos_vbo = gfx.create_vertex_buffer(vertices).build()?;

        let colors = (0..INSTANCES)
            .map(|_| Color::rgb(random::gen(), random::gen(), random::gen()).to_rgba())
            .collect::<Vec<_>>();

        let color_vbo = gfx.create_vertex_buffer(&colors).build()?;

        #[rustfmt::skip]
            let indices: &[u16] = &[
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];
        let ebo = gfx.create_index_buffer(indices).build()?;

        let mvp = create_mvp();
        let ubo = gfx
            .create_uniform_buffer(mvp.as_ref())
            .with_write_flag(true)
            .build()?;

        let bind_group = gfx
            .create_bind_group()
            .with_uniform(UniformBinding::new(0, &ubo).with_vertex_visibility(true))
            .build()?;

        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(VertexLayout::new().with_attr(0, VertexFormat::Float32x3))
            .with_vertex_layout(
                VertexLayout::new()
                    .with_step_mode(VertexStepMode::Instance)
                    .with_attr(1, VertexFormat::Float32x4),
            )
            .with_bind_group(&bind_group)
            .with_index_format(IndexFormat::UInt16)
            .with_cull_mode(CullMode::Front)
            .build()?;

        Ok(State {
            pip,
            pos_vbo,
            color_vbo,
            ebo,
            ubo,
            bind_group,
            angle: 0.0,
            mvp,
        })
    }

    fn rotated_mvp(&self) -> Mat4 {
        self.mvp * Mat4::from_rotation_x(self.angle) * Mat4::from_rotation_y(self.angle)
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(Platform::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .on(on_update)
        .build()
}

fn on_update(_: &event::Update, time: &mut Time, state: &mut State) {
    state.angle += 0.6 * time.delta_f32();
}

fn on_draw(evt: &event::Draw, gfx: &mut Gfx, state: &mut State) {
    // update mvp
    gfx.write_buffer(&state.ubo)
        .with_data(state.rotated_mvp().as_ref())
        .build()
        .unwrap();

    let mut renderer = Renderer::new();
    renderer.begin(Color::rgb(0.1, 0.2, 0.3), 0, 0);
    renderer.apply_pipeline(&state.pip);
    renderer.apply_buffers(&[&state.pos_vbo, &state.color_vbo, &state.ebo]);
    renderer.apply_bindings(&[&state.bind_group]);
    renderer.draw_instanced(0..36, INSTANCES);
    gfx.render(evt.window_id, &renderer).unwrap();
}

fn create_mvp() -> Mat4 {
    let projection = Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(
        Vec3::new(4.0, 3.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    Mat4::IDENTITY * projection * view
}
