/*
Let's do some math:

THREADS_COUNT = 1024  // 1024 is limit on my GPU (can be higher?)
BLOCK_COUNT = 220  // amount of memory is the only limit here...

Each block has THREADS_COUNT threads.
One kernel (GPU device) invocation has BLOCK_COUNT * THREADS_COUNT parallel workers

NOTE: BLOCK_COUNT can be higher because right now my garbage code isn't super efficient, 
so I'm running out of memory. 


*/

#include "cuda_runtime.h"
#include "device_launch_parameters.h"

#include <stdio.h>

// FUTURE TODO: Do all sorts of crazy optimazations (__restrict__, branchless and other crazy stuff)
// TODO: explain

typedef unsigned char U8;
typedef unsigned short U16;
typedef unsigned int U32;
typedef unsigned long long int U64;

const int THREADS_COUNT = 1024;  // 1024 <-- limit
const int BLOCK_COUNT = 220;  // 226 <-- UNSTABLE limit (not enough memory on the stack)

// TODO: replace some things with DEFINE?

// Bruteforcer params
constexpr auto MAX_SKIP = 26;
constexpr auto L2A_SAFE_CHARS_COUNT = 14;  // 14 for DATA5 using key of length 27 with maximum skips
constexpr auto ITERATIONS_PER_INVOCATION = 32;
constexpr auto KEY_LEN = 7;
constexpr auto PASSING_SCORE = 2;
// DATA params
constexpr auto ACTUAL_DATA_SIZE = 511;
constexpr auto ALIGNED_DATA_LEN = 512;
// TODO: test what happens without using aligned length...
const __constant__ U8 DATA4[ALIGNED_DATA_LEN] = { 73, 118, 84, 117, 105, 116, 110, 32, 116, 97, 116, 32, 32, 101, 72, 101, 110, 111, 69, 105, 32, 112, 103, 118, 105, 32, 73, 65, 84, 102, 32, 110, 65, 79, 105, 77, 32, 112, 76, 108, 32, 115, 115, 121, 71, 110, 101, 108, 110, 117, 32, 108, 40, 69, 101, 111, 32, 115, 101, 73, 116, 104, 72, 78, 99, 59, 89, 65, 82, 115, 76, 32, 111, 101, 101, 111, 69, 105, 108, 110, 66, 115, 116, 40, 32, 101, 108, 116, 104, 32, 100, 121, 108, 97, 114, 120, 116, 108, 83, 65, 80, 97, 32, 89, 98, 83, 112, 78, 82, 39, 85, 114, 115, 108, 67, 80, 116, 104, 59, 66, 32, 76, 105, 121, 82, 79, 115, 111, 77, 112, 115, 70, 32, 83, 114, 97, 115, 67, 73, 115, 115, 110, 32, 75, 78, 69, 65, 76, 104, 65, 84, 105, 104, 32, 111, 32, 32, 97, 116, 111, 119, 114, 115, 116, 122, 67, 110, 105, 115, 117, 104, 99, 65, 72, 75, 65, 109, 32, 32, 101, 99, 73, 82, 69, 102, 116, 78, 115, 85, 77, 105, 78, 110, 97, 110, 32, 72, 101, 111, 115, 105, 114, 104, 72, 83, 115, 59, 65, 100, 116, 117, 117, 78, 112, 111, 100, 112, 69, 111, 59, 112, 101, 121, 32, 114, 54, 84, 32, 41, 114, 115, 40, 105, 115, 103, 32, 116, 121, 65, 105, 59, 115, 115, 101, 59, 116, 70, 101, 67, 116, 117, 116, 69, 106, 32, 83, 111, 101, 119, 117, 111, 101, 32, 116, 77, 78, 82, 32, 66, 32, 83, 101, 100, 99, 108, 32, 67, 32, 75, 120, 101, 101, 67, 79, 120, 97, 105, 59, 32, 116, 110, 105, 116, 76, 108, 76, 121, 32, 116, 101, 116, 110, 99, 85, 32, 79, 79, 32, 105, 106, 119, 69, 105, 67, 58, 98, 55, 79, 59, 101, 85, 105, 116, 104, 120, 32, 86, 79, 111, 100, 65, 114, 32, 41, 84, 117, 32, 120, 98, 115, 99, 58, 111, 110, 68, 32, 78, 110, 32, 116, 112, 115, 111, 116, 32, 75, 83, 101, 111, 59, 97, 101, 41, 68, 77, 69, 119, 50, 122, 32, 116, 102, 32, 69, 121, 102, 111, 116, 100, 97, 110, 114, 65, 101, 101, 116, 119, 117, 55, 100, 73, 101, 66, 32, 59, 109, 110, 104, 58, 98, 65, 32, 115, 108, 117, 32, 116, 32, 87, 76, 32, 107, 101, 116, 65, 116, 108, 68, 110, 107, 59, 114, 78, 108, 85, 32, 73, 76, 87, 101, 87, 55, 32, 105, 105, 68, 119, 85, 108, 110, 114, 84, 101, 69, 105, 111, 108, 116, 116, 100, 89, 84, 32, 79, 116, 119, 69, 101, 68, 98, 85, 101, 32, 105, 32, 104, 32, 72, 78, 102, 114, 65, 102, 99, 32, 100, 86, 81, 100, 107, 32, 65, 114, 122, 84, 79, 76, 97, 114, 65, 32, 105, 102, 32, 107, 121, 101, 59, 32, 116, 78, 32, 115, 84, 76, 121, 115, 32, 111, 84, 100, 0};

