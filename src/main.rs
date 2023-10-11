#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ndarray::prelude::*;
use ndarray::Array;
use std::collections::HashMap;
use std::ffi::CString;
use std::iter::FromIterator;
use std::mem;

// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
// TODO: https://github.com/EmbarkStudios/rust-gpu/issues/550  (target _feature to test)
// TODO: make CI
// TODO: read tutorial and make more calls for educational purposes

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn make_descriptive_error_codes() -> HashMap<_ze_result_t, &'static str> {
    let mut error_codes: HashMap<_ze_result_t, &str> = HashMap::default();

    error_codes.insert(1879048193, "ZE_RESULT_ERROR_DEVICE_LOST");
    error_codes.insert(1879048194, "ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY");
    error_codes.insert(1879048195, "ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY");
    error_codes.insert(1879048196, "ZE_RESULT_ERROR_MODULE_BUILD_FAILURE");
    error_codes.insert(1879048197, "ZE_RESULT_ERROR_MODULE_LINK_FAILURE");
    error_codes.insert(1879048198, "ZE_RESULT_ERROR_DEVICE_REQUIRES_RESET");
    error_codes.insert(1879048199, "ZE_RESULT_ERROR_DEVICE_IN_LOW_POWER_STATE");
    error_codes.insert(2146435073, "ZE_RESULT_EXP_ERROR_DEVICE_IS_NOT_VERTEX");
    error_codes.insert(2146435074, "ZE_RESULT_EXP_ERROR_VERTEX_IS_NOT_DEVICE");
    error_codes.insert(2146435075, "ZE_RESULT_EXP_ERROR_REMOTE_DEVICE");
    error_codes.insert(1879113728, "ZE_RESULT_ERROR_INSUFFICIENT_PERMISSIONS");
    error_codes.insert(1879113729, "ZE_RESULT_ERROR_NOT_AVAILABLE");
    error_codes.insert(1879179264, "ZE_RESULT_ERROR_DEPENDENCY_UNAVAILABLE");
    error_codes.insert(1879179265, "ZE_RESULT_WARNING_DROPPED_DATA");
    error_codes.insert(2013265921, "ZE_RESULT_ERROR_UNINITIALIZED");
    error_codes.insert(2013265922, "ZE_RESULT_ERROR_UNSUPPORTED_VERSION");
    error_codes.insert(2013265923, "ZE_RESULT_ERROR_UNSUPPORTED_FEATURE");
    error_codes.insert(2013265924, "ZE_RESULT_ERROR_INVALID_ARGUMENT");
    error_codes.insert(2013265925, "ZE_RESULT_ERROR_INVALID_NULL_HANDLE");
    error_codes.insert(2013265926, "ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE");
    error_codes.insert(2013265927, "ZE_RESULT_ERROR_INVALID_NULL_POINTER");
    error_codes.insert(2013265928, "ZE_RESULT_ERROR_INVALID_SIZE");
    error_codes.insert(2013265929, "ZE_RESULT_ERROR_UNSUPPORTED_SIZE");
    error_codes.insert(2013265930, "ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT");
    error_codes.insert(2013265931, "ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT");
    error_codes.insert(2013265932, "ZE_RESULT_ERROR_INVALID_ENUMERATION");
    error_codes.insert(2013265933, "ZE_RESULT_ERROR_UNSUPPORTED_ENUMERATION");
    error_codes.insert(2013265934, "ZE_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT");
    error_codes.insert(2013265935, "ZE_RESULT_ERROR_INVALID_NATIVE_BINARY");
    error_codes.insert(2013265936, "ZE_RESULT_ERROR_INVALID_GLOBAL_NAME");
    error_codes.insert(2013265937, "ZE_RESULT_ERROR_INVALID_KERNEL_NAME");
    error_codes.insert(2013265938, "ZE_RESULT_ERROR_INVALID_FUNCTION_NAME");
    error_codes.insert(2013265939, "ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION");
    error_codes.insert(2013265940, "ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION");
    error_codes.insert(2013265941, "ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_INDEX");
    error_codes.insert(2013265942, "ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_SIZE");
    error_codes.insert(2013265943, "ZE_RESULT_ERROR_INVALID_KERNEL_ATTRIBUTE_VALUE");
    error_codes.insert(2013265944, "ZE_RESULT_ERROR_INVALID_MODULE_UNLINKED");
    error_codes.insert(2013265945, "ZE_RESULT_ERROR_INVALID_COMMAND_LIST_TYPE");
    error_codes.insert(2013265946, "ZE_RESULT_ERROR_OVERLAPPING_REGIONS");
    error_codes.insert(2013265947, "ZE_RESULT_WARNING_ACTION_REQUIRED");
    error_codes.insert(2147483646, "ZE_RESULT_ERROR_UNKNOWN");
    error_codes.insert(2147483647, "ZE_RESULT_FORCE_UINT32");
    error_codes.insert(0, "ZE_RESULT_SUCCESS");
    error_codes.insert(1, "ZE_RESULT_NOT_READY");
    error_codes.insert(1879048193, "ZE_RESULT_ERROR_DEVICE_LOST");
    error_codes.insert(1879048194, "ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY");
    error_codes.insert(1879048195, "ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY");
    error_codes.insert(1879048196, "ZE_RESULT_ERROR_MODULE_BUILD_FAILURE");
    error_codes.insert(1879048197, "ZE_RESULT_ERROR_MODULE_LINK_FAILURE");
    error_codes.insert(1879048198, "ZE_RESULT_ERROR_DEVICE_REQUIRES_RESET");
    error_codes.insert(1879048199, "ZE_RESULT_ERROR_DEVICE_IN_LOW_POWER_STATE");
    error_codes.insert(2146435073, "ZE_RESULT_EXP_ERROR_DEVICE_IS_NOT_VERTEX");
    error_codes.insert(2146435074, "ZE_RESULT_EXP_ERROR_VERTEX_IS_NOT_DEVICE");
    error_codes.insert(2146435075, "ZE_RESULT_EXP_ERROR_REMOTE_DEVICE");
    error_codes.insert(1879113728, "ZE_RESULT_ERROR_INSUFFICIENT_PERMISSIONS");
    error_codes.insert(1879113729, "ZE_RESULT_ERROR_NOT_AVAILABLE");
    error_codes.insert(1879179264, "ZE_RESULT_ERROR_DEPENDENCY_UNAVAILABLE");
    error_codes.insert(1879179265, "ZE_RESULT_WARNING_DROPPED_DATA");
    error_codes.insert(2013265921, "ZE_RESULT_ERROR_UNINITIALIZED");
    error_codes.insert(2013265922, "ZE_RESULT_ERROR_UNSUPPORTED_VERSION");
    error_codes.insert(2013265923, "ZE_RESULT_ERROR_UNSUPPORTED_FEATURE");
    error_codes.insert(2013265924, "ZE_RESULT_ERROR_INVALID_ARGUMENT");
    error_codes.insert(2013265925, "ZE_RESULT_ERROR_INVALID_NULL_HANDLE");
    error_codes.insert(2013265926, "ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE");
    error_codes.insert(2013265927, "ZE_RESULT_ERROR_INVALID_NULL_POINTER");
    error_codes.insert(2013265928, "ZE_RESULT_ERROR_INVALID_SIZE");
    error_codes.insert(2013265929, "ZE_RESULT_ERROR_UNSUPPORTED_SIZE");
    error_codes.insert(2013265930, "ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT");
    error_codes.insert(2013265931, "ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT");
    error_codes.insert(2013265932, "ZE_RESULT_ERROR_INVALID_ENUMERATION");
    error_codes.insert(2013265933, "ZE_RESULT_ERROR_UNSUPPORTED_ENUMERATION");
    error_codes.insert(2013265934, "ZE_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT");
    error_codes.insert(2013265935, "ZE_RESULT_ERROR_INVALID_NATIVE_BINARY");
    error_codes.insert(2013265936, "ZE_RESULT_ERROR_INVALID_GLOBAL_NAME");
    error_codes.insert(2013265937, "ZE_RESULT_ERROR_INVALID_KERNEL_NAME");
    error_codes.insert(2013265938, "ZE_RESULT_ERROR_INVALID_FUNCTION_NAME");
    error_codes.insert(2013265939, "ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION");
    error_codes.insert(2013265940, "ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION");
    error_codes.insert(2013265941, "ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_INDEX");
    error_codes.insert(2013265942, "ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_SIZE");
    error_codes.insert(2013265943, "ZE_RESULT_ERROR_INVALID_KERNEL_ATTRIBUTE_VALUE");
    error_codes.insert(2013265944, "ZE_RESULT_ERROR_INVALID_MODULE_UNLINKED");
    error_codes.insert(2013265945, "ZE_RESULT_ERROR_INVALID_COMMAND_LIST_TYPE");
    error_codes.insert(2013265946, "ZE_RESULT_ERROR_OVERLAPPING_REGIONS");
    error_codes.insert(2013265947, "ZE_RESULT_WARNING_ACTION_REQUIRED");
    error_codes.insert(2147483646, "ZE_RESULT_ERROR_UNKNOWN");
    error_codes.insert(2147483647, "ZE_RESULT_FORCE_UINT32");

    error_codes
}

