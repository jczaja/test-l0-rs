#!/bin/bash
clang -cc1 -triple spir ./src/matrixMultiply.cl -O2 -finclude-default-header -emit-llvm-bc -o matrixMultiply.bc 
llvm-spirv matrixMultiply.bc -o matrixMultiply.spv
