__global__ void gpu_kerr(float *buffer) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < 1024) {
        buffer[idx] = buffer[idx] * 2.0f;
    }
}