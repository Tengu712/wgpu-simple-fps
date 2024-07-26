struct Uniforms {
  projection_matrix: mat4x4<f32>,
  view_matrix: mat4x4<f32>,
  world_matrix: mat4x4<f32>,
}

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) position: vec4<f32>) -> @builtin(position) vec4<f32> {
    return uniforms.projection_matrix * uniforms.view_matrix * uniforms.world_matrix * position;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
