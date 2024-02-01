#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER


const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);

@group(1) @binding(0) var<uniform> material_color: vec4<f32>;
@group(1) @binding(1) var material_color_texture: texture_2d<f32>;
@group(1) @binding(2) var material_color_sampler: sampler;
@group(1) @binding(3) var<uniform> material_percentageszd: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let pixel = textureSample(material_color_texture, material_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
    if (mesh.uv.x * mesh.uv.y < material_percentageszd) {
        return pixel;
    }

    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}