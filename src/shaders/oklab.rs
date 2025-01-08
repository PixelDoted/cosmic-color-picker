use cosmic::{
    iced::{wgpu, Rectangle},
    iced_wgpu::{self, graphics::Viewport},
    iced_widget::shader::{self, Storage},
};

use crate::shaders::ShaderPipeline;

// ---- Shader ----
pub struct ColorGraph<const MODE: u32> {
    pub lightness: f32,
    pub green_red: f32,
    pub blue_yellow: f32,
}

impl<const M: u32, Message> shader::Program<Message> for ColorGraph<M> {
    type State = ();
    type Primitive = Primitive<M>;

    fn draw(
        &self,
        state: &Self::State,
        cursor: cosmic::iced_core::mouse::Cursor,
        bounds: cosmic::iced::Rectangle,
    ) -> Self::Primitive {
        Primitive::<M>::new(self.lightness, self.green_red, self.blue_yellow)
    }
}

#[derive(Debug)]
pub struct Primitive<const M: u32> {
    uniforms: Uniforms,
}

impl<const M: u32> Primitive<M> {
    pub fn new(lightness: f32, green_red: f32, blue_yellow: f32) -> Self {
        Self {
            uniforms: Uniforms {
                lightness,
                green_red,
                blue_yellow,
                mode: M,
            },
        }
    }
}

impl<const M: u32> shader::Primitive for Primitive<M> {
    fn prepare(
        &self,

        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut Storage,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        if !storage.has::<ShaderPipeline<Uniforms, M>>() {
            storage.store(ShaderPipeline::<Uniforms, M>::new(
                device,
                queue,
                format,
                include_str!("oklab.wgsl"),
            ));
        }

        let pipeline = storage.get_mut::<ShaderPipeline<Uniforms, M>>().unwrap();
        pipeline.write(queue, &self.uniforms);
    }

    fn render(
        &self,

        encoder: &mut wgpu::CommandEncoder,
        storage: &Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<ShaderPipeline<Uniforms, M>>().unwrap();
        pipeline.render(target, encoder, clip_bounds);
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    lightness: f32,
    green_red: f32,
    blue_yellow: f32,
    mode: u32,
}