fn make_descriptive_devices_types() -> HashMap<_ze_device_type_t, &'static str> {
    let mut devices_types: HashMap<_ze_device_type_t, &str> = [
        (1, "ZE_DEVICE_TYPE_GPU"),
        (2, "ZE_DEVICE_TYPE_CPU"),
        (3, "ZE_DEVICE_TYPE_FPGA"),
        (4, "ZE_DEVICE_TYPE_MCA"),
        (5, "ZE_DEVICE_TYPE_VPU"),
        (2147483647, "ZE_DEVICE_TYPE_FORCE_UINT32"),
    ]
    .iter()
    .cloned()
    .collect();

    devices_types
}

fn get_first_driver() -> Result<ze_driver_handle_t, &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let mut result;
    let mut pcount: u32 = 0;
    let mut phdrivers: [ze_driver_handle_t; 10]; // problem above 10 drivers
    unsafe {
        phdrivers = mem::zeroed();
        // Get number of drivers of Level zero
        result = zeDriverGet(&mut pcount, phdrivers.as_mut_ptr());
    }
    log::info!("zeDriverGet: {}", error_msgs[&result]);
    match (result, pcount) {
        (_ze_result_t_ZE_RESULT_SUCCESS, 0) => return Err("No Level Zero drivers!"),
        (_ze_result_t_ZE_RESULT_SUCCESS, _) => println!("Num Level Zero drivers {pcount}"),
        (_, _) => return Err("Error: zeDriverGet failed!"),
    }

    // Get actual drivers
    unsafe {
        result = zeDriverGet(&mut pcount, phdrivers.as_mut_ptr());
    }
    log::info!("zeDriverGet: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero drivers {:?}", phdrivers),
        _ => return Err("Error: zeDriverGet failed!"),
    }
    Ok(phdrivers[0])
}

