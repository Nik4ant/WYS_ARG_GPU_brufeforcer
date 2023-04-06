#include "cuda_runtime.h"
#include "device_launch_parameters.h"

#include <stdio.h>
#include <stdlib.h>
#include <cmath>

typedef unsigned char U8;
typedef unsigned short U16;
typedef unsigned int U32;
typedef unsigned long long int U64;

// TODO: lower threads count could perform better overall? Test later
const int DEFAULT_THREADS_COUNT = 256;  // 1024 <-- limit (can be changed, but that's probably a bad idea)
const int DEFAULT_BLOCKS_COUNT = 1024;  // 4096 * 256 <-- stable. AMOUNT OF MEMORY IS THE ONLY LIMIT

// Bruteforcer params
constexpr auto MAX_SKIP = 26;
constexpr auto L2A_SAFE_CHARS_COUNT = 14;  // 14 for DATA5 using key of length 27 with maximum skips
constexpr auto ITERATIONS_PER_INVOCATION = 32;
constexpr auto KEY_LEN = 7;
constexpr auto PASSING_SCORE = 1;  // used with >
// DATA params
constexpr auto ACTUAL_DATA_SIZE = 511;
constexpr auto ALIGNED_DATA_LEN = 512;

const __constant__ U8 DATA4[ALIGNED_DATA_LEN] = { 73, 118, 84, 117, 105, 116, 110, 32, 116, 97, 116, 32, 32, 101, 72, 101, 110, 111, 69, 105, 32, 112, 103, 118, 105, 32, 73, 65, 84, 102, 32, 110, 65, 79, 105, 77, 32, 112, 76, 108, 32, 115, 115, 121, 71, 110, 101, 108, 110, 117, 32, 108, 40, 69, 101, 111, 32, 115, 101, 73, 116, 104, 72, 78, 99, 59, 89, 65, 82, 115, 76, 32, 111, 101, 101, 111, 69, 105, 108, 110, 66, 115, 116, 40, 32, 101, 108, 116, 104, 32, 100, 121, 108, 97, 114, 120, 116, 108, 83, 65, 80, 97, 32, 89, 98, 83, 112, 78, 82, 39, 85, 114, 115, 108, 67, 80, 116, 104, 59, 66, 32, 76, 105, 121, 82, 79, 115, 111, 77, 112, 115, 70, 32, 83, 114, 97, 115, 67, 73, 115, 115, 110, 32, 75, 78, 69, 65, 76, 104, 65, 84, 105, 104, 32, 111, 32, 32, 97, 116, 111, 119, 114, 115, 116, 122, 67, 110, 105, 115, 117, 104, 99, 65, 72, 75, 65, 109, 32, 32, 101, 99, 73, 82, 69, 102, 116, 78, 115, 85, 77, 105, 78, 110, 97, 110, 32, 72, 101, 111, 115, 105, 114, 104, 72, 83, 115, 59, 65, 100, 116, 117, 117, 78, 112, 111, 100, 112, 69, 111, 59, 112, 101, 121, 32, 114, 54, 84, 32, 41, 114, 115, 40, 105, 115, 103, 32, 116, 121, 65, 105, 59, 115, 115, 101, 59, 116, 70, 101, 67, 116, 117, 116, 69, 106, 32, 83, 111, 101, 119, 117, 111, 101, 32, 116, 77, 78, 82, 32, 66, 32, 83, 101, 100, 99, 108, 32, 67, 32, 75, 120, 101, 101, 67, 79, 120, 97, 105, 59, 32, 116, 110, 105, 116, 76, 108, 76, 121, 32, 116, 101, 116, 110, 99, 85, 32, 79, 79, 32, 105, 106, 119, 69, 105, 67, 58, 98, 55, 79, 59, 101, 85, 105, 116, 104, 120, 32, 86, 79, 111, 100, 65, 114, 32, 41, 84, 117, 32, 120, 98, 115, 99, 58, 111, 110, 68, 32, 78, 110, 32, 116, 112, 115, 111, 116, 32, 75, 83, 101, 111, 59, 97, 101, 41, 68, 77, 69, 119, 50, 122, 32, 116, 102, 32, 69, 121, 102, 111, 116, 100, 97, 110, 114, 65, 101, 101, 116, 119, 117, 55, 100, 73, 101, 66, 32, 59, 109, 110, 104, 58, 98, 65, 32, 115, 108, 117, 32, 116, 32, 87, 76, 32, 107, 101, 116, 65, 116, 108, 68, 110, 107, 59, 114, 78, 108, 85, 32, 73, 76, 87, 101, 87, 55, 32, 105, 105, 68, 119, 85, 108, 110, 114, 84, 101, 69, 105, 111, 108, 116, 116, 100, 89, 84, 32, 79, 116, 119, 69, 101, 68, 98, 85, 101, 32, 105, 32, 104, 32, 72, 78, 102, 114, 65, 102, 99, 32, 100, 86, 81, 100, 107, 32, 65, 114, 122, 84, 79, 76, 97, 114, 65, 32, 105, 102, 32, 107, 121, 101, 59, 32, 116, 78, 32, 115, 84, 76, 121, 115, 32, 111, 84, 100, 0};
const __constant__ U8 INITIAL_KEY[KEY_LEN] = { 24, 4, 25, 15, 25, 15, 25 };
// TODO: generate later at compile time by using KEY_LEN param
const __constant__ U8 KEY_MODULO_LOOKUP_TABLE[L2A_SAFE_CHARS_COUNT] = {
    0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6
};
// TODO: generate later at compile time by using ITERATIONS_PER_INVOCATION param
const __constant__ U32 BITS_SHIFTING_LOOKUP_TABLE[ITERATIONS_PER_INVOCATION] = {
    1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7, 1 << 8, 1 << 9, 1 << 10, 1 << 11, 
    1 << 12, 1 << 13, 1 << 14, 1 << 15, 1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20, 1 << 21, 1 << 22, 
    1 << 23, 1 << 24, 1 << 25, 1 << 26, 1 << 27, 1 << 28, 1 << 29, 1 << 30, 1 << 31
};

