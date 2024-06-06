use cosmic::iced_widget::shader::{self};

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
        format: shader::wgpu::TextureFormat,
        device: &shader::wgpu::Device,
        queue: &shader::wgpu::Queue,
        bounds: cosmic::iced::Rectangle,
        target_size: cosmic::iced::Size<u32>,
        scale_factor: f32,
        storage: &mut shader::Storage,
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
        storage: &shader::Storage,
        target: &shader::wgpu::TextureView,
        target_size: cosmic::iced::Size<u32>,
        viewport: cosmic::iced::Rectangle<u32>,
        encoder: &mut shader::wgpu::CommandEncoder,
    ) {
        let pipeline = storage.get::<ShaderPipeline<Uniforms, M>>().unwrap();
        pipeline.render(target, encoder, viewport);
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
