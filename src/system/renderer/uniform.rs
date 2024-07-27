use crate::util::{camera::CameraController, memory};
use glam::Mat4;
use std::mem;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages, Device,
    Queue, ShaderStages,
};

struct Camera {
    _projection_matrix: Mat4,
    _view_matrix: Mat4,
    _world_matrix: Mat4,
}

/// A function to create the layout of the bind group, @group(0).
pub(super) fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(mem::size_of::<Camera>() as u64),
            },
            count: None,
        }],
    })
}

/// A struct for the bind group, @group(0).
pub(super) struct Group0 {
    pub(super) camera_uniform_buffer: Buffer,
    pub(super) bind_group: BindGroup,
}

impl Group0 {
    pub(super) fn new(device: &Device, bind_group_layout: &BindGroupLayout) -> Self {
        // create a camera uniform buffer
        const CAMERA: Camera = Camera {
            _projection_matrix: Mat4::IDENTITY,
            _view_matrix: Mat4::IDENTITY,
            _world_matrix: Mat4::IDENTITY,
        };
        let camera_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::anything_to_u8slice(&CAMERA),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // create a bind group
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        // finish
        Self {
            camera_uniform_buffer,
            bind_group,
        }
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
                0.0,
                1000.0,
            ),
            _view_matrix: Mat4::look_to_lh(
                camera_controller.position,
                camera_controller.direction,
                camera_controller.up,
            ),
            _world_matrix: Mat4::IDENTITY,
        };
        queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            memory::anything_to_u8slice(&camera),
        );
    }
}