__global__ void l2aKernel(U32 *outputBuffer) {
    int invocation_id = blockIdx.x * blockDim.x + threadIdx.x;
    U32 result = 0;

    U8 current_score;
    int data_index;
    int previous_printable_data_index;

    #pragma unroll (ITERATIONS_PER_INVOCATION)
    for (int j = 0; j < ITERATIONS_PER_INVOCATION; j++) {
        current_score = 0;
        data_index = INITIAL_KEY[0];
        previous_printable_data_index = data_index;

        /*
        data_removed_count = [current iterations count]
        printable_data_index = data_index + data_removed_count
        key_index = [current iteration count] % [key length]
        */
        #pragma unroll
        for (int i = ACTUAL_DATA_SIZE - 1; i > ACTUAL_DATA_SIZE - L2A_SAFE_CHARS_COUNT; i--) {
            data_index = (data_index + INITIAL_KEY[KEY_MODULO_LOOKUP_TABLE[(ACTUAL_DATA_SIZE - i)]]) % i;
            current_score += (DATA4[previous_printable_data_index] > 96) & (DATA4[data_index + ACTUAL_DATA_SIZE - i] == 32);
            previous_printable_data_index = data_index + ACTUAL_DATA_SIZE - i;
        }
        result |= (current_score > PASSING_SCORE) * BITS_SHIFTING_LOOKUP_TABLE[j];
    }

    outputBuffer[invocation_id] = result;
}


// Handles error in Debug mode, but ignores them in Release
inline cudaError_t cudaUnwrap(cudaError_t result) {
    #if defined(DEBUG) || defined(_DEBUG)
        if (result != cudaSuccess) {
            fprintf(stderr, "CUDA Runtime Error: %sn\n", cudaGetErrorString(result));
            exit(1);
        }
    #endif
    return result;
}

// Helper function for calling kernel
void l2aWithCuda(U32 *outputBuffer, unsigned long threads, unsigned long blocks) {
    // Allocate initial key array
    // cudaUnwrap(cudaMalloc((void**)&initialKeyDevicePtr, KEY_LEN * sizeof(U8)));
    // cudaUnwrap(cudaMemcpy(initialKeyDevicePtr, initialKey, KEY_LEN * sizeof(U8), cudaMemcpyHostToDevice));
    // Launch kernel
    l2aKernel<<<blocks, threads>>>(outputBuffer);
    // Check for any errors after launch
    cudaError_t cudaStatus = cudaGetLastError();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "l2aKernel launch failed: %s\n", cudaGetErrorString(cudaStatus));
        // free(initialKeyDevicePtr);
        exit(1);
    }
    // cudaDeviceSynchronize waits for the kernel to finish, and returns
    // any errors encountered during the launch.
    cudaUnwrap(cudaDeviceSynchronize());
}

