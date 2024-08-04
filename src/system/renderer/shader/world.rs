use crate::{
    system::renderer::{
        model::{self, Model, ModelId},
        texture::depth,
    },
    util::{camera::CameraController, instance::InstanceController, memory, vector},
};
use glam::{Mat4, Vec3, Vec4};
use std::{borrow::Cow, cmp, collections::HashMap, mem};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages,
    ColorTargetState, CommandEncoder, Device, FragmentState, IndexFormat, LoadOp, MultisampleState,
    Operations, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp,
    TextureView, VertexState,
};

const SHADER: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/world.wgsl"));

struct Camera {
    _projection_matrix: Mat4,
    _view_matrix: Mat4,
}
struct Light {
    _direction: Vec4,
}
#[derive(Clone)]
struct Instance {
    _model_matrix: Mat4,
    _model_matrix_inversed: Mat4,
}
const MAX_INSTANCE_COUNT: u64 = 16;

/// A struct for descripting the detail of a draw request on an world pipeline.
pub struct DrawWorldDescriptor {
    /// Specify the model id to attach and the start and end index of instances.
    /// To reduce draw calls, you should group the same models together whenever possible.
    pub instance_indices: Vec<(ModelId, u32, u32)>,
}

/// A pipeline implementaion of world.wgsl.
pub struct WorldPipeline {
    render_pipeline: RenderPipeline,
    depth_texture_view: TextureView,
    camera_buffer: Buffer,
    instance_buffer: Buffer,
    bind_group_0: BindGroup,
}

impl WorldPipeline {
    /// A constructor.
    pub fn new(
        device: &Device,
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
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(mem::size_of::<Light>() as u64),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            mem::size_of::<Instance>() as u64 * MAX_INSTANCE_COUNT,
                        ),
                    },
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
                buffers: model::VERTEX_BUFFER_LAYOUTS,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(color_target_state)],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(depth::DEPTH_STENCIL_STATE),
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // create a depth texture view
        let depth_texture_view = depth::create_depth_texture_view(device, width, height);

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

        // create a light uniform buffer
        let light = Vec3::new(2.0, -10.0, 5.0).normalize();
        let light = Light {
            _direction: Vec4::new(light.x, light.y, light.z, 1.0),
        };
        let light_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::anything_to_u8slice(&light),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // create a instance uniform buffer
        let instances = (0..MAX_INSTANCE_COUNT)
            .into_iter()
            .map(|_| Instance {
                _model_matrix: Mat4::IDENTITY,
                _model_matrix_inversed: Mat4::IDENTITY,
            })
            .collect::<Vec<Instance>>();
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::slice_to_u8slice(instances.as_slice()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
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
                    resource: light_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: instance_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            render_pipeline,
            depth_texture_view,
            camera_buffer,
            instance_buffer,
            bind_group_0,
        }
    }

    /// A method to update instance buffer.
    ///
    /// WARN: Indices exceeding `MAX_INSTANCES_COUNT` will be completely ignored.
    pub fn update_instances(
        &self,
        queue: &Queue,
        instance_controllers: Vec<Option<InstanceController>>,
    ) {
        let instances_group = vector::get_consecutive_somes(
            vector::get_upto(&instance_controllers, MAX_INSTANCE_COUNT as usize),
            |n| {
                let model_matrix =
                    Mat4::from_scale_rotation_translation(n.scale, n.rotation, n.position);
                Instance {
                    _model_matrix: model_matrix,
                    _model_matrix_inversed: model_matrix.inverse().transpose(),
                }
            },
        );
        for (i, n) in instances_group {
            queue.write_buffer(
                &self.instance_buffer,
                mem::size_of::<Instance>() as u64 * i as u64,
                memory::slice_to_u8slice(n.as_slice()),
            );
        }
    }

    /// A method to draw models.
    ///
    /// WARN: Indices exceeding `MAX_INSTANCES_COUNT` will be completely ignored.
    pub fn draw<'a>(
        &self,
        command_encoder: &'a mut CommandEncoder,
        render_target_view: &TextureView,
        models: &HashMap<ModelId, Model>,
        descriptor: DrawWorldDescriptor,
    ) {
        // begin render pass
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
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

        // prepare
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);

        // draw
        for (name, start, end) in descriptor.instance_indices {
            if start >= MAX_INSTANCE_COUNT as u32 {
                continue;
            }
            let end = cmp::min(MAX_INSTANCE_COUNT as u32, end);
            let model = &models[&name];
            render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..model.index_count as u32, 0, start..end);
        }
    }

    /// A method to update the camera uniform buffer.
    ///
    /// It enqueues a `write_buffer` queue to `queue`.
    pub fn update_camera(&self, queue: &Queue, camera_controller: &CameraController) {
        let camera = Camera {
            _projection_matrix: Mat4::perspective_lh(
                camera_controller.pov,
                camera_controller.width / camera_controller.height,
                0.1,
                1000.0,
            ),
            _view_matrix: Mat4::look_to_lh(
                camera_controller.position,
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

    /// A method to recreate and resize the depth texture.
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.depth_texture_view = depth::create_depth_texture_view(device, width, height);
    }
}