__global__ void l2aKernel(U32* output_buffer, const U8* __restrict__ initial_key)
{
    auto invocation_id = blockIdx.x * blockDim.x + threadIdx.x;
    U32 result = 0;

    short current_score;
    int data_index;
    int previous_printable_data_index;

    // Works when iterations count is compile known value
    /*
    data_removed_count = [current iterations count]
    printable_data_index = data_index + data_removed_count
    key_index = [current iteration count] % [key length]
    */
    
    // WARNING: NEED TO IMPLEMENT PROPER KEY ITERATIONS AND SAFE l2a bounds
    // TODO: test manual unrolling to see if it does what intended
    #pragma unroll
    for (int j = 0; j < ITERATIONS_PER_INVOCATION; j++) {
        /*
        Formatting checks:
        59 = ';'
        32 = ' '
        1) TODO:
        
        */
        current_score = 0;
        // 1 step of the decryption
        data_index = initial_key[0];
        previous_printable_data_index = data_index;
        current_score -= (DATA4[data_index] == 32 | DATA4[data_index] == 59);
        // 2..Nth steps
        #pragma unroll
        for (int i = ACTUAL_DATA_SIZE - 1; i > ACTUAL_DATA_SIZE - L2A_SAFE_CHARS_COUNT; i--) {
            // debug only
            // output_buffer[ACTUAL_DATA_SIZE - i] = DATA4[data_index + ACTUAL_DATA_SIZE - i];

            data_index = (data_index + initial_key[(ACTUAL_DATA_SIZE - i) % KEY_LEN]) % i;
            // Characters before 97 aren't lowercase english laters
            current_score += ((DATA4[previous_printable_data_index] > 96) & (DATA4[data_index + ACTUAL_DATA_SIZE - i] == 32 | DATA4[data_index + ACTUAL_DATA_SIZE - i] == 59));
            previous_printable_data_index = data_index + ACTUAL_DATA_SIZE - i;
        }
        result |= (current_score >= PASSING_SCORE) * (1 << j);
    }

    output_buffer[invocation_id] = result;
}


cudaError_t l2aWithCuda(U32* output_buffer, const U8* __restrict__ initial_key, const size_t output_buffer_size);


