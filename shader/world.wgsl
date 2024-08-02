struct Camera {
    projection_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
}
@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Instance {
    model_matrix: mat4x4<f32>,
}
@group(0)
@binding(1)
var<uniform> instances: array<Instance, 4>;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
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
        * camera.view_matrix
        * instances[instance_index].model_matrix
        * vertex_input.position;

    let c = 1.0 - result.position.z / 40.0;
    result.color = vec4<f32>(c, c, c, 1.0);

    return result;
}

@fragment
fn fs_main(vertex_outout: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_outout.color;
}
