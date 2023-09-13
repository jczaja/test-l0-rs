#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashMap;
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

    Ok(())

    // TODO: make context
    // TODO: make queues
    // TODO: make command buffers
}
