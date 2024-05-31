use cosmic::iced_widget::shader::{self};

use crate::shaders::ShaderPipeline;

// ---- Shader ----
pub struct ColorGraph {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl<Message> shader::Program<Message> for ColorGraph {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        state: &Self::State,
        cursor: cosmic::iced_core::mouse::Cursor,
        bounds: cosmic::iced::Rectangle,
    ) -> Self::Primitive {
        Primitive::new(self.hue, self.saturation, self.value)
    }
}

#[derive(Debug)]
pub struct Primitive {
    uniforms: Uniforms,
}

impl Primitive {
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        Self {
            uniforms: Uniforms {
                hue,
                saturation,
                value,
            },
        }
    }
}

impl shader::Primitive for Primitive {
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
        if !storage.has::<ShaderPipeline<Uniforms, 0>>() {
            storage.store(ShaderPipeline::<Uniforms, 0>::new(
                device,
                queue,
                format,
                include_str!("hsv.wgsl"),
            ));
        }

        let pipeline = storage.get_mut::<ShaderPipeline<Uniforms, 0>>().unwrap();
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
        let pipeline = storage.get::<ShaderPipeline<Uniforms, 0>>().unwrap();
        pipeline.render(target, encoder, viewport);
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    hue: f32,
    saturation: f32,
    value: f32,
}
