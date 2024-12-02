// The shader reads the previous frame's state from the `input` texture, and writes the new state of
// each pixel to the `output` texture. The textures are flipped each step to progress the
// simulation.
// Two textures are needed for the game of life as each pixel of step N depends on the state of its
// neighbors at step N-1.

@group(0) @binding(0) var<uniform> wrap: u32;

@group(0) @binding(1) var<uniform> layer_num: u32;

@group(0) @binding(2) var<storage, read> permutation: array<f32>;

@group(0) @binding(3) var<storage, read> layers: array<f32>;

@group(0) @binding(4) var<storage, write> vertices: array<f32>;

@compute @workgroup_size(8, 8, 8)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
}
