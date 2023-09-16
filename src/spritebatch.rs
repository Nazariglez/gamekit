use gk_gfx::*;
use gk_math::{Mat4, Vec2};

// language=wgsl
const SHADER: &str = r#"
struct Transform {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> transform: Transform;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uvs: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uvs: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uvs = model.uvs;
    out.pos = transform.mvp * vec4<f32>(model.pos, 0.0, 1.0);
    return out;
}

@group(0) @binding(1)
var t_texture: texture_2d<f32>;
@group(0) @binding(2)
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.uvs);
}
"#;

pub struct SpriteBatch {
    pip: RenderPipeline,
    vbo: Buffer,
    ebo: Buffer,
    ubo: Buffer,
    bind_group: BindGroup,
    vbo_data: Vec<f32>,
    ebo_data: Vec<u32>,
    projection: Mat4,
    dirty_upload: bool,
    dirty_resize: bool,
    dirty_projection: bool,
    max_elements: usize,
    element_index: usize,
}

impl SpriteBatch {
    pub fn new(tex_data: &[u8], gfx: &mut Gfx) -> Result<Self, String> {
        let max_elements = 256;

        let vbo_data: Vec<f32> = vec![0.0; max_elements * 16];
        let vbo = gfx
            .create_vertex_buffer(&vbo_data)
            .with_write_flag(true)
            .build()?;

        let ebo_data: Vec<u32> = vec![0; max_elements * 6];
        let ebo = gfx
            .create_index_buffer(&ebo_data)
            .with_write_flag(true)
            .build()?;

        let mvp = Mat4::IDENTITY; //::orthographic_rh_gl(0.0, width as _, height as _, 0.0, -1.0, 1.0);
        let ubo = gfx
            .create_uniform_buffer(mvp.as_ref())
            .with_write_flag(true)
            .build()?;

        let texture = gfx.create_texture().from_image(tex_data).build()?;

        let sampler = gfx.create_sampler().build()?;

        let bind_group = gfx
            .create_bind_group()
            .with_uniform(UniformBinding::new(0, &ubo).with_vertex_visibility(true))
            .with_texture(
                TextureBinding::new()
                    .with_texture(1, &texture)
                    .with_sampler(2, &sampler)
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
            .with_index_format(IndexFormat::UInt32)
            .with_bind_group(&bind_group)
            .with_blend_mode(BlendMode::NORMAL)
            .build()?;

        Ok(Self {
            pip,
            vbo,
            ebo,
            ubo,
            bind_group,
            vbo_data,
            ebo_data,
            dirty_upload: false,
            dirty_resize: false,
            dirty_projection: false,
            max_elements,
            element_index: 0,
            projection: mvp,
        })
    }

    fn increase_data_buffers(&mut self) {
        self.max_elements *= 2;
        self.vbo_data.resize(self.max_elements * 16, 0.0);
        self.ebo_data.resize(self.max_elements * 6, 0);
        self.dirty_resize = true;
    }

    pub fn draw(&mut self, pos: Vec2) {
        if self.max_elements < self.element_index + 1 {
            self.increase_data_buffers();
        }

        let size = Vec2::new(100.0, 100.0);
        let pos2 = pos + size;
        println!("{:?}", pos2);

        #[rustfmt::skip]
        let vertices = [
            //pos           //coords
            pos2.x,  pos2.y,     1.0, 1.0,
            pos2.x, pos.y,     1.0, 0.0,
            -pos.x, -pos.y,     0.0, 0.0,
            -pos.x,  pos2.y,     0.0, 1.0
        ];

        let vbo_index_start = self.element_index * 16;
        let vbo_index_end = vbo_index_start + 16;
        self.vbo_data
            .splice(vbo_index_start..vbo_index_end, vertices);

        let ebo_index_start = self.element_index * 6;
        let ebo_index_end = ebo_index_start + 6;
        let i = (self.element_index * 4) as u32; //4 vertices per element
        #[rustfmt::skip]
        let indices = [
            0+i, 1+i, 3+i,
            1+i, 2+i, 3+i,
        ];

        self.ebo_data
            .splice(ebo_index_start..ebo_index_end, indices);

        self.dirty_upload = true;
        self.element_index += 1;
    }

    fn resize_gpu_buffers(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        if !self.dirty_resize {
            return Ok(());
        }

        log::info!(
            "Creating a new Vertex Buffer with size: {}",
            self.vbo_data.len()
        );
        let vbo = gfx
            .create_vertex_buffer(&self.vbo_data)
            .with_write_flag(true)
            .build()?;

        self.vbo = vbo;

        log::info!(
            "Creating a new Index Buffer with size: {}",
            self.ebo_data.len()
        );
        let ebo = gfx
            .create_index_buffer(&self.ebo_data)
            .with_write_flag(true)
            .build()?;

        self.ebo = ebo;

        self.dirty_resize = false;
        self.dirty_upload = false;

        Ok(())
    }

    fn upload_gpu_buffers(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        if !self.dirty_upload {
            return Ok(());
        }

        log::info!("Uploading buffer to gpu");
        gfx.write_buffer(&self.vbo)
            .with_data(&self.vbo_data)
            .build()?;
        gfx.write_buffer(&self.ebo)
            .with_data(&self.ebo_data)
            .build()?;

        self.dirty_upload = false;

        Ok(())
    }

    fn upload_gpu_projection(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        if !self.dirty_projection {
            return Ok(());
        }

        gfx.write_buffer(&self.ubo)
            .with_data(self.projection.as_ref())
            .build()?;

        self.dirty_projection = false;
        Ok(())
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
        self.dirty_projection = true;
    }

    pub fn flush(&mut self, gfx: &mut Gfx) -> Result<(), String> {
        self.resize_gpu_buffers(gfx);
        self.upload_gpu_buffers(gfx);
        self.upload_gpu_projection(gfx);

        let index = (self.element_index * 6) as _;
        let mut renderer = Renderer::new();
        renderer.begin(1600, 800);
        renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
        renderer.apply_pipeline(&self.pip);
        renderer.apply_buffers(&[&self.vbo, &self.ebo]);
        renderer.apply_bindings(&[&self.bind_group]);
        renderer.draw(0..index);
        gfx.render(&renderer)
    }

    pub fn reset(&mut self) {
        self.element_index = 0;
    }
}