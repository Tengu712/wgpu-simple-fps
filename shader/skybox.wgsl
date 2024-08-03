struct Camera {
    projection_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
}
@group(0)
@binding(0)
var<uniform> camera: Camera;

@group(0)
@binding(1)
var image_texture: texture_2d<f32>;

@group(0)
@binding(2)
var image_sampler: sampler;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) tex_coord: vec2<f32>,
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var result: VertexOutput;

    result.position =
        camera.projection_matrix
        * camera.view_matrix
        * vertex_input.position;

    result.tex_coord = vertex_input.tex_coord;

    return result;
}

@fragment
fn fs_main(vertex_outout: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(image_texture, image_sampler, vertex_outout.tex_coord);
}
