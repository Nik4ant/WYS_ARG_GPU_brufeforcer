// NOTE: wgpu library example: https://github.com/gfx-rs/wgpu/tree/master/wgpu/examples/hello-compute
// NOTE: WGSL specs: https://www.w3.org/TR/WGSL/#composite-types
// NOTE: Official tutorial: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#wgsl
// Arrays: https://www.w3.org/TR/WGSL/#array-types
// NOTE: Is this a thing to create an array of arrays? https://www.w3.org/TR/WGSL/#composite-types


@group(0)
@binding(0)
// FIXME: Can I just change this to a runtime-sized array?
// TODO: readonly
var<storage, read_write> input_keys: array<array<u32, 7>>;

@group(1)
@binding(0)
// FIXME: Can I just change this to a runtime-sized array?
// TODO: readonly
var<storage, read_write> output_buffer: array<u32, 50>;

fn l2a(key: array<u32, 7>) -> u32{
    return 420u;
}

// Thanks __noop__ :)
fn l2a_chars_count() {
	// TODO:
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
	// FIXME: fuuuuuuuuuuuuck....
    output_buffer[global_id.x] = l2a(input_keys[global_id.x]);
}