use std::{error::Error, fs::File};

use png::Decoder;
use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, ImageDataLayout, Queue, Sampler, SamplerDescriptor,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

/// A function to create an image texture view.
///
/// WARN: It enqueues `write_texture` queue to `queue` and submits it.
pub fn create_image_texture_view(
    device: &Device,
    queue: &Queue,
    path: &str,
) -> Result<TextureView, Box<dyn Error>> {
    // load an image file
    let mut reader = Decoder::new(File::open(path)?).read_info()?;
    let mut buffer = vec![0; reader.output_buffer_size()];
    let output_info = reader.next_frame(&mut buffer)?;

    // create a texture and its view
    let size = Extent3d {
        width: output_info.width,
        height: output_info.height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&TextureViewDescriptor::default());

    // write the bitmap data to texture
    queue.write_texture(
        texture.as_image_copy(),
        &buffer,
        ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(output_info.width * 4),
            rows_per_image: None,
        },
        size,
    );
    queue.submit(None);

    Ok(texture_view)
}

/// A function to create a sampler.
pub fn create_sampler(device: &Device) -> Sampler {
    device.create_sampler(&SamplerDescriptor {
        label: None,
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        address_mode_w: AddressMode::Repeat,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
    })
}
