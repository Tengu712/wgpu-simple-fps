use std::{error::Error, fs::File};

use png::Decoder;
use wgpu::{
    Device, Extent3d, ImageDataLayout, Queue, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor,
};

/// A struct for an image texture.
///
/// Its texture format is TextureFormat::Rgba8UnormSrgb.
pub struct ImageTexture {
    pub texture_view: TextureView,
}

impl ImageTexture {
    /// A constructor.
    ///
    /// WARN: It pushes write_texture command into the queue and submits the queue.
    pub fn new(device: &Device, queue: &Queue, path: &str) -> Result<Self, Box<dyn Error>> {
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

        // finish
        Ok(Self { texture_view })
    }
}