[[location(0)]]
var<in> in_vertex_index: vec3<f32>;
[[builtin(position)]]
var<out> out_pos: vec4<f32>;

[[stage(vertex)]]
fn vs_main() {
  out_pos = vec4<f32>(in_vertex_index, 1.0);
}

[[location(0)]]
var<out> out_color: vec4<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
