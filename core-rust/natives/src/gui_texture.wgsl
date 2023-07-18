@group(0) @binding(0)
var default_sampler: sampler;
@group(0) @binding(1)
var tile_sampler: sampler;

struct FrameUniform {
  view_transform: mat4x4<f32> 
}

@group(1) @binding(0)
var<uniform> u_frame: FrameUniform;
@group(1) @binding(1)
var gui_textures: binding_array<texture_2d<f32>, 32>;

struct VertexOutput {
    @builtin(position) vertex: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tex_config: vec2<u32>,
};

@vertex
fn vs_main(
  @location(0) position: vec2<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) tex_config: vec2<u32>
) -> VertexOutput {

   let pos: vec4<f32> = u_frame.view_transform * vec4<f32>(position.x, position.y, 0.0, 1.0);
   var result: VertexOutput;
   result.vertex = pos;
   result.uv = uv;
   result.tex_config = tex_config;
   return result;
}


@fragment
fn fs_main(
    @location(0) uv: vec2<f32>,
    @location(1) tex_config: vec2<u32>
) -> @location(0) vec4<f32> {
    return textureSample(gui_textures[tex_config[0]], default_sampler, uv.xy);
}
