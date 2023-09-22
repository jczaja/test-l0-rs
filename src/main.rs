#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ffi::CString;
use std::mem;

// TODO: read tutorial and make more calls for educational purposes
// TODO: Some basic kernel should be read loaded and executed

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
        device_mem_desc.flags = _ze_device_mem_alloc_flag_t_ZE_DEVICE_MEM_ALLOC_FLAG_BIAS_CACHED;
        device_mem_desc.ordinal = 0;
        host_mem_desc = mem::zeroed();
        host_mem_desc.flags = _ze_host_mem_alloc_flag_t_ZE_HOST_MEM_ALLOC_FLAG_BIAS_CACHED;

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

fn get_shader(
    context: &ze_context_handle_t,
    device: &ze_device_handle_t,
) -> Result<ze_kernel_handle_t, &'static str> {
    let error_msgs = make_descriptive_error_codes();
    const COMPUTE_SHADER: &[u8] = include_bytes!(env!("shader.spv"));

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
    kernelDesc.pKernelName = CString::new("main_cs")
        .expect("Empty string to c_char error")
        .into_raw(); // Name of the Kernel.
    unsafe {
        result = zeKernelCreate(module, &kernelDesc, &mut kernel);
    }
    log::info!("zeKernelCreate: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => Ok(kernel),
        _ => return Err("Error: zeKernelCreate failed!"),
    }
}

fn dispatch_kernel(qlist: &ze_command_list_handle_t,kernel : &ze_kernel_handle_t) -> Result<(),&'static str>
{
    let error_msgs = make_descriptive_error_codes();

    let globalSizeX: u32 = 32;
    let globalSizeY: u32 = 32;
    let globalSizeZ: u32 = 1;
    let mut groupSizeX: u32 = 0;
    let mut groupSizeY: u32 = 0;
    let mut groupSizeZ: u32 = 0;
    let mut result; 
    unsafe {
        result = zeKernelSuggestGroupSize(*kernel , globalSizeX, globalSizeY, globalSizeZ, &mut groupSizeX, &mut groupSizeY, &mut groupSizeZ);
    }
    log::info!("zeKernelSuggestGroupSize: {}", error_msgs[&result]);

    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => log::info!("Level Zero zeKernelSuggestGroupSize : [{groupSizeX},{groupSizeY},{groupSizeZ}]"),
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

    todo!();
}


fn main() -> Result<(), &'static str> {
    println!("Hello, Level-Zero world!");

    // Make a default logging level: error
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error")
    }
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let error_msgs = make_descriptive_error_codes();

    let result;
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

    let (queue, qlist) = get_command_queue(&l0_device, &context)?;

    let matrix_n_dim: usize = 1024;

    let A_matrix = get_shared_buffer(&context, &l0_device, matrix_n_dim * matrix_n_dim * 4)?;
    let B_matrix = get_shared_buffer(&context, &l0_device, matrix_n_dim * matrix_n_dim * 4)?;
    let C_matrix = get_shared_buffer(&context, &l0_device, matrix_n_dim * matrix_n_dim * 4)?;

    let kernel = get_shader(&context, &l0_device)?;

    dispatch_kernel(&qlist,&kernel)?;
    Ok(())

    // TODO: make CI
    // TODO: load spirv
    // TODO: dispatch
}
