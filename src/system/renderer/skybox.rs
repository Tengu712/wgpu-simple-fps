//! This code is an implementation of skybox.wgsl.

use super::{model::Model, texture::ImageTexture};
use crate::util::{camera::CameraController, memory};
use glam::{Mat4, Vec3};
use std::{borrow::Cow, mem, process};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferSize,
    BufferUsages, Color, ColorTargetState, CommandEncoder, CompareFunction, DepthBiasState,
    DepthStencilState, Device, Extent3d, FilterMode, FragmentState, IndexFormat, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, StencilState, StoreOp, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexState, VertexStepMode,
};

const SHADER: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/skybox.wgsl"));
const IMAGE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/image/skybox.png");

struct Camera {
    _projection_matrix: Mat4,
    _view_matrix: Mat4,
}
struct VertexInput {
    _position: [f32; 4],
    _tex_coord: [f32; 2],
}

const VERTEX_BUFFER_LAYOUTS: &[VertexBufferLayout] = &[VertexBufferLayout {
    array_stride: mem::size_of::<VertexInput>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: mem::size_of::<[f32; 4]>() as u64,
            shader_location: 1,
        },
    ],
}];
// WARN: Culling clockwise will make it disappear.
const VERTEX_DATA: &[VertexInput] = &[
    VertexInput {
        _position: [-50.0, 50.0, -50.0, 1.0],
        _tex_coord: [0.0, 0.0],
    }, // top: top left
    VertexInput {
        _position: [50.0, 50.0, -50.0, 1.0],
        _tex_coord: [0.0, 0.5],
    }, // top: top right
    VertexInput {
        _position: [50.0, 50.0, 50.0, 1.0],
        _tex_coord: [0.5, 0.5],
    }, // top: bottom right
    VertexInput {
        _position: [-50.0, 50.0, 50.0, 1.0],
        _tex_coord: [0.0, 0.5],
    }, // top: bottom left
    VertexInput {
        _position: [-50.0, 50.0, -50.0, 1.0],
        _tex_coord: [0.0, 0.5],
    }, // left: top left
    VertexInput {
        _position: [-50.0, 50.0, 50.0, 1.0],
        _tex_coord: [0.5, 0.5],
    }, // left: top right
    VertexInput {
        _position: [-50.0, -50.0, 50.0, 1.0],
        _tex_coord: [0.5, 1.0],
    }, // left: bottom right
    VertexInput {
        _position: [-50.0, -50.0, -50.0, 1.0],
        _tex_coord: [0.0, 1.0],
    }, // left: bottom left
    VertexInput {
        _position: [-50.0, 50.0, 50.0, 1.0],
        _tex_coord: [0.0, 0.5],
    }, // back: top left
    VertexInput {
        _position: [50.0, 50.0, 50.0, 1.0],
        _tex_coord: [0.5, 0.5],
    }, // back: top right
    VertexInput {
        _position: [50.0, -50.0, 50.0, 1.0],
        _tex_coord: [0.5, 1.0],
    }, // back: bottom right
    VertexInput {
        _position: [-50.0, -50.0, 50.0, 1.0],
        _tex_coord: [0.0, 1.0],
    }, // back: bottom left
    VertexInput {
        _position: [50.0, 50.0, 50.0, 1.0],
        _tex_coord: [0.0, 0.5],
    }, // right: top left
    VertexInput {
        _position: [50.0, 50.0, -50.0, 1.0],
        _tex_coord: [0.5, 0.5],
    }, // right: top right
    VertexInput {
        _position: [50.0, -50.0, -50.0, 1.0],
        _tex_coord: [0.5, 1.0],
    }, // right: bottom right
    VertexInput {
        _position: [50.0, -50.0, 50.0, 1.0],
        _tex_coord: [0.0, 1.0],
    }, // right: bottom left
    VertexInput {
        _position: [50.0, 50.0, -50.0, 1.0],
        _tex_coord: [0.0, 0.5],
    }, // front: top left
    VertexInput {
        _position: [-50.0, 50.0, -50.0, 1.0],
        _tex_coord: [0.5, 0.5],
    }, // front: top right
    VertexInput {
        _position: [-50.0, -50.0, -50.0, 1.0],
        _tex_coord: [0.5, 1.0],
    }, // front: bottom right
    VertexInput {
        _position: [50.0, -50.0, -50.0, 1.0],
        _tex_coord: [0.0, 1.0],
    }, // front: bottom left
    VertexInput {
        _position: [-50.0, -50.0, 50.0, 1.0],
        _tex_coord: [0.5, 0.5],
    }, // bottom: top left
    VertexInput {
        _position: [50.0, -50.0, 50.0, 1.0],
        _tex_coord: [1.0, 0.5],
    }, // bottom: top right
    VertexInput {
        _position: [50.0, -50.0, -50.0, 1.0],
        _tex_coord: [1.0, 1.0],
    }, // bottom: bottom right
    VertexInput {
        _position: [-50.0, -50.0, -50.0, 1.0],
        _tex_coord: [0.5, 1.0],
    }, // bottom: bottom left
];
const INDEX_DATA: &[u16] = &[
    0, 1, 2, 0, 2, 3, // top
    4, 5, 6, 4, 6, 7, // left
    8, 9, 10, 8, 10, 11, // back
    12, 13, 14, 12, 14, 15, // right
    16, 17, 18, 16, 18, 19, // front
    20, 21, 22, 20, 22, 23, // bottom
];
const CLEAR_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

