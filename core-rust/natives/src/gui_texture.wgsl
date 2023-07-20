@group(0) @binding(0)
var default_sampler: sampler;
@group(0) @binding(1)
var tile_sampler: sampler;

struct PushConstants {
    texture_style: u32,
}
var<push_constant> pc: PushConstants;

struct FrameUniform {
  view_transform: mat4x4<f32> 
}

@group(1) @binding(0)
var<uniform> u_frame: FrameUniform;
@group(1) @binding(1)
var u_tex: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) vertex: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
  @location(0) position: vec2<f32>,
  @location(1) uv: vec2<f32>
) -> VertexOutput {
   let pos: vec4<f32> = u_frame.view_transform * vec4<f32>(position.x, position.y, 0.0, 1.0);
   var result: VertexOutput;
   result.vertex = pos;
   result.uv = uv;
   return result;
}


@fragment
fn fs_main(
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    return textureSample(u_tex, default_sampler, uv.xy);
}