fn get_first_device(driver: &ze_driver_handle_t) -> Result<ze_device_handle_t, &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let mut num_devices = 0;
    let mut phdevices: [ze_device_handle_t; 10];
    let mut result;
    unsafe {
        phdevices = mem::zeroed();
        result = zeDeviceGet(*driver, &mut num_devices, phdevices.as_mut_ptr());
    }
    log::info!("zeDeviceGet: {}", error_msgs[&result]);

    match (result, num_devices) {
        (_ze_result_t_ZE_RESULT_SUCCESS, 0) => return Err("No Level Zero devices!"),
        (_ze_result_t_ZE_RESULT_SUCCESS, _) => println!("Num Level Zero devices {num_devices}"),
        (_, _) => return Err("Error: zeDeviceGet failed!"),
    }

    unsafe {
        result = zeDeviceGet(*driver, &mut num_devices, phdevices.as_mut_ptr());
    }
    log::info!("zeDeviceGet: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero devices {:?}", phdevices),
        _ => return Err("Error: zeDeviceGet failed!"),
    }

    let mut pDeviceProperties: ze_device_properties_t;
    unsafe {
        pDeviceProperties = mem::zeroed();
        result = zeDeviceGetProperties(phdevices[0], &mut pDeviceProperties);
    }
    log::info!("zeDeviceGetProperties: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            let device_types = make_descriptive_devices_types();

            let iname = pDeviceProperties
                .name
                .iter()
                .map(|&x| x as u8)
                .collect::<Vec<u8>>();

            println!(
                "device name: {:?} ({})",
                std::ffi::CStr::from_bytes_until_nul(&iname[..])
                    .expect("Error converting Device Name"),
                device_types[&pDeviceProperties.type_]
            );
            log::info!(
                "    maxMemAllocSize[MiB]: {}",
                pDeviceProperties.maxMemAllocSize / 1024 / 1024
            );
            log::info!("    numThreadsPerEU: {}", pDeviceProperties.numThreadsPerEU);
            log::info!(
                "    physicalEUSimdWidth: {}",
                pDeviceProperties.physicalEUSimdWidth
            );
            log::info!(
                "    numEUsPerSubslice: {}",
                pDeviceProperties.numEUsPerSubslice
            );
            log::info!(
                "    numSubslicesPerSlice: {}",
                pDeviceProperties.numSubslicesPerSlice
            );
            log::info!("    numSlices: {}", pDeviceProperties.numSlices);
        }
        _ => return Err("Error: zeDeviceGetProperties failed!"),
    }

    // get compute specific properties of device

    let mut pComputeProperties: ze_device_compute_properties_t;
    unsafe {
        pComputeProperties = mem::zeroed();
        result = zeDeviceGetComputeProperties(phdevices[0], &mut pComputeProperties);
    }
    log::info!("zeDeviceGetComputeProperties: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!(
                "    maxTotalGroupSize: {}",
                pComputeProperties.maxTotalGroupSize
            );
            log::info!("    maxGroupSizeX: {}", pComputeProperties.maxGroupSizeX);
            log::info!("    maxGroupSizeY: {}", pComputeProperties.maxGroupSizeY);
            log::info!("    maxGroupSizeZ: {}", pComputeProperties.maxGroupSizeZ);
            log::info!("    maxGroupCountX: {}", pComputeProperties.maxGroupCountX);
            log::info!("    maxGroupCountY: {}", pComputeProperties.maxGroupCountY);
            log::info!("    maxGroupCountZ: {}", pComputeProperties.maxGroupCountZ);
            log::info!(
                "    maxSharedLocalMemory[KiB]: {}",
                pComputeProperties.maxSharedLocalMemory / 1024
            );
            log::info!(
                "    numSubGroupSizes: {}",
                pComputeProperties.numSubGroupSizes
            );
            log::info!("    subGroupSizes: {:?}", pComputeProperties.subGroupSizes);
        }
        _ => return Err("Error: zeDeviceGetComputeProperties failed!"),
    }

    Ok(phdevices[0])
}

