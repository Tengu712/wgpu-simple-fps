use wgpu::{
    CompareFunction, DepthBiasState, DepthStencilState, Device, Extent3d, StencilFaceState,
    StencilState, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

/// A constant for descripting a depth texture state to create a render pipeline.
pub const DEPTH_STENCIL_STATE: DepthStencilState = DepthStencilState {
    format: TextureFormat::Depth32Float,
    depth_write_enabled: true,
    depth_compare: CompareFunction::Less,
    stencil: StencilState {
        front: StencilFaceState::IGNORE,
        back: StencilFaceState::IGNORE,
        read_mask: 0,
        write_mask: 0,
    },
    bias: DepthBiasState {
        constant: 0,
        slope_scale: 0.0,
        clamp: 0.0,
    },
};

/// A function to create a depth texture view.
pub fn create_depth_texture_view(device: &Device, width: u32, height: u32) -> TextureView {
    device
        .create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&TextureViewDescriptor::default())
}
