struct Camera {
    projection_matrix: mat4x4<f32>,
}
@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Instance {
    model_matrix: mat4x4<f32>,
}
@group(0)
@binding(1)
var<uniform> instances: array<Instance, 16>;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) tex_coord: vec2<f32>,
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(instance_index) instance_index: u32,
    vertex_input: VertexInput,
) -> VertexOutput {
    var result: VertexOutput;

    result.position =
        camera.projection_matrix
        * instances[instance_index].model_matrix
        * vertex_input.position;

    result.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);

    return result;
}

@fragment
fn fs_main(vertex_outout: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_outout.color;
}