fn get_context(driver: &ze_driver_handle_t) -> Result<ze_context_handle_t, &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let mut context: ze_context_handle_t;
    let result;
    unsafe {
        context = mem::zeroed();
        let mut contextDescription: ze_context_desc_t = mem::zeroed();
        contextDescription.stype = _ze_structure_type_t_ZE_STRUCTURE_TYPE_CONTEXT_DESC;
        result = zeContextCreate(*driver, &contextDescription, &mut context);
    }

    log::info!("zeContextCreate: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero context created"),
        _ => return Err("Error: zeContextCreate failed!"),
    }

    Ok(context)
}

fn get_command_queue(
    device: &ze_device_handle_t,
    context: &ze_context_handle_t,
) -> Result<(ze_command_queue_handle_t, ze_command_list_handle_t), &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let mut result;
    let mut count: u32 = 0;
    let mut command_queue_group_properties: Vec<ze_command_queue_group_properties_t>;
    unsafe {
        command_queue_group_properties = vec![];

        result = zeDeviceGetCommandQueueGroupProperties(
            *device,
            &mut count,
            command_queue_group_properties.as_mut_ptr(),
        );
    }
    log::info!(
        "zeDeviceGetCommandQueueGroupProperties: {}",
        error_msgs[&result]
    );

    match (result, count) {
        (_ze_result_t_ZE_RESULT_SUCCESS, 0) => return Err("No Level Zero command queue groups!"),
        (_ze_result_t_ZE_RESULT_SUCCESS, _) => {
            log::info!("Num Level Zero command queue groups {count}")
        }
        (_, _) => return Err("Error: zeDeviceGetCommandQueueGroupProperties failed!"),
    }

    unsafe {
        command_queue_group_properties = vec![mem::zeroed(); count as usize];
        result = zeDeviceGetCommandQueueGroupProperties(
            *device,
            &mut count,
            command_queue_group_properties.as_mut_ptr(),
        );
    }

    log::info!(
        "zeDeviceGetCommandQueueGroupProperties: {}",
        error_msgs[&result]
    );

    if result != _ze_result_t_ZE_RESULT_SUCCESS {
        return Err("Error: zeDeviceGetCommandQueueGroupProperties failed!");
    }

    let mut cmdQueueDesc: ze_command_queue_desc_t;
    unsafe {
        cmdQueueDesc = mem::zeroed();
    }
    for i in 0..count {
        if command_queue_group_properties[i as usize].flags
            & _ze_command_queue_group_property_flag_t_ZE_COMMAND_QUEUE_GROUP_PROPERTY_FLAG_COMPUTE
            != 0
        {
            cmdQueueDesc.ordinal = i;
            break;
        }
    }

    cmdQueueDesc.index = 0;
    cmdQueueDesc.mode = _ze_command_queue_mode_t_ZE_COMMAND_QUEUE_MODE_ASYNCHRONOUS;
    let mut cmdQueue: ze_command_queue_handle_t;
    unsafe {
        cmdQueue = mem::zeroed();
        result = zeCommandQueueCreate(*context, *device, &cmdQueueDesc, &mut cmdQueue);
    }

    log::info!("zeCommandQueueCreate: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero command queue created"),
        _ => return Err("Error: zeCommandQueueCreate failed!"),
    }

    // Create a command list
    let mut cmdList: ze_command_list_handle_t;
    unsafe {
        cmdList = mem::zeroed();
        let mut cmdListDesc: ze_command_list_desc_t = mem::zeroed();
        cmdListDesc.commandQueueGroupOrdinal = cmdQueueDesc.ordinal;
        result = zeCommandListCreate(*context, *device, &cmdListDesc, &mut cmdList);
    }

    log::info!("zeCommandListCreate: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => Ok((cmdQueue, cmdList)),
        _ => return Err("Error: zeCommandListCreate failed!"),
    }
}

