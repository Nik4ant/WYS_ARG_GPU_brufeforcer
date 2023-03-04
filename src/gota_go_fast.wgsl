// NOTE: wgpu library example: https://github.com/gfx-rs/wgpu/tree/master/wgpu/examples/hello-compute
// NOTE: WGSL specs: https://www.w3.org/TR/WGSL/#composite-types
// NOTE: Official tutorial: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#wgsl
// Arrays: https://www.w3.org/TR/WGSL/#array-types
// NOTE: Is this a thing to create an array of arrays? https://www.w3.org/TR/WGSL/#composite-types

const ALIGNED_DATA_SIZE: u32 = 512u;
const ACTUAL_DATA_SIZE: u32 = 511u;
const KEY_LENGTH: u32 = 7u;

@group(0)
@binding(0)
var<storage, read_write> output_buffer: array<u32, ALIGNED_DATA_SIZE>;

@group(0)
@binding(1)
var<storage, read> input_keys: array<array<u32, KEY_LENGTH>>;

@group(1)
@binding(0)
var<uniform> data: array<vec4<u32>, 128>;

fn l2a() {
    /*
    data_removed_count = [current iterations count]
    printable_data_index = data_index + data_removed_count
    key_index = [current iteration count] % [key length]
    */
    let expected_output_size = get_correct_chars_count();

    var data_index: u32 = 0u;
    var i: u32 = ACTUAL_DATA_SIZE;
    loop {
        if (i < ACTUAL_DATA_SIZE - expected_output_size) {
            break;
        }
        // Dynamic index works only with storage values and stuff like this
        data_index = (data_index + input_keys[0][(ACTUAL_DATA_SIZE - i) % KEY_LENGTH]) % i;
        // Data is an array of vec4 aligned to 16 <-- Why? No idea. Is it required? Yes
        output_buffer[ACTUAL_DATA_SIZE - i] = data[(data_index + (ACTUAL_DATA_SIZE - i)) / 4u][(data_index + (ACTUAL_DATA_SIZE - i)) % 4u];

        i -= 1u;
    }
}

fn get_correct_chars_count() -> u32 {
	// TODO: implement __noop__'s idea on GPU side
    return 25u;
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // NOTE: This is too long to explaion, just don't mind this for now, ok?
    // I'll polish and explain everything later
    l2a();
}