int main() {
    const unsigned int OUTPUT_BUFFER_LEN = THREADS_COUNT * BLOCK_COUNT;
    const U8 INITIAL_KEY[KEY_LEN] = { 24, 4, 25, 15, 25, 15, 25 };

    U32 output_buffer[OUTPUT_BUFFER_LEN] = { 0 };
    
    // Messuaring execution time using cuda events
    // (See: https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/#using-cuda-gpu-timers)
    float exectuionTimeMs = 0.0;
    cudaEvent_t executionStart, executionEnd;
    cudaEventCreate(&executionStart);
    cudaEventCreate(&executionEnd);

    // Running CUDA
    cudaEventRecord(executionStart, 0);
    cudaError_t cudaStatus = l2aWithCuda(output_buffer, INITIAL_KEY, OUTPUT_BUFFER_LEN * sizeof(U32));
    cudaEventRecord(executionEnd, 0);
    // Execution time
    cudaEventSynchronize(executionEnd);
    cudaEventElapsedTime(&exectuionTimeMs, executionStart, executionEnd);
    printf("Elapsed: %f ms\n", exectuionTimeMs);
    cudaEventDestroy(executionStart);
    cudaEventDestroy(executionEnd);

    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "l2aWithCuda failed!\n");
        return 1;
    }
    
    // cudaDeviceReset must be called before exiting in order for profiling and
    // tracing tools such as Nsight and Visual Profiler to show complete traces.
    cudaStatus = cudaDeviceReset();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceReset failed!");
        return 1;
    }
    
    // LEVEL 4 OUTPUT: ifo llszu uyjx just kiddi
    for (int i = OUTPUT_BUFFER_LEN - 1; i < OUTPUT_BUFFER_LEN; i++) {
        printf("%u ", output_buffer[i]);
    }
    printf("\n");

    printf("\nDONE!\n");
    // printf("Press ENTER key to exit\n");
    // getchar();
    return 0;
}


// Helper function for using CUDA
cudaError_t l2aWithCuda(U32* output_buffer, const U8* __restrict__ initial_key, const size_t output_buffer_size)
{
    U8* device_ptr_key = 0;
    U32* device_ptr_output_buffer = 0;
    cudaError_t cudaStatus;

    // Select GPU
    cudaStatus = cudaSetDevice(0);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaSetDevice failed!  Do you have a CUDA-capable GPU installed?");
        goto Error;
    }

    // Allocating GPU buffers.
    // Output buffer
    cudaStatus = cudaMalloc((void**)&device_ptr_output_buffer, output_buffer_size);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed! (output buffer)");
        goto Error;
    }
    // Key buffer
    cudaStatus = cudaMalloc((void**)&device_ptr_key, KEY_LEN * sizeof(U8));
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed! (key data)");
        goto Error;
    }

    // Coping buffers from host (CPU) to GPU.
    // Key data
    cudaStatus = cudaMemcpy(device_ptr_key, initial_key, KEY_LEN * sizeof(U8), cudaMemcpyHostToDevice);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed! (key data)");
        goto Error;
    }

    // Launching kernel
    l2aKernel<<<BLOCK_COUNT, THREADS_COUNT>>>(device_ptr_output_buffer, device_ptr_key);

    // Check for any errors after launch
    cudaStatus = cudaGetLastError();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "addKernel launch failed: %s\n", cudaGetErrorString(cudaStatus));
        goto Error;
    }

    // cudaDeviceSynchronize waits for the kernel to finish, and returns
    // any errors encountered during the launch.
    cudaStatus = cudaDeviceSynchronize();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceSynchronize returned error code %d after launching addKernel!\n", cudaStatus);
        goto Error;
    }

    // Copy output from GPU to host (CPU)
    cudaStatus = cudaMemcpy(output_buffer, device_ptr_output_buffer, output_buffer_size, cudaMemcpyDeviceToHost);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed!");
        goto Error;
    }

Error:
    cudaFree(device_ptr_key);
    cudaFree(device_ptr_output_buffer);

    return cudaStatus;
}