fn zero_data_f32(
    mat: &*mut ::std::os::raw::c_void,
    num_elements: usize,
) -> Result<(), &'static str> {
    let v;
    unsafe {
        let ptr = *mat as *const _ as *mut f32;
        v = std::slice::from_raw_parts_mut::<f32>(ptr, num_elements);
    }
    v.iter_mut().enumerate().for_each(|(_, el)| *el = 0.0f32);
    Ok(())
}

fn buffer_to_ndarray(
    mat: &*mut ::std::os::raw::c_void,
    matrix_n_dim: usize,
) -> Result<ndarray::Array2<f32>, &'static str> {
    let a;
    let num_elements = matrix_n_dim * matrix_n_dim;
    unsafe {
        let ptr = *mat as *const _ as *mut f32;
        a = std::slice::from_raw_parts_mut::<f32>(ptr, num_elements);
    }
    Ok(Array::from_iter(a.iter().cloned())
        .into_shape((matrix_n_dim, matrix_n_dim))
        .expect("Could not create first ndarray"))
}

fn matrix_multiply(
    mat1: &*mut ::std::os::raw::c_void,
    mat2: &*mut ::std::os::raw::c_void,
    matrix_n_dim: usize,
) -> Result<ndarray::Array2<f32>, &'static str> {
    let a1;
    let a2;
    let num_elements = matrix_n_dim * matrix_n_dim;
    unsafe {
        let ptr = *mat1 as *const _ as *mut f32;
        a1 = std::slice::from_raw_parts_mut::<f32>(ptr, num_elements);
    }
    unsafe {
        let ptr = *mat2 as *const _ as *mut f32;
        a2 = std::slice::from_raw_parts_mut::<f32>(ptr, num_elements);
    }

    let a = Array::from_iter(a1.iter().cloned())
        .into_shape((matrix_n_dim, matrix_n_dim))
        .expect("Could not create first ndarray");
    let b = Array::from_iter(a2.iter().cloned())
        .into_shape((matrix_n_dim, matrix_n_dim))
        .expect("Could not create second ndarray");
    let mut c = Array::zeros((matrix_n_dim, matrix_n_dim));

    ndarray::linalg::general_mat_mul(1.0f32, &a, &b, 0.0f32, &mut c);
    Ok(c)
}

fn fill_data_f32(
    mat: &*mut ::std::os::raw::c_void,
    num_elements: usize,
) -> Result<(), &'static str> {
    let v;
    unsafe {
        let ptr = *mat as *const _ as *mut f32;
        v = std::slice::from_raw_parts_mut::<f32>(ptr, num_elements);
    }
    v.iter_mut().enumerate().for_each(|(i, el)| *el = i as f32);
    Ok(())
}

fn get_shared_buffer(
    context: &ze_context_handle_t,
    device: &ze_device_handle_t,
    allocsize: usize,
) -> Result<*mut ::std::os::raw::c_void, &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let mut device_mem_desc: ze_device_mem_alloc_desc_t;
    let mut host_mem_desc: ze_host_mem_alloc_desc_t;
    let mut pptr: *mut ::std::os::raw::c_void;
    let result;
    unsafe {
        pptr = std::ptr::null_mut();
        device_mem_desc = mem::zeroed();
        device_mem_desc.stype = _ze_structure_type_t_ZE_STRUCTURE_TYPE_DEVICE_MEM_ALLOC_DESC;
        host_mem_desc = mem::zeroed();
        host_mem_desc.stype = _ze_structure_type_t_ZE_STRUCTURE_TYPE_HOST_MEM_ALLOC_DESC;

        result = zeMemAllocShared(
            *context,
            &device_mem_desc,
            &host_mem_desc,
            allocsize,
            1,
            *device,
            &mut pptr,
        );
    }

    log::info!("zeMemAllocShared: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => Ok(pptr),
        _ => return Err("Error: zeMemAllocShared failed!"),
    }
}

