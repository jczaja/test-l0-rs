#!/bin/bash
set +x
clang -cc1 -triple spir ./shader/src/matrixMultiply.cl -O2 -finclude-default-header -emit-llvm-bc -o matrixMultiply.bc 
llvm-spirv matrixMultiply.bc -o matrixMultiply.spv