fn create_depth_texture_view(device: &Device, width: u32, height: u32) -> TextureView {
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

/// A pipeline implementaion of world.wgsl.
pub(super) struct SkyboxPipeline {
    render_pipeline: RenderPipeline,
    depth_texture_view: TextureView,
    camera_buffer: Buffer,
    bind_group_0: BindGroup,
    model: Model,
}

impl SkyboxPipeline {
    pub(super) fn new(
        device: &Device,
        queue: &Queue,
        color_target_state: ColorTargetState,
        width: u32,
        height: u32,
    ) -> Self {
        // create a shader module
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::from(SHADER)),
        });

        // create a bind group layout, @group(0)
        let bind_group_0_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(mem::size_of::<Camera>() as u64),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // create a pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_0_layout],
            push_constant_ranges: &[],
        });

        // create a render pipeline
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: VERTEX_BUFFER_LAYOUTS,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(color_target_state)],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // create a depth texture view
        let depth_texture_view = create_depth_texture_view(device, width, height);

        // create a camera uniform buffer
        const CAMERA: Camera = Camera {
            _projection_matrix: Mat4::IDENTITY,
            _view_matrix: Mat4::IDENTITY,
        };
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::anything_to_u8slice(&CAMERA),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // create a skybox image texture
        let image_texture = match ImageTexture::new(device, queue, IMAGE_PATH) {
            Ok(n) => n,
            Err(e) => {
                error!(
                    "SkyboxPipeline.new",
                    "failed to create an image texture: {}: {}",
                    IMAGE_PATH,
                    e.to_string()
                );
                process::exit(1);
            }
        };

        // create a sampler
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        // create a bind group, @group(0)
        let bind_group_0 = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_0_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&image_texture.texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        // create a cube model
        let model = Model::from(device, VERTEX_DATA, INDEX_DATA);

        Self {
            render_pipeline,
            depth_texture_view,
            camera_buffer,
            bind_group_0,
            model,
        }
    }

    /// A method to draw a skybox.
    ///
    /// WARN: It clears render target texture and depth texture.
    pub(super) fn draw<'a>(
        &self,
        command_encoder: &'a mut CommandEncoder,
        render_target_view: &TextureView,
    ) {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(CLEAR_COLOR),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);
        render_pass.set_vertex_buffer(0, self.model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.model.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.model.index_count as u32, 0, 0..1);
    }

    pub(super) fn enqueue_update_camera(
        &self,
        queue: &Queue,
        camera_controller: &CameraController,
    ) {
        let camera = Camera {
            _projection_matrix: Mat4::perspective_lh(
                camera_controller.pov,
                camera_controller.width / camera_controller.height,
                0.1,
                1000.0,
            ),
            _view_matrix: Mat4::look_to_lh(
                Vec3::default(),
                camera_controller
                    .rotation
                    .mul_vec3(Vec3::new(0.0, 0.0, 1.0)),
                camera_controller
                    .rotation
                    .mul_vec3(Vec3::new(0.0, 1.0, 0.0)),
            ),
        };
        queue.write_buffer(&self.camera_buffer, 0, memory::anything_to_u8slice(&camera));
    }

    pub(super) fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.depth_texture_view = create_depth_texture_view(device, width, height);
    }
}