fn get_kernel(
    context: &ze_context_handle_t,
    device: &ze_device_handle_t,
) -> Result<ze_kernel_handle_t, &'static str> {
    let error_msgs = make_descriptive_error_codes();
    //TODO: Enable when shader compilation to Kernel works
    // const COMPUTE_SHADER: &[u8] = include_bytes!(env!("shader.spv"));

    const COMPUTE_SHADER: &[u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/matrixMultiply.spv"));

    let mut moduleDesc: ze_module_desc_t;
    unsafe {
        moduleDesc = mem::zeroed();
    }
    moduleDesc.format = _ze_module_format_t_ZE_MODULE_FORMAT_IL_SPIRV;
    moduleDesc.pInputModule = COMPUTE_SHADER.as_ptr();
    moduleDesc.inputSize = COMPUTE_SHADER.len();
    let empty_string = CString::new("").expect("Empty string to c_char error");
    moduleDesc.pBuildFlags = empty_string.as_ptr();

    let mut buildLog: ze_module_build_log_handle_t;
    let mut module: ze_module_handle_t;
    let mut result;
    unsafe {
        buildLog = mem::zeroed();
        module = mem::zeroed();
        result = zeModuleCreate(*context, *device, &moduleDesc, &mut module, &mut buildLog);
    }

    log::info!("zeModuleCreate: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("zeModuleCreate success!")
        }
        _ => {
            let mut szLog: usize = 0;
            let mut result;
            unsafe {
                result = zeModuleBuildLogGetString(buildLog, &mut szLog, std::ptr::null_mut());
                log::info!("zeModuleBuildLogGetString: {}", error_msgs[&result]);
                match result {
                    _ze_result_t_ZE_RESULT_SUCCESS => {
                        log::info!("zeModuleBuildLogGetString log size: {szLog}")
                    }
                    _ => return Err("Error: zeModuleBuildLogGetString failed!"),
                }

                let strLog = CString::from_vec_unchecked(vec![0; szLog]).into_raw();
                result = zeModuleBuildLogGetString(buildLog, &mut szLog, strLog);
                log::info!("zeModuleBuildLogGetString: {}", error_msgs[&result]);

                match result {
                    _ze_result_t_ZE_RESULT_SUCCESS => {
                        log::info!("Build Log: {:?}", strLog)
                    }
                    _ => return Err("Error: zeModuleBuildLogGetString failed!"),
                }
            }
            return Err("Error: zeModuleCreate failed!");
        }
    };

    // Kernel creation (out of module)

    let mut kernelDesc: ze_kernel_desc_t;
    let mut kernel: ze_kernel_handle_t;
    unsafe {
        kernelDesc = mem::zeroed();
        kernel = mem::zeroed();
    }
    kernelDesc.pKernelName = CString::new("mxm")
        .expect("Empty string to c_char error")
        .into_raw(); // Name of the Kernel.
    unsafe {
        result = zeKernelCreate(module, &kernelDesc, &mut kernel);
    }
    log::info!("zeKernelCreate: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("zeKErnelCreate succeeded!"),
        _ => return Err("Error: zeKernelCreate failed!"),
    }

    // quering kernel properties
    let mut kernelProperties: ze_kernel_properties_t;
    unsafe {
        kernelProperties = mem::zeroed();
        result = zeKernelGetProperties(kernel, &mut kernelProperties);
    }
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Kernel properties:");
            log::info!("  numKernelArgs: {}", kernelProperties.numKernelArgs);
            log::info!(
                "  requiredGroupSizeX: {}",
                kernelProperties.requiredGroupSizeX
            );
            log::info!(
                "  requiredGroupSizeY: {}",
                kernelProperties.requiredGroupSizeY
            );
            log::info!(
                "  requiredGroupSizeZ: {}",
                kernelProperties.requiredGroupSizeZ
            );
            log::info!(
                "  requiredNumSubgroups: {}",
                kernelProperties.requiredNumSubGroups
            );
            log::info!(
                "  requiredSubgroupSize: {}",
                kernelProperties.requiredSubgroupSize
            );
            log::info!("  maxSubgroupSize: {}", kernelProperties.maxSubgroupSize);
            log::info!("  maxNumSubgroups: {}", kernelProperties.maxNumSubgroups);
            log::info!("  localMemSize: {}", kernelProperties.localMemSize);
            log::info!("  privateMemSize: {}", kernelProperties.privateMemSize);
            log::info!("  spillMemSize: {}", kernelProperties.spillMemSize);
        }
        _ => return Err("Error: zeKernelGetProperties failed!"),
    }

    Ok(kernel)
}

