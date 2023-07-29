use crate::color::Color;
use crate::BufferUsage;
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
