use crate::color::Color;
use crate::{BufferUsage, VertexFormat, VertexStepMode};
use wgpu::BufferUsages;

pub fn wgpu_color(color: Color) -> wgpu::Color {
    wgpu::Color {
        r: color.r as f64,
        g: color.g as f64,
        b: color.b as f64,
        a: color.a as f64,
    }
}

pub fn wgpu_buffer_usages(usage: BufferUsage) -> wgpu::BufferUsages {
    match usage {
        BufferUsage::Vertex => BufferUsages::VERTEX,
        BufferUsage::Index => BufferUsages::INDEX,
        BufferUsage::Uniform => BufferUsages::UNIFORM,
    }
}

pub fn wgpu_vertex_format(format: VertexFormat) -> wgpu::VertexFormat {
    match format {
        VertexFormat::UInt8x2 => wgpu::VertexFormat::Uint8x2,
        VertexFormat::UInt8x4 => wgpu::VertexFormat::Uint8x4,
        VertexFormat::Int8x2 => wgpu::VertexFormat::Sint8x2,
        VertexFormat::Int8x4 => wgpu::VertexFormat::Sint8x4,
        VertexFormat::U8x2norm => wgpu::VertexFormat::Unorm8x2,
        VertexFormat::U8x4norm => wgpu::VertexFormat::Unorm8x4,
        VertexFormat::I8x2norm => wgpu::VertexFormat::Snorm8x2,
        VertexFormat::I8x4norm => wgpu::VertexFormat::Snorm8x4,
        VertexFormat::UInt16x2 => wgpu::VertexFormat::Uint16x2,
        VertexFormat::UInt16x4 => wgpu::VertexFormat::Uint16x4,
        VertexFormat::Int16x2 => wgpu::VertexFormat::Sint16x2,
        VertexFormat::Int16x4 => wgpu::VertexFormat::Sint16x4,
        VertexFormat::U16x2norm => wgpu::VertexFormat::Unorm16x2,
        VertexFormat::U16x4norm => wgpu::VertexFormat::Unorm16x4,
        VertexFormat::Int16x2norm => wgpu::VertexFormat::Snorm16x2,
        VertexFormat::Int16x4norm => wgpu::VertexFormat::Snorm16x4,
        VertexFormat::Float16x2 => wgpu::VertexFormat::Float16x2,
        VertexFormat::Float16x4 => wgpu::VertexFormat::Float16x4,
        VertexFormat::Float32 => wgpu::VertexFormat::Float32,
        VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
        VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
        VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
        VertexFormat::UInt32 => wgpu::VertexFormat::Uint32,
        VertexFormat::UInt32x2 => wgpu::VertexFormat::Uint32x2,
        VertexFormat::UInt32x3 => wgpu::VertexFormat::Uint32x3,
        VertexFormat::UInt32x4 => wgpu::VertexFormat::Uint32x4,
        VertexFormat::Int32 => wgpu::VertexFormat::Sint32,
        VertexFormat::Int32x2 => wgpu::VertexFormat::Sint32x2,
        VertexFormat::Int32x3 => wgpu::VertexFormat::Sint32x3,
        VertexFormat::Int32x4 => wgpu::VertexFormat::Sint32x4,
    }
}

pub fn wgpu_step_mode(step_mode: VertexStepMode) -> wgpu::VertexStepMode {
    match step_mode {
        VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
        VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
    }
}
