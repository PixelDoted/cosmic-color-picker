use cosmic::{
    iced::{wgpu, Rectangle},
    iced_wgpu::graphics::Viewport,
    iced_widget::shader::{self, Storage},
};

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
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut Storage,
        bounds: &Rectangle,
        viewport: &Viewport,
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
        encoder: &mut wgpu::CommandEncoder,
        storage: &Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<ShaderPipeline<Uniforms, 0>>().unwrap();
        pipeline.render(target, encoder, clip_bounds);
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    hue: f32,
    saturation: f32,
    value: f32,
}
