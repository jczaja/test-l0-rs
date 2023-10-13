__kernel void mxm(__global float* a, __global float* b, __global float *c, const int n) {
	uint idx = get_global_id(0);
	uint jdx = get_global_id(1);

  int n2 = get_global_size(0);

	int sum = 0;
	for (int k = 0; k < n; k++) {
		sum += a[idx * n + k] * b[k * n + jdx];
	}

	c[idx * n + jdx] = sum;
}

__kernel void reduce_sum(__global float* src, __global float* dst, int size) {

//  int n = get_global_size(0);


   dst[0] = 1000.0; // nie dziala
//  dst[idx] = 1000.0; // dziala

//int xsize = get_global_size(0);
//int idx = get_global_id(0);
//int idy = get_global_id(1);

//int flatId = idx * xsize + idy;

//dst[flatId] = flatId +1;

size = 1000;



}
