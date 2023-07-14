@group(0) @binding(0)
var gui_textures: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var sampler_textures: binding_array<sampler>;

struct FrameUniform {
  view_transform: mat4x4<f32> 
}

@group(0) @binding(2)
var<uniform> u_frame: FrameUniform;

struct TextureUniform {
  transform: mat2x2<f32>,
  uv_transform: mat2x2<f32>
}

//@group(0) @binding(4)
//var<uniform> u_object: TextureUniform;

struct VertexOutput {
    @builtin(position) vertex: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vert
fn vs_main(
  @location(0) position: vec2<f32>,
  @location(1) uv: vec2<f32>
) -> VertexOutput {
  // vec2<f32> pos = u_frame.view_transform * position;
  // vec2<f32> final_uv = u_object.uv_transform * uv;
   
   vec4<f32> pos = u_frame.view_transform * vec4<f32>(pos.x, pos.y, 0, 1);
   var result: VertexOutput;
   result.vertex = pos;
   result.uv = uv;   
   return result;
}

@frag
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