enum AnyPointer {
    C(*const ::std::os::raw::c_void),
    M(*mut ::std::os::raw::c_void),
}

fn set_kernel_args(
    kernel: &mut ze_kernel_handle_t,
    arg_index: u32,
    mats_size: usize,
    mat: &AnyPointer,
) -> Result<(), &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let result;
    unsafe {
        result = match *mat {
            AnyPointer::C(val) => zeKernelSetArgumentValue(*kernel, arg_index, mats_size, val),
            AnyPointer::M(val) => zeKernelSetArgumentValue(*kernel, arg_index, mats_size, val),
        }
    }
    log::info!("zeKernelSetArgumentValue: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeKernelSetArgumentValue successful!");
            Ok(())
        }
        _ => Err("Error: zeKernelSetArgumentValue failed!"),
    }
}

fn dispatch_kernel(
    queue: &ze_command_queue_handle_t,
    qlist: &mut ze_command_list_handle_t,
    kernel: &mut ze_kernel_handle_t,
    dst_mat: &*mut ::std::os::raw::c_void,
    src1_mat: &*mut ::std::os::raw::c_void,
    src2_mat: &*mut ::std::os::raw::c_void,
    matrix_n_dim: u32,
) -> Result<(), &'static str> {
    let error_msgs = make_descriptive_error_codes();

    let mats_size: usize = matrix_n_dim as usize * matrix_n_dim as usize * 4;

    let globalSizeX: u32 = matrix_n_dim;
    let globalSizeY: u32 = matrix_n_dim;
    let globalSizeZ: u32 = 1;
    let mut groupSizeX: u32 = 0;
    let mut groupSizeY: u32 = 0;
    let mut groupSizeZ: u32 = 0;
    let mut result;
    unsafe {
        result = zeKernelSuggestGroupSize(
            *kernel,
            globalSizeX,
            globalSizeY,
            globalSizeZ,
            &mut groupSizeX,
            &mut groupSizeY,
            &mut groupSizeZ,
        );
    }
    log::info!("zeKernelSuggestGroupSize: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!(
            "Level Zero zeKernelSuggestGroupSize : [{groupSizeX},{groupSizeY},{groupSizeZ}]"
        ),
        _ => return Err("Error: zeKernelSuggestGroupSize failed!"),
    }

    unsafe {
        result = zeKernelSetGroupSize(*kernel, groupSizeX, groupSizeY, groupSizeZ);
    }

    log::info!("zeKernelSetGroupSize: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero zeKernelSetGroupSize successful!"),
        _ => return Err("Error: zeKernelSetGroupSize failed!"),
    }

    let src1_ptr = src1_mat as *const _ as *mut libc::c_void;
    set_kernel_args(kernel, 0, mats_size, &AnyPointer::C(src1_ptr))?;
    let src2_ptr = src2_mat as *const _ as *mut libc::c_void;
    set_kernel_args(kernel, 1, mats_size, &AnyPointer::C(src2_ptr))?;
    let dst_ptr = dst_mat as *const _ as *mut libc::c_void;
    set_kernel_args(kernel, 2, mats_size, &AnyPointer::C(dst_ptr))?;

    let dim_ptr = &matrix_n_dim as *const _ as *mut libc::c_void;
    set_kernel_args(
        kernel,
        3,
        std::mem::size_of::<i32>(),
        &AnyPointer::C(dim_ptr),
    )?;

    let dispatch = ze_group_count_t {
        groupCountX: globalSizeX / groupSizeX,
        groupCountY: globalSizeY / groupSizeY,
        groupCountZ: globalSizeZ,
    };
    unsafe {
        result = zeCommandListAppendLaunchKernel(
            *qlist,
            *kernel,
            &dispatch,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        );
    }
    log::info!("zeCommandListAppendLaunchKernel: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeCommandListAppendLaunchKernel successful!")
        }
        _ => return Err("Error: zeCommandListAppendLaunchKernel failed!"),
    }

    unsafe {
        result = zeCommandListClose(*qlist);
    }
    log::info!("zeCommandListClose: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeCommandListClose successful!")
        }
        _ => return Err("Error: zeCommandListClose failed!"),
    }

    unsafe {
        result = zeCommandQueueExecuteCommandLists(*queue, 1, qlist, std::ptr::null_mut());
    }
    log::info!("zeCommandQueueExecuteCommandLists: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeCommandQueueExecuteCommandLists successful!")
        }
        _ => return Err("Error: zeCommandQueueExecuteCommandLists failed!"),
    }
    Ok(())
}

