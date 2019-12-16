use phper::alloc::EAllocator;
use phper::zval_str;
use phper::{
    php_fe_end, php_function, php_minit_function, php_mshutdown_function, php_rinit_function,
    php_rshutdown_function, zend_call_num_args,
};

use libc::size_t;
use phper_sys::{
    strpprintf, zend_function_entry, zend_function_entry_wrapper, zend_internal_arg_info,
    zend_module_entry, zend_module_entry_wrapper, zend_parse_parameters, zend_string,
    ZEND_RESULT_CODE_FAILURE, ZEND_RESULT_CODE_SUCCESS,
};

use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void};

#[global_allocator]
static GLOBAL: EAllocator = EAllocator;

#[php_function]
pub fn confirm_{{crate_name}}_compiled() {
    //    #[zend_parse_parameters(type_spec = "s")]
    //    let (arg, arg_len): ((), usize);

    let mut arg: *mut c_char = 0 as *mut c_char;
    let mut arg_len: size_t = 0;
    let len: size_t;
    let mut strg: *mut zend_string;

    unsafe {
        if zend_parse_parameters(
            zend_call_num_args!(execute_data) as c_int,
            "s\0".as_ptr() as *const c_char,
            &mut arg,
            &mut arg_len,
        ) == ZEND_RESULT_CODE_FAILURE
        {
            return;
        }
    }

    let s = "test ebox".to_string();
    println!("{}", s);

    let b =  Box::new(123);
    let _ = Box::into_raw(b);

    unsafe {
        strg = strpprintf(
              0,
              "Congratulations! You have successfully modified ext/%.78s/config.m4. Module %.78s is now compiled into PHP.\0".as_ptr() as *const c_char,
              "{{crate_name}}\0".as_ptr() as *const c_char,
              arg);
    }

    unsafe {
        zval_str!(return_value, strg);
        return;
    }
}

#[php_minit_function]
pub fn {{crate_name}}() {
    return ZEND_RESULT_CODE_SUCCESS;
}

#[php_mshutdown_function]
pub fn {{crate_name}}() {
    return ZEND_RESULT_CODE_SUCCESS;
}

#[php_rinit_function]
pub fn {{crate_name}}() {
    return ZEND_RESULT_CODE_SUCCESS;
}

#[php_rshutdown_function]
pub fn {{crate_name}}() {
    return ZEND_RESULT_CODE_SUCCESS;
}

#[no_mangle]
static {{crate_name}}_functions: zend_function_entry_wrapper = zend_function_entry_wrapper(&[
    zend_function_entry {
        fname: "confirm_{{crate_name}}_compiled\0".as_ptr() as *const c_char,
        handler: Some(zif_confirm_{{crate_name}}_compiled),
        arg_info: 0 as *const zend_internal_arg_info,
        num_args: 0,
        flags: 0,
    },
    php_fe_end!(),
]
    as *const zend_function_entry);

#[no_mangle]
pub static {{crate_name}}_module_entry: zend_module_entry_wrapper =
    zend_module_entry_wrapper(&zend_module_entry {
        size: size_of::<zend_module_entry>() as c_ushort,
        zend_api: ::phper_sys::ZEND_MODULE_API_NO as c_uint,
        zend_debug: ::phper_sys::ZEND_DEBUG as c_uchar,
        zts: ::phper_sys::USING_ZTS as c_uchar,
        ini_entry: 0 as *const ::phper_sys::zend_ini_entry,
        deps: 0 as *const ::phper_sys::zend_module_dep,
        name: "{{crate_name}}\0".as_ptr() as *const c_char,
        functions: {{crate_name}}_functions.0,
        module_startup_func: Some(zm_activate_{{crate_name}}),
        module_shutdown_func: Some(zm_deactivate_{{crate_name}}),
        request_startup_func: Some(zm_shutdown_{{crate_name}}),
        request_shutdown_func: Some(zm_startup_{{crate_name}}),
        info_func: None,
        version: "0.1.0\0".as_ptr() as *const c_char,
        globals_size: 0usize,
        globals_ptr: 0 as *mut c_void,
        globals_ctor: None,
        globals_dtor: None,
        post_deactivate_func: None,
        module_started: 0,
        type_: 0,
        handle: 0 as *mut c_void,
        module_number: 0,
        build_id: "API20160303,NTS\0".as_ptr() as *const c_char,
    } as *const zend_module_entry);


#[no_mangle]
pub extern "C" fn get_module() -> *const zend_module_entry {
    {{crate_name}}_module_entry.0
}

