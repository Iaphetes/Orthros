struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    @builtin(position) coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>
) -> @location(0) vec4<f32> {
    var grid_width : f32 = 5.0;
    var grid_x : f32 = f32(i32(uv.x * 10000.0) % 100);
    var grid_y : f32 = f32(i32(uv.y * 10000.0) % 100);
    var green : f32 = 0.0;
    var alpha : f32 = 0.0;
    var grid_shift : f32 = grid_width / 2.0;
    var grid_scale : f32 = grid_shift * grid_shift;
    if grid_x < grid_width {
       green += (-(grid_x - grid_shift) * (grid_x - grid_shift) + grid_scale) / grid_scale * 1.0;
       alpha = 0.5;
    }
    if grid_y < grid_width {
       green += (-(grid_y - grid_shift) * (grid_y - grid_shift) + grid_scale) / grid_scale * 1.0;
       alpha = 0.5;
    }
    return vec4<f32>(green / 2.0, green / 2.0, green / 2.0, alpha) ;
}