//
//
//
//
// NOTE: THIS GUY IS GENIUS; USE HIS TIPS: http://impact.crhc.illinois.edu/shared/report/phd-thesis-shane-ryoo.pdf
// TODO: open intal-D music in the backround at the beggining xD
//
//
//
int main(int argc, char **argv) {
    unsigned long threads_count = DEFAULT_THREADS_COUNT;
    unsigned long blocks_count = DEFAULT_BLOCKS_COUNT;
    if (argc == 3) {
        // argv[0] is executable path (not sure if it's always the case)
        printf("Detected 2 args; `%s` threads and `%s` blocks\n", argv[1], argv[2]);
        // There is no such thing as safety, only hope...
        threads_count = strtoul(argv[1], &argv[1], 10);
        blocks_count = strtoul(argv[2], &argv[2], 10);
    } else {
        printf("No args for threads count and blocks count were detected; Using default values...\n");
    }

    U32 output_buffer_len = threads_count * blocks_count;

    // Select GPU
    cudaUnwrap(cudaSetDevice(0));

    // Allocating output buffer
    U32* outputBufferHost;
    // TODO: benchmark different variants
    cudaUnwrap(cudaMallocManaged((void**)&outputBufferHost, threads_count * blocks_count * sizeof(U32)));
    // TODO: async prefetching + double buffering
    #pragma unroll
    for (U32 i = 0; i < output_buffer_len; i++) {
        outputBufferHost[i] = i;
    }
    
    // FIXME: THIS NUMBER IS WRONG; smth wrong with the types
    long double cudaCallsCount = ceil(powl(MAX_SKIP, KEY_LEN) / (output_buffer_len * ITERATIONS_PER_INVOCATION));
    printf("Starting bruteforcer, pray to all gods that you know...\n");
    // printf("(NOT WORKING )Expected amount of calls to cuda kernel = %Lf\n", cudaCallsCount);

    // Messuaring execution time using cuda events
    // (See: https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/#using-cuda-gpu-timers)
    float exectuionTimeMs = 0.0;
    cudaEvent_t executionStart, executionEnd;
    cudaUnwrap(cudaEventCreate(&executionStart));
    cudaUnwrap(cudaEventCreate(&executionEnd));
    cudaUnwrap(cudaEventRecord(executionStart, 0));

    // TODO: manual unrolling? (this tag doesn't seem to work here)
    #pragma unroll
    for (int cudaCallIndex = 0; cudaCallIndex < 1; cudaCallIndex++) {
        l2aWithCuda(outputBufferHost, threads_count, blocks_count);
        // Step 2. Check keys that produce something that doesn't classified as giberish
        #pragma unroll
        for (U32 i = 0; i < output_buffer_len; i++) {
            if (outputBufferHost[i] != 0) {
                // TODO: find a way to turn that into a switch statement (since there are only 32 bits)
                for (int bitPosition = 0; bitPosition < 32; bitPosition++) {
                    if ((outputBufferHost[i] >> bitPosition) & 1) {
                        // printf("Found a key: TODO:");
                    }
                }
                // printf("\n");
                break;
            }
        }
    }
    cudaUnwrap(cudaEventRecord(executionEnd, 0));
    // Execution time
    cudaUnwrap(cudaEventSynchronize(executionEnd));
    cudaUnwrap(cudaEventElapsedTime(&exectuionTimeMs, executionStart, executionEnd));
    printf("Elapsed: %fms\n", exectuionTimeMs);

    cudaUnwrap(cudaEventDestroy(executionStart));
    cudaUnwrap(cudaEventDestroy(executionEnd));
    
    // LEVEL 4 OUTPUT: ifo llszu uyjx just kiddi
    for (U32 i = output_buffer_len - 1; i < output_buffer_len; i++) {
        printf("%u ", outputBufferHost[i]);
    }
    cudaUnwrap(cudaFree(outputBufferHost));

    // cudaDeviceReset must be called before exiting in order for profiling and
    // tracing tools such as Nsight and Visual Profiler to show complete traces.
    cudaUnwrap(cudaDeviceReset());
    return 0;
}
