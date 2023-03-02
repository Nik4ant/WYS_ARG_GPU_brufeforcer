// NOTE: wgpu library example: https://github.com/gfx-rs/wgpu/tree/master/wgpu/examples/hello-compute
// NOTE: WGSL specs: https://www.w3.org/TR/WGSL/#composite-types
// NOTE: Official tutorial: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#wgsl
// Arrays: https://www.w3.org/TR/WGSL/#array-types
// NOTE: Is this a thing to create an array of arrays? https://www.w3.org/TR/WGSL/#composite-types

@group(0)
@binding(0)
var<storage, read_write> output_buffer: array<u32, 512>;

@group(0)
@binding(1)
var<storage, read> input_keys: array<array<u32, 7>>;

@group(1)
@binding(0)

var<uniform> data: array<vec4<u32>, 128>;

fn l2a() -> u32 {
    var data_index: u32 = 0u;
	var printable_data_index: u32 = 0u;
    var data_removed_count: u32 = 0u;

    var key_index: u32 = 0u;
    var i: u32 = 511u;

    loop {
        if (i < 511u - 25u) {
            break;
        }

        data_index = (data_index + input_keys[0][key_index]) % i;
        printable_data_index = data_index + data_removed_count;
        key_index = (key_index + 1u) % 7u;

        output_buffer[511u - i] = data[printable_data_index / 4u][printable_data_index % 4u];
        data_removed_count += 1u;

        i -= 1u;
    }

    return 42u;
}

fn l2a_chars_count() {
	// TODO: implement __noop__'s idea on GPU side
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // NOTE: This is too long to explaion, just don't mind this for now, ok?
    // I'll polish and explain everything later
    l2a();
}