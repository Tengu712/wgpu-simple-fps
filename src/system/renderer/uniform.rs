use glam::{Mat4, Vec3};
use std::mem;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages, Device,
    ShaderStages,
};

fn anything_to_u8slice<T>(a: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((a as *const T).cast::<u8>(), mem::size_of::<T>()) }
}

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
    pub(super) fn new(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        width: f32,
        height: f32,
    ) -> Self {
        // create a camera uniform buffer
        let camera = Camera {
            _projection_matrix: Mat4::perspective_rh(
                45.0f32.to_radians(),
                width / height,
                0.0,
                1000.0,
            ),
            _view_matrix: Mat4::look_to_rh(
                Vec3::new(0.0, 0.0, -10.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            _world_matrix: Mat4::IDENTITY,
        };
        let camera_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: anything_to_u8slice(&camera),
            usage: BufferUsages::UNIFORM,
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
}