fn free_command_list_and_queue(
    queue: &ze_command_queue_handle_t,
    qlist: &ze_command_list_handle_t,
) -> Result<(), &'static str> {
    let error_msgs = make_descriptive_error_codes();
    let mut result;
    unsafe {
        result = zeCommandListDestroy(*qlist);
    }
    log::info!("zeCommandListDestroy: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero zeCommandListDestroy successful!"),
        _ => return Err("Error: zeCommandListDestroy failed!"),
    }

    unsafe {
        result = zeCommandQueueDestroy(*queue);
    }
    log::info!("zeCommandQueueDestroy: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeCommandQueueDestroy successful!")
        }
        _ => return Err("Error: zeCommandQueueDestroy failed!"),
    }
    Ok(())
}

fn free_context(context: &ze_context_handle_t) -> Result<(), &'static str> {
    let error_msgs = make_descriptive_error_codes();

    let result;
    unsafe {
        result = zeContextDestroy(*context);
    }
    log::info!("zeContextDestroy: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeContextDestroy successful!");
            Ok(())
        }
        _ => return Err("Error: zeContextDestroy failed!"),
    }
}

fn free_buffer(
    context: &ze_context_handle_t,
    buffer: &*mut ::std::os::raw::c_void,
) -> Result<(), &'static str> {
    let error_msgs = make_descriptive_error_codes();

    let result;
    unsafe {
        result = zeMemFree(*context, *buffer);
    }
    log::info!("zeMemFree: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeMemFree successful!");
            Ok(())
        }
        _ => return Err("Error: zeMemFree failed!"),
    }
}

fn main() -> Result<(), &'static str> {
    println!("Hello, Level-Zero world!");

    // Make a default logging level: error
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error")
    }
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let error_msgs = make_descriptive_error_codes();

    let mut result;
    unsafe {
        result = zeInit(_ze_init_flag_t_ZE_INIT_FLAG_GPU_ONLY);
    }

    log::info!("zeInit: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => println!("Success: Level zero initialized!"),
        _ => panic!("Error: zeInit failed!"),
    }

    let l0_driver = get_first_driver()?;

    let l0_device = get_first_device(&l0_driver)?;

    let context = get_context(&l0_driver)?;

    let (queue, mut qlist) = get_command_queue(&l0_device, &context)?;

    let matrix_n_dim: usize = 32; // Dim of square matrix
    let matrix_size = matrix_n_dim * matrix_n_dim * 4;

    let A_matrix = get_shared_buffer(&context, &l0_device, matrix_size)?;
    let B_matrix = get_shared_buffer(&context, &l0_device, matrix_size)?;
    let C_matrix = get_shared_buffer(&context, &l0_device, matrix_size)?;

    fill_data_f32(&A_matrix, matrix_n_dim * matrix_n_dim)?;
    fill_data_f32(&B_matrix, matrix_n_dim * matrix_n_dim)?;
    zero_data_f32(&C_matrix, matrix_n_dim * matrix_n_dim)?;

    let mut kernel = get_kernel(&context, &l0_device)?;

    dispatch_kernel(
        &queue,
        &mut qlist,
        &mut kernel,
        &C_matrix,
        &A_matrix,
        &B_matrix,
        matrix_n_dim as u32,
    )?;

    unsafe {
        result = zeCommandQueueSynchronize(queue, u64::MAX);
    }
    log::info!("zeCommandQueueSynchronize: {}", error_msgs[&result]);
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => {
            log::info!("Level Zero zeCommandQueueSynchronize successful!")
        }
        _ => return Err("Error: zeCommandQueueSynchronize failed!"),
    }

    // Get CPU result..
    let ref_C_matrix = matrix_multiply(&A_matrix, &B_matrix, matrix_n_dim)?;

    let C_nd_matrix = buffer_to_ndarray(&C_matrix, matrix_n_dim)?;
    //..and compare with GPU result
    assert!(
        C_nd_matrix == ref_C_matrix,
        "Error: GPU result does not match CPU (reference) result"
    );

    // Clean up
    free_buffer(&context, &A_matrix)?;
    free_buffer(&context, &B_matrix)?;
    free_buffer(&context, &C_matrix)?;
    free_command_list_and_queue(&queue, &qlist)?;
    free_context(&context)?;
    Ok(())
}
