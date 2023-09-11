#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// TODO: read tutorial and make more calls for educational purposes
// TODO: Some basic kernel should be read loaded and executed

fn main() {
    println!("Hello, Level-Zero world!");

    // Make a default logging level: error
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error")
    }
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let result;
    unsafe{
        result = zeInit(_ze_init_flag_t_ZE_INIT_FLAG_GPU_ONLY) ;
    }
    log::info!("zeInit: {}",result); 
    match result {
        _ze_result_t_ZE_RESULT_SUCCESS => println!("Success: Level zero initialized!"),
        _ => panic!("Error: zeInit failed!")
    }
}
