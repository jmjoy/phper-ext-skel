use std::cell::Cell;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void};

use phper::{
    c_str_ptr, define_php_ini, define_zend_functions, define_zend_module_entry, php_function,
    php_minfo_function, php_minit_function, php_mshutdown_function, php_rinit_function,
    php_rshutdown_function, std_php_ini_entry, zend_call_num_args, zend_function_entry,
    zend_get_module, zval_str, IniEntries, NotThreadSafe, StaticZendFunctionEntry,
    StaticZendModuleEntry,
};

use phper::sys::{
    display_ini_entries, php_info_print_table_end, php_info_print_table_header,
    php_info_print_table_start, strpprintf, zend_long, zend_parse_parameters, zend_string,
    OnUpdateLong, OnUpdateString, PHP_INI_ALL, ZEND_RESULT_CODE_FAILURE,
};

thread_local! {
    static GLOBAL_VALUE: Cell<zend_long> = Cell::new(0);
    static GLOBAL_STRING: Cell<*const c_char> = Cell::new(c_str_ptr!(""));

    static INI_ENTRIES: IniEntries = define_php_ini![
        std_php_ini_entry!("{{crate_name}}.global_value", "43", PHP_INI_ALL, OnUpdateLong, GLOBAL_VALUE),
        std_php_ini_entry!("{{crate_name}}.global_string", "foobar", PHP_INI_ALL, OnUpdateString, GLOBAL_STRING),
    ];
}

#[php_function]
pub fn zif_confirm_{{crate_name}}_compiled() {
    let mut arg: *mut c_char = 0 as *mut c_char;
    let mut arg_len: usize = 0;
    let strg: *mut zend_string;

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
pub fn zm_startup_{{crate_name}}() -> bool {
    true
}

#[php_mshutdown_function]
pub fn zm_shutdown_{{crate_name}}() -> bool {
    true
}

#[php_rinit_function]
pub fn zm_activate_{{crate_name}}() -> bool {
    true
}

#[php_rshutdown_function]
pub fn zm_deactivate_{{crate_name}}() -> bool {
    true
}

#[php_minfo_function]
pub fn zm_info_{{crate_name}}() {
    unsafe {
        php_info_print_table_start();
        php_info_print_table_header(2, c_str_ptr!("{{crate_name}} support"), c_str_ptr!("enabled"));
        php_info_print_table_end();

        display_ini_entries(zend_module);
    }
}

pub static FUNCTIONS: StaticZendFunctionEntry = define_zend_functions![zend_function_entry!(
    c_str_ptr!("confirm_{{crate_name}}_compiled"),
    Some(zif_confirm_{{crate_name}}_compiled)
),];

pub static MODULE_ENTRY: StaticZendModuleEntry = define_zend_module_entry!(
    c_str_ptr!("{{crate_name}}"),
    FUNCTIONS,
    Some(zm_startup_{{crate_name}}),
    Some(zm_shutdown_{{crate_name}}),
    Some(zm_activate_{{crate_name}}),
    Some(zm_deactivate_{{crate_name}}),
    Some(zm_info_{{crate_name}})
);

zend_get_module!(MODULE_ENTRY);
