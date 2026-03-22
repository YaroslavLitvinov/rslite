#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod sqlite3ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sqlite3_api_routines {
        pub aggregate_context:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub aggregate_count: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ::core::ffi::c_int>,
        pub bind_blob:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_double:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_int:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_int64:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                crate::sqlite3_h::sqlite_int64,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_null:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub bind_parameter_count: 
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub bind_parameter_index:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_parameter_name:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub bind_text:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_text16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_value:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *const crate::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >,
        pub busy_handler:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub busy_timeout: 
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, ::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub changes:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub close:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub collation_needed:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut crate::sqliteInt_h::sqlite3,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                    ) -> (),
                >,
            ) -> ::core::ffi::c_int,
        >,
        pub collation_needed16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut crate::sqliteInt_h::sqlite3,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> (),
                >,
            ) -> ::core::ffi::c_int,
        >,
        pub column_blob:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_bytes:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub column_bytes16:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub column_count:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub column_database_name:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub column_database_name16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_decltype:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub column_decltype16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_double:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_double,
        >,
        pub column_int:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub column_int64: 
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> crate::sqlite3_h::sqlite_int64>,
        pub column_name:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub column_name16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_origin_name:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub column_origin_name16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_table_name:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub column_table_name16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_text:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_uchar,
        >,
        pub column_text16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        >,
        pub column_type:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub column_value:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> *mut crate::vdbeInt_h::sqlite3_value,
        >,
        pub commit_hook:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub complete: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
        pub complete16: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub create_collation:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >,
            ) -> ::core::ffi::c_int,
        >,
        pub create_collation16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >,
            ) -> ::core::ffi::c_int,
        >,
        pub create_function:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub create_function16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub create_module:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *const crate::sqlite3_h::sqlite3_module,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub data_count:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub db_handle:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> *mut crate::sqliteInt_h::sqlite3>,
        pub declare_vtab:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
        >,
        pub enable_shared_cache: 
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub errcode:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub errmsg:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> *const ::core::ffi::c_char>,
        pub errmsg16:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> *const ::core::ffi::c_void>,
        pub exec:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_callback,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub expired:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub finalize:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub free:  Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub free_table:  Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_char) -> ()>,
        pub get_autocommit:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub get_auxdata:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub get_table:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *mut *mut *mut ::core::ffi::c_char,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub global_recover:  Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        pub interruptx:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ()>,
        pub last_insert_rowid:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> crate::sqlite3_h::sqlite_int64>,
        pub libversion:  Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
        pub libversion_number:  Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        pub malloc:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void>,
        pub mprintf:  Option<
            unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char,
        >,
        pub open:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut *mut crate::sqliteInt_h::sqlite3,
            ) -> ::core::ffi::c_int,
        >,
        pub open16:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *mut *mut crate::sqliteInt_h::sqlite3,
            ) -> ::core::ffi::c_int,
        >,
        pub prepare:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut *mut crate::sqlite3_h::sqlite3_stmt,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub prepare16:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::sqlite3_h::sqlite3_stmt,
                *mut *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub profile:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        crate::sqlite3_h::sqlite_uint64,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub progress_handler:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub realloc:  Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub reset:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub result_blob:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub result_double: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, ::core::ffi::c_double) -> ()>,
        pub result_error:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub result_error16:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub result_int: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, ::core::ffi::c_int) -> ()>,
        pub result_int64:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, crate::sqlite3_h::sqlite_int64) -> ()>,
        pub result_null:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
        pub result_text:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub result_text16:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub result_text16be:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub result_text16le:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub result_value: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, *mut crate::vdbeInt_h::sqlite3_value) -> ()>,
        pub rollback_hook:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub set_authorizer:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub set_auxdata:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub xsnprintf:  Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ...
            ) -> *mut ::core::ffi::c_char,
        >,
        pub step:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub table_column_metadata:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub thread_cleanup:  Option<unsafe extern "C" fn() -> ()>,
        pub total_changes:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub trace:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub transfer_bindings:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, *mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int,
        >,
        pub update_hook:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        crate::sqlite3_h::sqlite_int64,
                    ) -> (),
                >,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub user_data: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> *mut ::core::ffi::c_void>,
        pub value_blob: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> *const ::core::ffi::c_void>,
        pub value_bytes:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub value_bytes16:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub value_double:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_double>,
        pub value_int:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub value_int64:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> crate::sqlite3_h::sqlite_int64>,
        pub value_numeric_type: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub value_text: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> *const ::core::ffi::c_uchar>,
        pub value_text16: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> *const ::core::ffi::c_void>,
        pub value_text16be: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> *const ::core::ffi::c_void>,
        pub value_text16le: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> *const ::core::ffi::c_void>,
        pub value_type:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub vmprintf:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> *mut ::core::ffi::c_char,
        >,
        pub overload_function:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub prepare_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut *mut crate::sqlite3_h::sqlite3_stmt,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub prepare16_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::sqlite3_h::sqlite3_stmt,
                *mut *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub clear_bindings:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub create_module_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *const crate::sqlite3_h::sqlite3_module,
                *mut ::core::ffi::c_void,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_zeroblob:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub blob_bytes:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_blob) -> ::core::ffi::c_int>,
        pub blob_close:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_blob) -> ::core::ffi::c_int>,
        pub blob_open:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_int64,
                ::core::ffi::c_int,
                *mut *mut crate::sqlite3_h::sqlite3_blob,
            ) -> ::core::ffi::c_int,
        >,
        pub blob_read:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_blob,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub blob_write:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_blob,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub create_collation_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub file_control:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub memory_highwater:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> crate::sqlite3_h::sqlite3_int64>,
        pub memory_used:  Option<unsafe extern "C" fn() -> crate::sqlite3_h::sqlite3_int64>,
        pub mutex_alloc:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex>,
        pub mutex_enter:  Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
        pub mutex_free:  Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
        pub mutex_leave:  Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
        pub mutex_try:  Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int>,
        pub open_v2:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub release_memory:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub result_error_nomem:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
        pub result_error_toobig:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
        pub sleep:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub soft_heap_limit:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        pub vfs_find:  Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut crate::sqlite3_h::sqlite3_vfs>,
        pub vfs_register:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub vfs_unregister:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vfs) -> ::core::ffi::c_int>,
        pub xthreadsafe:  Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        pub result_zeroblob: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, ::core::ffi::c_int) -> ()>,
        pub result_error_code: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, ::core::ffi::c_int) -> ()>,
        pub test_control: 
            Option<unsafe extern "C" fn(::core::ffi::c_int, ...) -> ::core::ffi::c_int>,
        pub randomness: 
            Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()>,
        pub context_db_handle:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> *mut crate::sqliteInt_h::sqlite3>,
        pub extended_result_codes: 
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, ::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub limit:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub next_stmt: 
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, *mut crate::sqlite3_h::sqlite3_stmt) -> *mut crate::sqlite3_h::sqlite3_stmt>,
        pub sql:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> *const ::core::ffi::c_char>,
        pub status:  Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub backup_finish:  Option<unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup) -> ::core::ffi::c_int>,
        pub backup_init:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
            ) -> *mut crate::src::src::backup::sqlite3_backup,
        >,
        pub backup_pagecount: 
            Option<unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup) -> ::core::ffi::c_int>,
        pub backup_remaining: 
            Option<unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup) -> ::core::ffi::c_int>,
        pub backup_step:  Option<
            unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub compileoption_get: 
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> *const ::core::ffi::c_char>,
        pub compileoption_used: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
        pub create_function_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub db_config:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, ::core::ffi::c_int, ...) -> ::core::ffi::c_int,
        >,
        pub db_mutex:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> *mut crate::src::src::mutex_unix::sqlite3_mutex>,
        pub db_status:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub extended_errcode:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub log: 
            Option<unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char, ...) -> ()>,
        pub soft_heap_limit64:  Option<unsafe extern "C" fn(crate::sqlite3_h::sqlite3_int64) -> crate::sqlite3_h::sqlite3_int64>,
        pub sourceid:  Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
        pub stmt_status:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub strnicmp:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub unlock_notify:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
                >,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub wal_autocheckpoint: 
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, ::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub wal_checkpoint:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
        >,
        pub wal_hook:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut crate::sqliteInt_h::sqlite3,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub blob_reopen: 
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_blob, crate::sqlite3_h::sqlite3_int64) -> ::core::ffi::c_int>,
        pub vtab_config:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, ::core::ffi::c_int, ...) -> ::core::ffi::c_int,
        >,
        pub vtab_on_conflict:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub close_v2:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub db_filename:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
            ) -> *const ::core::ffi::c_char,
        >,
        pub db_readonly:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
        >,
        pub db_release_memory:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub errstr:  Option<unsafe extern "C" fn(::core::ffi::c_int) -> *const ::core::ffi::c_char>,
        pub stmt_busy:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub stmt_readonly:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub stricmp:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub uri_boolean:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub uri_int64:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_int64,
            ) -> crate::sqlite3_h::sqlite3_int64,
        >,
        pub uri_parameter:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> *const ::core::ffi::c_char,
        >,
        pub xvsnprintf:  Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> *mut ::core::ffi::c_char,
        >,
        pub wal_checkpoint_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub auto_extension:  Option<
            unsafe extern "C" fn(Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int,
        >,
        pub bind_blob64:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
                crate::sqlite3_h::sqlite3_uint64,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_text64:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_uint64,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                ::core::ffi::c_uchar,
            ) -> ::core::ffi::c_int,
        >,
        pub cancel_auto_extension:  Option<
            unsafe extern "C" fn(Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int,
        >,
        pub load_extension:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub malloc64:  Option<unsafe extern "C" fn(crate::sqlite3_h::sqlite3_uint64) -> *mut ::core::ffi::c_void>,
        pub msize:  Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> crate::sqlite3_h::sqlite3_uint64>,
        pub realloc64:  Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                crate::sqlite3_h::sqlite3_uint64,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub reset_auto_extension:  Option<unsafe extern "C" fn() -> ()>,
        pub result_blob64:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_void,
                crate::sqlite3_h::sqlite3_uint64,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub result_text64:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_uint64,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                ::core::ffi::c_uchar,
            ) -> (),
        >,
        pub strglob:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub value_dup:  Option<unsafe extern "C" fn(*const crate::vdbeInt_h::sqlite3_value) -> *mut crate::vdbeInt_h::sqlite3_value>,
        pub value_free:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ()>,
        pub result_zeroblob64:  Option<
            unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, crate::sqlite3_h::sqlite3_uint64) -> ::core::ffi::c_int,
        >,
        pub bind_zeroblob64:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                crate::sqlite3_h::sqlite3_uint64,
            ) -> ::core::ffi::c_int,
        >,
        pub value_subtype:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_uint>,
        pub result_subtype: 
            Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context, ::core::ffi::c_uint) -> ()>,
        pub status64:  Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut crate::sqlite3_h::sqlite3_int64,
                *mut crate::sqlite3_h::sqlite3_int64,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub strlike:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_int,
        >,
        pub db_cacheflush:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub system_errno:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub trace_v2:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_uint,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_uint,
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub expanded_sql: 
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> *mut ::core::ffi::c_char>,
        pub set_last_insert_rowid:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, crate::sqlite3_h::sqlite3_int64) -> ()>,
        pub prepare_v3:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_uint,
                *mut *mut crate::sqlite3_h::sqlite3_stmt,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub prepare16_v3:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_uint,
                *mut *mut crate::sqlite3_h::sqlite3_stmt,
                *mut *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub bind_pointer:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_stmt,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub result_pointer:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> (),
        >,
        pub value_pointer:  Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_value,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub vtab_nochange:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ::core::ffi::c_int>,
        pub value_nochange:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub vtab_collation:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_index_info,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub keyword_count:  Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        pub keyword_name:  Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut *const ::core::ffi::c_char,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub keyword_check:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub str_new:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> *mut crate::sqliteInt_h::sqlite3_str>,
        pub str_finish:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str) -> *mut ::core::ffi::c_char>,
        pub str_appendf: 
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str, *const ::core::ffi::c_char, ...) -> ()>,
        pub str_vappendf:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3_str,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub str_append:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3_str,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub str_appendall: 
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str, *const ::core::ffi::c_char) -> ()>,
        pub str_appendchar:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str, ::core::ffi::c_int, ::core::ffi::c_char) -> (),
        >,
        pub str_reset:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str) -> ()>,
        pub str_errcode:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str) -> ::core::ffi::c_int>,
        pub str_length:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str) -> ::core::ffi::c_int>,
        pub str_value:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3_str) -> *mut ::core::ffi::c_char>,
        pub create_window_function:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
                Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
                Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub normalized_sql: 
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> *const ::core::ffi::c_char>,
        pub stmt_isexplain:  Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt) -> ::core::ffi::c_int>,
        pub value_frombind:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub drop_modules:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub hard_heap_limit64:  Option<unsafe extern "C" fn(crate::sqlite3_h::sqlite3_int64) -> crate::sqlite3_h::sqlite3_int64>,
        pub uri_key:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub filename_database: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char>,
        pub filename_journal: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char>,
        pub filename_wal: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char>,
        pub create_filename:  Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut *const ::core::ffi::c_char,
            ) -> *const ::core::ffi::c_char,
        >,
        pub free_filename:  Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
        pub database_file_object: 
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut crate::sqlite3_h::sqlite3_file>,
        pub txn_state:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
        >,
        pub changes64:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> crate::sqlite3_h::sqlite3_int64>,
        pub total_changes64:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> crate::sqlite3_h::sqlite3_int64>,
        pub autovacuum_pages:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                    ) -> ::core::ffi::c_uint,
                >,
                *mut ::core::ffi::c_void,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub error_offset:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub vtab_rhs_value:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_index_info,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >,
        pub vtab_distinct: 
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_index_info) -> ::core::ffi::c_int>,
        pub vtab_in:  Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_index_info,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub vtab_in_first:  Option<
            unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value, *mut *mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int,
        >,
        pub vtab_in_next:  Option<
            unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value, *mut *mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int,
        >,
        pub deserialize:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_uchar,
                crate::sqlite3_h::sqlite3_int64,
                crate::sqlite3_h::sqlite3_int64,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_int,
        >,
        pub serialize:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *mut crate::sqlite3_h::sqlite3_int64,
                ::core::ffi::c_uint,
            ) -> *mut ::core::ffi::c_uchar,
        >,
        pub db_name:  Option<
            unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, ::core::ffi::c_int) -> *const ::core::ffi::c_char,
        >,
        pub value_encoding:  Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_value) -> ::core::ffi::c_int>,
        pub is_interrupted:  Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3) -> ::core::ffi::c_int>,
        pub stmt_explain:  Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub get_clientdata:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub set_clientdata:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_void,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ) -> ::core::ffi::c_int,
        >,
        pub setlk_timeout:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub set_errmsg:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub db_status64:  Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *mut crate::sqlite3_h::sqlite3_int64,
                *mut crate::sqlite3_h::sqlite3_int64,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
    }
    
    
    pub type sqlite3_loadext_entry = Option<
        unsafe extern "C" fn(
            *mut crate::sqliteInt_h::sqlite3,
            *mut *mut ::core::ffi::c_char,
            *const crate::sqlite3ext_h::sqlite3_api_routines,
        ) -> ::core::ffi::c_int,
    >;
}pub mod pcache_h {
    pub use crate::src::src::pcache::PCache;
}pub mod __stddef_ptrdiff_t_h {
    pub type ptrdiff_t =  isize;
}pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list =  crate::internal::__builtin_va_list;
}pub mod __stddef_size_t_h {
    pub type size_t =  usize;
}pub mod whereInt_h {
    use crate::src::ext::rtree::rtree::u32_0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct WhereMemBlock {
        pub pNext:  *mut crate::whereInt_h::WhereMemBlock,
        pub sz:  crate::src::ext::rtree::rtree::u64_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereRightJoin {
        pub iMatch: ::core::ffi::c_int,
        pub regBloom: ::core::ffi::c_int,
        pub regReturn: ::core::ffi::c_int,
        pub addrSubrtn: ::core::ffi::c_int,
        pub endSubrtn: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereLevel {
        pub iLeftJoin: ::core::ffi::c_int,
        pub iTabCur: ::core::ffi::c_int,
        pub iIdxCur: ::core::ffi::c_int,
        pub addrBrk: ::core::ffi::c_int,
        pub addrHalt: ::core::ffi::c_int,
        pub addrNxt: ::core::ffi::c_int,
        pub addrSkip: ::core::ffi::c_int,
        pub addrCont: ::core::ffi::c_int,
        pub addrFirst: ::core::ffi::c_int,
        pub addrBody: ::core::ffi::c_int,
        pub regBignull: ::core::ffi::c_int,
        pub addrBignull: ::core::ffi::c_int,
        pub iLikeRepCntr: crate::src::ext::rtree::rtree::u32_0,
        pub addrLikeRep: ::core::ffi::c_int,
        pub regFilter: ::core::ffi::c_int,
        pub pRJ: *mut crate::whereInt_h::WhereRightJoin,
        pub iFrom: crate::src::ext::rtree::rtree::u8_0,
        pub op: crate::src::ext::rtree::rtree::u8_0,
        pub p3: crate::src::ext::rtree::rtree::u8_0,
        pub p5: crate::src::ext::rtree::rtree::u8_0,
        pub p1: ::core::ffi::c_int,
        pub p2: ::core::ffi::c_int,
        pub u: crate::whereInt_h::__anon_union_19,
        pub pWLoop: *mut crate::whereInt_h::WhereLoop,
        pub notReady: crate::sqliteInt_h::Bitmask,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_19 {
        pub in_0: crate::whereInt_h::__anon_struct_11,
        pub pCoveringIdx: *mut crate::sqliteInt_h::Index,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_11 {
        pub nIn: ::core::ffi::c_int,
        pub aInLoop: *mut crate::whereInt_h::InLoop,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct InLoop {
        pub iCur: ::core::ffi::c_int,
        pub addrInTop: ::core::ffi::c_int,
        pub iBase: ::core::ffi::c_int,
        pub nPrefix: ::core::ffi::c_int,
        pub eEndLoopOp: crate::src::ext::rtree::rtree::u8_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereLoop {
        pub prereq: crate::sqliteInt_h::Bitmask,
        pub maskSelf: crate::sqliteInt_h::Bitmask,
        pub iTab: crate::src::ext::rtree::rtree::u8_0,
        pub iSortIdx: crate::src::ext::rtree::rtree::u8_0,
        pub rSetup: crate::sqliteInt_h::LogEst,
        pub rRun: crate::sqliteInt_h::LogEst,
        pub nOut: crate::sqliteInt_h::LogEst,
        pub u: crate::whereInt_h::__anon_union_20,
        pub wsFlags: crate::src::ext::rtree::rtree::u32_0,
        pub nLTerm: crate::src::fts5::u16_0,
        pub nSkip: crate::src::fts5::u16_0,
        pub nLSlot: crate::src::fts5::u16_0,
        pub aLTerm: *mut *mut crate::whereInt_h::WhereTerm,
        pub pNextLoop: *mut crate::whereInt_h::WhereLoop,
        pub aLTermSpace: [*mut crate::whereInt_h::WhereTerm; 3],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_20 {
        pub btree: crate::whereInt_h::__anon_struct_12,
        pub vtab: crate::whereInt_h::__anon_struct_13,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_12 {
        pub nEq: crate::src::fts5::u16_0,
        pub nBtm: crate::src::fts5::u16_0,
        pub nTop: crate::src::fts5::u16_0,
        pub nDistinctCol: crate::src::fts5::u16_0,
        pub pIndex: *mut crate::sqliteInt_h::Index,
        pub pOrderBy: *mut crate::sqliteInt_h::ExprList,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct __anon_struct_13 {
        pub idxNum: ::core::ffi::c_int,
        #[bitfield(name = "needFree", ty = "u32_0", bits = "0..=0")]
        #[bitfield(name = "bOmitOffset", ty = "u32_0", bits = "1..=1")]
        #[bitfield(name = "bIdxNumHex", ty = "u32_0", bits = "2..=2")]
        pub needFree_bOmitOffset_bIdxNumHex: [u8; 1],
        pub isOrdered: crate::sqliteInt_h::i8_0,
        pub omitMask: crate::src::fts5::u16_0,
        pub idxStr: *mut ::core::ffi::c_char,
        pub mHandleIn: crate::src::ext::rtree::rtree::u32_0,
    }
    
    
    pub const WHERE_LOOP_XFER_SZ: ::core::ffi::c_ulong = 56 as ::core::ffi::c_ulong;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereOrCost {
        pub prereq: crate::sqliteInt_h::Bitmask,
        pub rRun: crate::sqliteInt_h::LogEst,
        pub nOut: crate::sqliteInt_h::LogEst,
    }
    
    
    pub const N_OR_COST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereOrSet {
        pub n: crate::src::fts5::u16_0,
        pub a: [crate::whereInt_h::WhereOrCost; 3],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WherePath {
        pub maskLoop: crate::sqliteInt_h::Bitmask,
        pub revLoop: crate::sqliteInt_h::Bitmask,
        pub nRow: crate::sqliteInt_h::LogEst,
        pub rCost: crate::sqliteInt_h::LogEst,
        pub rUnsort: crate::sqliteInt_h::LogEst,
        pub isOrdered: crate::sqliteInt_h::i8_0,
        pub aLoop: *mut *mut crate::whereInt_h::WhereLoop,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereTerm {
        pub pExpr: *mut crate::sqliteInt_h::Expr,
        pub pWC: *mut crate::whereInt_h::WhereClause,
        pub truthProb: crate::sqliteInt_h::LogEst,
        pub wtFlags: crate::src::fts5::u16_0,
        pub eOperator: crate::src::fts5::u16_0,
        pub nChild: crate::src::ext::rtree::rtree::u8_0,
        pub eMatchOp: crate::src::ext::rtree::rtree::u8_0,
        pub iParent: ::core::ffi::c_int,
        pub leftCursor: ::core::ffi::c_int,
        pub u: crate::whereInt_h::__anon_union_21,
        pub prereqRight: crate::sqliteInt_h::Bitmask,
        pub prereqAll: crate::sqliteInt_h::Bitmask,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_21 {
        pub x: crate::whereInt_h::__anon_struct_14,
        pub pOrInfo: *mut crate::whereInt_h::WhereOrInfo,
        pub pAndInfo: *mut crate::whereInt_h::WhereAndInfo,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_14 {
        pub leftColumn: ::core::ffi::c_int,
        pub iField: ::core::ffi::c_int,
    }
    
    
    pub const TERM_DYNAMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const TERM_VIRTUAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const TERM_CODED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const TERM_COPIED: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const TERM_ORINFO: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const TERM_ANDINFO: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const TERM_OK: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const TERM_VNULL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const TERM_LIKEOPT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const TERM_LIKECOND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const TERM_LIKE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const TERM_IS: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const TERM_VARSELECT: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const TERM_HEURTRUTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const TERM_HIGHTRUTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const TERM_SLICE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereScan {
        pub pOrigWC: *mut crate::whereInt_h::WhereClause,
        pub pWC: *mut crate::whereInt_h::WhereClause,
        pub zCollName: *const ::core::ffi::c_char,
        pub pIdxExpr: *mut crate::sqliteInt_h::Expr,
        pub k: ::core::ffi::c_int,
        pub opMask: crate::src::ext::rtree::rtree::u32_0,
        pub idxaff: ::core::ffi::c_char,
        pub iEquiv: ::core::ffi::c_uchar,
        pub nEquiv: ::core::ffi::c_uchar,
        pub aiCur: [::core::ffi::c_int; 11],
        pub aiColumn: [crate::src::fts5::i16_0; 11],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereClause {
        pub pWInfo: *mut crate::whereInt_h::WhereInfo,
        pub pOuter: *mut crate::whereInt_h::WhereClause,
        pub op: crate::src::ext::rtree::rtree::u8_0,
        pub hasOr: crate::src::ext::rtree::rtree::u8_0,
        pub nTerm: ::core::ffi::c_int,
        pub nSlot: ::core::ffi::c_int,
        pub nBase: ::core::ffi::c_int,
        pub a: *mut crate::whereInt_h::WhereTerm,
        pub aStatic: [crate::whereInt_h::WhereTerm; 8],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereOrInfo {
        pub wc: crate::whereInt_h::WhereClause,
        pub indexable: crate::sqliteInt_h::Bitmask,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereAndInfo {
        pub wc: crate::whereInt_h::WhereClause,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereMaskSet {
        pub bVarSelect: ::core::ffi::c_int,
        pub n: ::core::ffi::c_int,
        pub ix: [::core::ffi::c_int; 64],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct WhereLoopBuilder {
        pub pWInfo: *mut crate::whereInt_h::WhereInfo,
        pub pWC: *mut crate::whereInt_h::WhereClause,
        pub pNew: *mut crate::whereInt_h::WhereLoop,
        pub pOrSet: *mut crate::whereInt_h::WhereOrSet,
        pub bldFlags1: ::core::ffi::c_uchar,
        pub bldFlags2: ::core::ffi::c_uchar,
        pub iPlanLimit: ::core::ffi::c_uint,
    }
    
    
    pub const SQLITE_BLDF1_INDEXED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BLDF1_UNIQUE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_QUERY_PLANNER_LIMIT: ::core::ffi::c_int = 20000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_QUERY_PLANNER_LIMIT_INCR: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct WhereInfo {
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub pTabList: *mut crate::sqliteInt_h::SrcList,
        pub pOrderBy: *mut crate::sqliteInt_h::ExprList,
        pub pResultSet: *mut crate::sqliteInt_h::ExprList,
        pub pSelect: *mut crate::sqliteInt_h::Select,
        pub aiCurOnePass: [::core::ffi::c_int; 2],
        pub iContinue: ::core::ffi::c_int,
        pub iBreak: ::core::ffi::c_int,
        pub savedNQueryLoop: ::core::ffi::c_int,
        pub wctrlFlags: crate::src::fts5::u16_0,
        pub iLimit: crate::sqliteInt_h::LogEst,
        pub nLevel: crate::src::ext::rtree::rtree::u8_0,
        pub nOBSat: crate::sqliteInt_h::i8_0,
        pub eOnePass: crate::src::ext::rtree::rtree::u8_0,
        pub eDistinct: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "bDeferredSeek", ty = "::core::ffi::c_uint", bits = "0..=0")]
        #[bitfield(name = "untestedTerms", ty = "::core::ffi::c_uint", bits = "1..=1")]
        #[bitfield(name = "bOrderedInnerLoop", ty = "::core::ffi::c_uint", bits = "2..=2")]
        #[bitfield(name = "sorted", ty = "::core::ffi::c_uint", bits = "3..=3")]
        #[bitfield(name = "bStarDone", ty = "::core::ffi::c_uint", bits = "4..=4")]
        #[bitfield(name = "bStarUsed", ty = "::core::ffi::c_uint", bits = "5..=5")]
        pub bDeferredSeek_untestedTerms_bOrderedInnerLoop_sorted_bStarDone_bStarUsed: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 1],
        pub nRowOut: crate::sqliteInt_h::LogEst,
        pub iTop: ::core::ffi::c_int,
        pub iEndWhere: ::core::ffi::c_int,
        pub pLoops: *mut crate::whereInt_h::WhereLoop,
        pub pMemToFree: *mut crate::whereInt_h::WhereMemBlock,
        pub revMask: crate::sqliteInt_h::Bitmask,
        pub sWC: crate::whereInt_h::WhereClause,
        pub sMaskSet: crate::whereInt_h::WhereMaskSet,
        pub a: [crate::whereInt_h::WhereLevel; 0],
    }
    
    
    pub const WO_IN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const WO_EQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const WO_LT: ::core::ffi::c_int = crate::whereInt_h::WO_EQ << crate::src::parse::TK_LT_1 - crate::src::parse::TK_EQ;
    
    
    pub const WO_LE: ::core::ffi::c_int = crate::whereInt_h::WO_EQ << crate::src::parse::TK_LE - crate::src::parse::TK_EQ;
    
    
    pub const WO_GT: ::core::ffi::c_int = crate::whereInt_h::WO_EQ << crate::src::parse::TK_GT_1 - crate::src::parse::TK_EQ;
    
    
    pub const WO_GE: ::core::ffi::c_int = crate::whereInt_h::WO_EQ << crate::src::parse::TK_GE - crate::src::parse::TK_EQ;
    
    
    pub const WO_AUX: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const WO_IS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const WO_ISNULL: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const WO_OR: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const WO_AND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const WO_EQUIV: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const WO_ROWVAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const WO_ALL: ::core::ffi::c_int = 0x3fff as ::core::ffi::c_int;
    
    
    pub const WO_SINGLE: ::core::ffi::c_int = 0x1ff as ::core::ffi::c_int;
    
    
    pub const WHERE_COLUMN_EQ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const WHERE_COLUMN_RANGE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const WHERE_COLUMN_IN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const WHERE_COLUMN_NULL: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const WHERE_CONSTRAINT: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
    
    
    pub const WHERE_TOP_LIMIT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const WHERE_BTM_LIMIT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const WHERE_BOTH_LIMIT: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
    
    
    pub const WHERE_IDX_ONLY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const WHERE_IPK: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const WHERE_INDEXED: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const WHERE_VIRTUALTABLE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const WHERE_IN_ABLE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const WHERE_ONEROW: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const WHERE_MULTI_OR: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const WHERE_AUTO_INDEX: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const WHERE_SKIPSCAN: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const WHERE_UNQ_WANTED: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const WHERE_PARTIALIDX: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const WHERE_IN_EARLYOUT: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const WHERE_BIGNULL_SORT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
    
    
    pub const WHERE_IN_SEEKSCAN: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
    
    
    pub const WHERE_TRANSCONS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
    
    
    pub const WHERE_BLOOMFILTER: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
    
    
    pub const WHERE_SELFCULL: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
    
    
    pub const WHERE_COROUTINE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
    
    
    pub const WHERE_EXPRIDX: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
}pub mod vdbeInt_h {
    pub use crate::src::src::vdbesort::VdbeSorter;
    use crate::src::src::window::bft;
    
    pub const SQLITE_MAX_SCHEMA_RETRY: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    
    
    pub type Op = crate::src::src::vdbe::VdbeOp;
    
    
    pub type Bool = ::core::ffi::c_uint;
    
    
    pub const CURTYPE_BTREE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const CURTYPE_BTREE_1: ::core::ffi::c_int = 0;
    
    
    pub const CURTYPE_SORTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const CURTYPE_SORTER_1: ::core::ffi::c_int = 1;
    
    
    pub const CURTYPE_VTAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const CURTYPE_VTAB_1: ::core::ffi::c_int = 2;
    
    
    pub const CURTYPE_PSEUDO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct VdbeCursor {
        pub eCurType: crate::src::ext::rtree::rtree::u8_0,
        pub iDb: crate::sqliteInt_h::i8_0,
        pub nullRow: crate::src::ext::rtree::rtree::u8_0,
        pub deferredMoveto: crate::src::ext::rtree::rtree::u8_0,
        pub isTable: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
        #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
        #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
        #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
        #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
        pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
        pub seekHit: crate::src::fts5::u16_0,
        pub ub: crate::vdbeInt_h::__anon_union_17,
        pub seqCount: crate::src::ext::rtree::rtree::i64_0,
        pub cacheStatus: crate::src::ext::rtree::rtree::u32_0,
        pub seekResult: ::core::ffi::c_int,
        pub pAltCursor: *mut crate::vdbeInt_h::VdbeCursor,
        pub uc: crate::vdbeInt_h::__anon_union_18,
        pub pKeyInfo: *mut crate::sqliteInt_h::KeyInfo,
        pub iHdrOffset: crate::src::ext::rtree::rtree::u32_0,
        pub pgnoRoot: crate::src::src::pager::Pgno,
        pub nField: crate::src::fts5::i16_0,
        pub nHdrParsed: crate::src::fts5::u16_0,
        pub movetoTarget: crate::src::ext::rtree::rtree::i64_0,
        pub aOffset: *mut crate::src::ext::rtree::rtree::u32_0,
        pub aRow: *const crate::src::ext::rtree::rtree::u8_0,
        pub payloadSize: crate::src::ext::rtree::rtree::u32_0,
        pub szRow: crate::src::ext::rtree::rtree::u32_0,
        pub pCache: *mut crate::vdbeInt_h::VdbeTxtBlbCache,
        pub aType: [crate::src::ext::rtree::rtree::u32_0; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_17 {
        pub pBtx: *mut crate::btreeInt_h::Btree,
        pub aAltMap: *mut crate::src::ext::rtree::rtree::u32_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_18 {
        pub pCursor: *mut crate::btreeInt_h::BtCursor,
        pub pVCur: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
        pub pSorter: *mut crate::vdbeInt_h::VdbeSorter,
    }
    
    
    pub const CACHE_STALE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VdbeTxtBlbCache {
        pub pCValue: *mut ::core::ffi::c_char,
        pub iOffset: crate::src::ext::rtree::rtree::i64_0,
        pub iCol: ::core::ffi::c_int,
        pub cacheStatus: crate::src::ext::rtree::rtree::u32_0,
        pub colCacheCtr: crate::src::ext::rtree::rtree::u32_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VdbeFrame {
        pub v: *mut crate::vdbeInt_h::Vdbe,
        pub pParent: *mut crate::vdbeInt_h::VdbeFrame,
        pub aOp: *mut crate::vdbeInt_h::Op,
        pub aMem: *mut crate::src::src::vdbe::Mem,
        pub apCsr: *mut *mut crate::vdbeInt_h::VdbeCursor,
        pub aOnce: *mut crate::src::ext::rtree::rtree::u8_0,
        pub token: *mut ::core::ffi::c_void,
        pub lastRowid: crate::src::ext::rtree::rtree::i64_0,
        pub pAuxData: *mut crate::vdbeInt_h::AuxData,
        pub nCursor: ::core::ffi::c_int,
        pub pc: ::core::ffi::c_int,
        pub nOp: ::core::ffi::c_int,
        pub nMem: ::core::ffi::c_int,
        pub nChildMem: ::core::ffi::c_int,
        pub nChildCsr: ::core::ffi::c_int,
        pub nChange: crate::src::ext::rtree::rtree::i64_0,
        pub nDbChange: crate::src::ext::rtree::rtree::i64_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_value {
        pub u: crate::vdbeInt_h::MemValue,
        pub z: *mut ::core::ffi::c_char,
        pub n: ::core::ffi::c_int,
        pub flags: crate::src::fts5::u16_0,
        pub enc: crate::src::ext::rtree::rtree::u8_0,
        pub eSubtype: crate::src::ext::rtree::rtree::u8_0,
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub szMalloc: ::core::ffi::c_int,
        pub uTemp: crate::src::ext::rtree::rtree::u32_0,
        pub zMalloc: *mut ::core::ffi::c_char,
        pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union MemValue {
        pub r: ::core::ffi::c_double,
        pub i: crate::src::ext::rtree::rtree::i64_0,
        pub nZero: ::core::ffi::c_int,
        pub zPType: *const ::core::ffi::c_char,
        pub pDef: *mut crate::sqliteInt_h::FuncDef,
    }
    
    
    pub const MEMCELLSIZE: ::core::ffi::c_ulong = 24 as ::core::ffi::c_ulong;
    
    
    pub const MEM_Undefined: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const MEM_Str: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const MEM_Int: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const MEM_Real: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const MEM_Blob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const MEM_IntReal: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const MEM_AffMask: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
    
    
    pub const MEM_FromBind: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const MEM_Cleared: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const MEM_Term: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const MEM_Subtype: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;
    
    
    pub const MEM_Dyn: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const MEM_Static: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const MEM_Ephem: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const MEM_Agg: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AuxData {
        pub iAuxOp: ::core::ffi::c_int,
        pub iAuxArg: ::core::ffi::c_int,
        pub pAux: *mut ::core::ffi::c_void,
        pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub pNextAux: *mut crate::vdbeInt_h::AuxData,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_context {
        pub pOut: *mut crate::src::src::vdbe::Mem,
        pub pFunc: *mut crate::sqliteInt_h::FuncDef,
        pub pMem: *mut crate::src::src::vdbe::Mem,
        pub pVdbe: *mut crate::vdbeInt_h::Vdbe,
        pub iOp: ::core::ffi::c_int,
        pub isError: ::core::ffi::c_int,
        pub enc: crate::src::ext::rtree::rtree::u8_0,
        pub skipFlag: crate::src::ext::rtree::rtree::u8_0,
        pub argc: crate::src::fts5::u16_0,
        pub argv: [*mut crate::vdbeInt_h::sqlite3_value; 0],
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct Vdbe {
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub ppVPrev: *mut *mut crate::vdbeInt_h::Vdbe,
        pub pVNext: *mut crate::vdbeInt_h::Vdbe,
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub nVar: crate::sqliteInt_h::ynVar,
        pub nMem: ::core::ffi::c_int,
        pub nCursor: ::core::ffi::c_int,
        pub cacheCtr: crate::src::ext::rtree::rtree::u32_0,
        pub pc: ::core::ffi::c_int,
        pub rc: ::core::ffi::c_int,
        pub nChange: crate::src::ext::rtree::rtree::i64_0,
        pub iStatement: ::core::ffi::c_int,
        pub iCurrentTime: crate::src::ext::rtree::rtree::i64_0,
        pub nFkConstraint: crate::src::ext::rtree::rtree::i64_0,
        pub nStmtDefCons: crate::src::ext::rtree::rtree::i64_0,
        pub nStmtDefImmCons: crate::src::ext::rtree::rtree::i64_0,
        pub aMem: *mut crate::src::src::vdbe::Mem,
        pub apArg: *mut *mut crate::src::src::vdbe::Mem,
        pub apCsr: *mut *mut crate::vdbeInt_h::VdbeCursor,
        pub aVar: *mut crate::src::src::vdbe::Mem,
        pub aOp: *mut crate::vdbeInt_h::Op,
        pub nOp: ::core::ffi::c_int,
        pub nOpAlloc: ::core::ffi::c_int,
        pub aColName: *mut crate::src::src::vdbe::Mem,
        pub pResultRow: *mut crate::src::src::vdbe::Mem,
        pub zErrMsg: *mut ::core::ffi::c_char,
        pub pVList: *mut crate::sqliteInt_h::VList,
        pub startTime: crate::src::ext::rtree::rtree::i64_0,
        pub nResColumn: crate::src::fts5::u16_0,
        pub nResAlloc: crate::src::fts5::u16_0,
        pub errorAction: crate::src::ext::rtree::rtree::u8_0,
        pub minWriteFileFormat: crate::src::ext::rtree::rtree::u8_0,
        pub prepFlags: crate::src::ext::rtree::rtree::u8_0,
        pub eVdbeState: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
        #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
        #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
        #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
        #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
        #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
        #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
        pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 2],
        pub btreeMask: crate::sqliteInt_h::yDbMask,
        pub lockMask: crate::sqliteInt_h::yDbMask,
        pub aCounter: [crate::src::ext::rtree::rtree::u32_0; 9],
        pub zSql: *mut ::core::ffi::c_char,
        pub pFree: *mut ::core::ffi::c_void,
        pub pFrame: *mut crate::vdbeInt_h::VdbeFrame,
        pub pDelFrame: *mut crate::vdbeInt_h::VdbeFrame,
        pub nFrame: ::core::ffi::c_int,
        pub expmask: crate::src::ext::rtree::rtree::u32_0,
        pub pProgram: *mut crate::src::src::vdbe::SubProgram,
        pub pAuxData: *mut crate::vdbeInt_h::AuxData,
    }
    
    
    pub const VDBE_INIT_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const VDBE_READY_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const VDBE_RUN_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const VDBE_HALT_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PreUpdate {
        pub v: *mut crate::vdbeInt_h::Vdbe,
        pub pCsr: *mut crate::vdbeInt_h::VdbeCursor,
        pub op: ::core::ffi::c_int,
        pub aRecord: *mut crate::src::ext::rtree::rtree::u8_0,
        pub pKeyinfo: *mut crate::sqliteInt_h::KeyInfo,
        pub pUnpacked: *mut crate::sqliteInt_h::UnpackedRecord,
        pub pNewUnpacked: *mut crate::sqliteInt_h::UnpackedRecord,
        pub iNewReg: ::core::ffi::c_int,
        pub iBlobWrite: ::core::ffi::c_int,
        pub iKey1: crate::src::ext::rtree::rtree::i64_0,
        pub iKey2: crate::src::ext::rtree::rtree::i64_0,
        pub oldipk: crate::src::src::vdbe::Mem,
        pub aNew: *mut crate::src::src::vdbe::Mem,
        pub pTab: *mut crate::sqliteInt_h::Table,
        pub pPk: *mut crate::sqliteInt_h::Index,
        pub apDflt: *mut *mut crate::vdbeInt_h::sqlite3_value,
        pub uKey: crate::vdbeInt_h::__anon_struct_10,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_10 {
        pub keyinfoSpace: [crate::src::ext::rtree::rtree::u8_0; 32],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct ValueList {
        pub pCsr: *mut crate::btreeInt_h::BtCursor,
        pub pOut: *mut crate::vdbeInt_h::sqlite3_value,
    }
}pub mod keywordhash_h {
    pub const SQLITE_N_KEYWORD:  ::core::ffi::c_int =
         147 as ::core::ffi::c_int;
}pub mod vxworks_h {
    pub const OS_VXWORKS:  ::core::ffi::c_int =  0 as ::core::ffi::c_int;
}pub mod sqliteLimit_h {
    pub const SQLITE_MAX_LENGTH:  ::core::ffi::c_int =
         1000000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MIN_LENGTH: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_COLUMN: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_SQL_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_EXPR_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_COMPOUND_SELECT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_VDBE_OP: ::core::ffi::c_int = 250000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_FUNCTION_ARG: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = -(2000 as ::core::ffi::c_int);
    
    
    pub const SQLITE_DEFAULT_WAL_AUTOCHECKPOINT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_ATTACHED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_VARIABLE_NUMBER: ::core::ffi::c_int = 32766 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_PAGE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_PAGE_COUNT: ::core::ffi::c_uint = 0xfffffffe as ::core::ffi::c_uint;
    
    
    pub const SQLITE_MAX_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_TRIGGER_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
}pub mod internal {
    pub type __builtin_va_list =  [crate::internal::__va_list_tag; 1];
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
    
    
    pub const SQLITE_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
    
    
    pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_THREADSAFE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const __SIZEOF_POINTER__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}pub mod btreeInt_h {
    pub const SQLITE_FILE_HEADER:  [::core::ffi::c_char; 16] =
         unsafe {
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SQLite format 3\0")
    };
    
    
    pub const PTF_INTKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const PTF_ZERODATA: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const PTF_LEAFDATA: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const PTF_LEAF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct MemPage {
        pub isInit: crate::src::ext::rtree::rtree::u8_0,
        pub intKey: crate::src::ext::rtree::rtree::u8_0,
        pub intKeyLeaf: crate::src::ext::rtree::rtree::u8_0,
        pub pgno: crate::src::src::pager::Pgno,
        pub leaf: crate::src::ext::rtree::rtree::u8_0,
        pub hdrOffset: crate::src::ext::rtree::rtree::u8_0,
        pub childPtrSize: crate::src::ext::rtree::rtree::u8_0,
        pub max1bytePayload: crate::src::ext::rtree::rtree::u8_0,
        pub nOverflow: crate::src::ext::rtree::rtree::u8_0,
        pub maxLocal: crate::src::fts5::u16_0,
        pub minLocal: crate::src::fts5::u16_0,
        pub cellOffset: crate::src::fts5::u16_0,
        pub nFree: ::core::ffi::c_int,
        pub nCell: crate::src::fts5::u16_0,
        pub maskPage: crate::src::fts5::u16_0,
        pub aiOvfl: [crate::src::fts5::u16_0; 4],
        pub apOvfl: [*mut crate::src::ext::rtree::rtree::u8_0; 4],
        pub pBt: *mut crate::btreeInt_h::BtShared,
        pub aData: *mut crate::src::ext::rtree::rtree::u8_0,
        pub aDataEnd: *mut crate::src::ext::rtree::rtree::u8_0,
        pub aCellIdx: *mut crate::src::ext::rtree::rtree::u8_0,
        pub aDataOfst: *mut crate::src::ext::rtree::rtree::u8_0,
        pub pDbPage: *mut crate::src::src::pager::DbPage,
        pub xCellSize: Option<unsafe extern "C" fn(*mut crate::btreeInt_h::MemPage, *mut crate::src::ext::rtree::rtree::u8_0) -> crate::src::fts5::u16_0>,
        pub xParseCell: Option<unsafe extern "C" fn(*mut crate::btreeInt_h::MemPage, *mut crate::src::ext::rtree::rtree::u8_0, *mut crate::btreeInt_h::CellInfo) -> ()>,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct BtLock {
        pub pBtree: *mut crate::btreeInt_h::Btree,
        pub iTable: crate::src::src::pager::Pgno,
        pub eLock: crate::src::ext::rtree::rtree::u8_0,
        pub pNext: *mut crate::btreeInt_h::BtLock,
    }
    
    
    pub const READ_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const WRITE_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Btree {
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub pBt: *mut crate::btreeInt_h::BtShared,
        pub inTrans: crate::src::ext::rtree::rtree::u8_0,
        pub sharable: crate::src::ext::rtree::rtree::u8_0,
        pub locked: crate::src::ext::rtree::rtree::u8_0,
        pub hasIncrblobCur: crate::src::ext::rtree::rtree::u8_0,
        pub wantToLock: ::core::ffi::c_int,
        pub nBackup: ::core::ffi::c_int,
        pub iBDataVersion: crate::src::ext::rtree::rtree::u32_0,
        pub pNext: *mut crate::btreeInt_h::Btree,
        pub pPrev: *mut crate::btreeInt_h::Btree,
        pub lock: crate::btreeInt_h::BtLock,
    }
    
    
    pub const TRANS_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const TRANS_READ: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const TRANS_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct BtShared {
        pub pPager: *mut crate::src::src::pager::Pager,
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub pCursor: *mut crate::btreeInt_h::BtCursor,
        pub pPage1: *mut crate::btreeInt_h::MemPage,
        pub openFlags: crate::src::ext::rtree::rtree::u8_0,
        pub autoVacuum: crate::src::ext::rtree::rtree::u8_0,
        pub incrVacuum: crate::src::ext::rtree::rtree::u8_0,
        pub bDoTruncate: crate::src::ext::rtree::rtree::u8_0,
        pub inTransaction: crate::src::ext::rtree::rtree::u8_0,
        pub max1bytePayload: crate::src::ext::rtree::rtree::u8_0,
        pub nReserveWanted: crate::src::ext::rtree::rtree::u8_0,
        pub btsFlags: crate::src::fts5::u16_0,
        pub maxLocal: crate::src::fts5::u16_0,
        pub minLocal: crate::src::fts5::u16_0,
        pub maxLeaf: crate::src::fts5::u16_0,
        pub minLeaf: crate::src::fts5::u16_0,
        pub pageSize: crate::src::ext::rtree::rtree::u32_0,
        pub usableSize: crate::src::ext::rtree::rtree::u32_0,
        pub nTransaction: ::core::ffi::c_int,
        pub nPage: crate::src::ext::rtree::rtree::u32_0,
        pub pSchema: *mut ::core::ffi::c_void,
        pub xFreeSchema: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
        pub pHasContent: *mut crate::src::src::bitvec::Bitvec,
        pub nRef: ::core::ffi::c_int,
        pub pNext: *mut crate::btreeInt_h::BtShared,
        pub pLock: *mut crate::btreeInt_h::BtLock,
        pub pWriter: *mut crate::btreeInt_h::Btree,
        pub pTmpSpace: *mut crate::src::ext::rtree::rtree::u8_0,
        pub nPreformatSize: ::core::ffi::c_int,
    }
    
    
    pub const BTS_READ_ONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const BTS_PAGESIZE_FIXED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const BTS_SECURE_DELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const BTS_FAST_SECURE: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    
    
    pub const BTS_INITIALLY_EMPTY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const BTS_NO_WAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const BTS_EXCLUSIVE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const BTS_PENDING: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct CellInfo {
        pub nKey: crate::src::ext::rtree::rtree::i64_0,
        pub pPayload: *mut crate::src::ext::rtree::rtree::u8_0,
        pub nPayload: crate::src::ext::rtree::rtree::u32_0,
        pub nLocal: crate::src::fts5::u16_0,
        pub nSize: crate::src::fts5::u16_0,
    }
    
    
    pub const BTCURSOR_MAX_DEPTH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct BtCursor {
        pub eState: crate::src::ext::rtree::rtree::u8_0,
        pub curFlags: crate::src::ext::rtree::rtree::u8_0,
        pub curPagerFlags: crate::src::ext::rtree::rtree::u8_0,
        pub hints: crate::src::ext::rtree::rtree::u8_0,
        pub skipNext: ::core::ffi::c_int,
        pub pBtree: *mut crate::btreeInt_h::Btree,
        pub aOverflow: *mut crate::src::src::pager::Pgno,
        pub pKey: *mut ::core::ffi::c_void,
        pub pBt: *mut crate::btreeInt_h::BtShared,
        pub pNext: *mut crate::btreeInt_h::BtCursor,
        pub info: crate::btreeInt_h::CellInfo,
        pub nKey: crate::src::ext::rtree::rtree::i64_0,
        pub pgnoRoot: crate::src::src::pager::Pgno,
        pub iPage: crate::sqliteInt_h::i8_0,
        pub curIntKey: crate::src::ext::rtree::rtree::u8_0,
        pub ix: crate::src::fts5::u16_0,
        pub aiIdx: [crate::src::fts5::u16_0; 19],
        pub pKeyInfo: *mut crate::sqliteInt_h::KeyInfo,
        pub pPage: *mut crate::btreeInt_h::MemPage,
        pub apPage: [*mut crate::btreeInt_h::MemPage; 19],
    }
    
    
    pub const BTCF_WriteFlag: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const BTCF_ValidNKey: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const BTCF_ValidOvfl: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const BTCF_AtLast: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const BTCF_Incrblob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const BTCF_Multiple: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const BTCF_Pinned: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const CURSOR_VALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const CURSOR_INVALID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const CURSOR_SKIPNEXT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const CURSOR_REQUIRESEEK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const CURSOR_FAULT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const PTRMAP_ROOTPAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const PTRMAP_FREEPAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const PTRMAP_OVERFLOW1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const PTRMAP_OVERFLOW2: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const PTRMAP_BTREE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct IntegrityCk {
        pub pBt: *mut crate::btreeInt_h::BtShared,
        pub pPager: *mut crate::src::src::pager::Pager,
        pub aPgRef: *mut crate::src::ext::rtree::rtree::u8_0,
        pub nCkPage: crate::src::src::pager::Pgno,
        pub mxErr: ::core::ffi::c_int,
        pub nErr: ::core::ffi::c_int,
        pub rc: ::core::ffi::c_int,
        pub nStep: crate::src::ext::rtree::rtree::u32_0,
        pub zPfx: *const ::core::ffi::c_char,
        pub v0: crate::src::src::pager::Pgno,
        pub v1: crate::src::src::pager::Pgno,
        pub v2: ::core::ffi::c_int,
        pub errMsg: crate::sqliteInt_h::StrAccum,
        pub heap: *mut crate::src::ext::rtree::rtree::u32_0,
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub nRow: crate::src::ext::rtree::rtree::i64_0,
    }
}pub mod opcodes_h {
    pub const OP_Savepoint:  ::core::ffi::c_int =  0 as ::core::ffi::c_int;
    
    
    pub const OP_Savepoint_1: ::core::ffi::c_int = 0;
    
    
    pub const OP_AutoCommit: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const OP_AutoCommit_1: ::core::ffi::c_int = 1;
    
    
    pub const OP_Transaction: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const OP_Transaction_1: ::core::ffi::c_int = 2;
    
    
    pub const OP_Checkpoint: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const OP_Checkpoint_1: ::core::ffi::c_int = 3;
    
    
    pub const OP_JournalMode: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const OP_JournalMode_1: ::core::ffi::c_int = 4;
    
    
    pub const OP_Vacuum: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const OP_Vacuum_1: ::core::ffi::c_int = 5;
    
    
    pub const OP_VFilter: ::core::ffi::c_int = 6;
    
    
    pub const OP_VFilter_1: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const OP_VUpdate: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const OP_VUpdate_1: ::core::ffi::c_int = 7;
    
    
    pub const OP_Init: ::core::ffi::c_int = 8;
    
    
    pub const OP_Init_1: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const OP_Goto_1: ::core::ffi::c_int = 9;
    
    
    pub const OP_Gosub: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const OP_Gosub_1: ::core::ffi::c_int = 10;
    
    
    pub const OP_InitCoroutine: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const OP_InitCoroutine_1: ::core::ffi::c_int = 11;
    
    
    pub const OP_Yield: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    
    pub const OP_Yield_1: ::core::ffi::c_int = 12;
    
    
    pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    
    
    pub const OP_MustBeInt_1: ::core::ffi::c_int = 13;
    
    
    pub const OP_Jump: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    
    
    pub const OP_Jump_1: ::core::ffi::c_int = 14;
    
    
    pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    
    
    pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    
    
    pub const OP_If_1: ::core::ffi::c_int = 16;
    
    
    pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
    
    
    pub const OP_IfNot_1: ::core::ffi::c_int = 17;
    
    
    pub const OP_IsType: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    
    
    pub const OP_IsType_1: ::core::ffi::c_int = 18;
    
    
    pub const OP_Not: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
    
    
    pub const OP_Not_1: ::core::ffi::c_int = 19;
    
    
    pub const OP_IfNullRow: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    
    pub const OP_IfNullRow_1: ::core::ffi::c_int = 20;
    
    
    pub const OP_SeekLT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
    
    
    pub const OP_SeekLE: ::core::ffi::c_int = 22;
    
    
    pub const OP_SeekLE_1: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    
    
    pub const OP_SeekGE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    
    
    pub const OP_SeekGT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
    
    
    pub const OP_IfNotOpen: ::core::ffi::c_int = 25;
    
    
    pub const OP_IfNotOpen_1: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    
    
    pub const OP_IfNoHope: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
    
    
    pub const OP_NoConflict: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
    
    
    pub const OP_NotFound: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    
    
    pub const OP_NotFound_1: ::core::ffi::c_int = 28;
    
    
    pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
    
    
    pub const OP_SeekRowid: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    
    
    pub const OP_SeekRowid_1: ::core::ffi::c_int = 30;
    
    
    pub const OP_NotExists: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
    
    
    pub const OP_NotExists_1: ::core::ffi::c_int = 31;
    
    
    pub const OP_Last: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    
    
    pub const OP_Last_1: ::core::ffi::c_int = 32;
    
    
    pub const OP_IfSizeBetween: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
    
    
    pub const OP_IfSizeBetween_1: ::core::ffi::c_int = 33;
    
    
    pub const OP_SorterSort: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
    
    
    pub const OP_SorterSort_1: ::core::ffi::c_int = 34;
    
    
    pub const OP_Sort: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    
    
    pub const OP_Sort_1: ::core::ffi::c_int = 35;
    
    
    pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    
    
    pub const OP_Rewind_1: ::core::ffi::c_int = 36;
    
    
    pub const OP_IfEmpty: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
    
    
    pub const OP_IfEmpty_1: ::core::ffi::c_int = 37;
    
    
    pub const OP_SorterNext: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
    
    
    pub const OP_SorterNext_1: ::core::ffi::c_int = 38;
    
    
    pub const OP_Prev: ::core::ffi::c_int = 39;
    
    
    pub const OP_Prev_1: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
    
    
    pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    
    
    pub const OP_Next_1: ::core::ffi::c_int = 40;
    
    
    pub const OP_IdxLE: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
    
    
    pub const OP_IdxLE_1: ::core::ffi::c_int = 41;
    
    
    pub const OP_IdxGT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
    
    
    pub const OP_IdxGT_1: ::core::ffi::c_int = 42;
    
    
    pub const OP_Or: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
    
    
    pub const OP_Or_1: ::core::ffi::c_int = 43;
    
    
    pub const OP_And: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
    
    
    pub const OP_IdxLT: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
    
    
    pub const OP_IdxGE: ::core::ffi::c_int = 46;
    
    
    pub const OP_IdxGE_1: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
    
    
    pub const OP_RowSetRead: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
    
    
    pub const OP_RowSetRead_1: ::core::ffi::c_int = 47;
    
    
    pub const OP_RowSetTest: ::core::ffi::c_int = 48;
    
    
    pub const OP_RowSetTest_1: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
    
    
    pub const OP_Program: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
    
    
    pub const OP_Program_1: ::core::ffi::c_int = 49;
    
    
    pub const OP_FkIfZero: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    
    
    pub const OP_FkIfZero_1: ::core::ffi::c_int = 50;
    
    
    pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
    
    
    pub const OP_IsNull_1: ::core::ffi::c_int = 51;
    
    
    pub const OP_NotNull: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
    
    
    pub const OP_NotNull_1: ::core::ffi::c_int = 52;
    
    
    pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
    
    
    pub const OP_Ne_1: ::core::ffi::c_int = 53;
    
    
    pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
    
    
    pub const OP_Eq_1: ::core::ffi::c_int = 54;
    
    
    pub const OP_Gt: ::core::ffi::c_int = 55;
    
    
    pub const OP_Gt_1: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
    
    
    pub const OP_Le: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
    
    
    pub const OP_Le_1: ::core::ffi::c_int = 56;
    
    
    pub const OP_Lt: ::core::ffi::c_int = 57;
    
    
    pub const OP_Lt_1: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
    
    
    pub const OP_Ge: ::core::ffi::c_int = 58;
    
    
    pub const OP_Ge_1: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
    
    
    pub const OP_ElseEq: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
    
    
    pub const OP_ElseEq_1: ::core::ffi::c_int = 59;
    
    
    pub const OP_IfPos: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
    
    
    pub const OP_IfPos_1: ::core::ffi::c_int = 60;
    
    
    pub const OP_IfNotZero: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
    
    
    pub const OP_IfNotZero_1: ::core::ffi::c_int = 61;
    
    
    pub const OP_DecrJumpZero: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
    
    
    pub const OP_DecrJumpZero_1: ::core::ffi::c_int = 62;
    
    
    pub const OP_IncrVacuum: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
    
    
    pub const OP_IncrVacuum_1: ::core::ffi::c_int = 63;
    
    
    pub const OP_VNext: ::core::ffi::c_int = 64;
    
    
    pub const OP_VNext_1: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    
    
    pub const OP_Filter: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
    
    
    pub const OP_Filter_1: ::core::ffi::c_int = 65;
    
    
    pub const OP_PureFunc: ::core::ffi::c_int = 66;
    
    
    pub const OP_PureFunc_1: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
    
    
    pub const OP_Function: ::core::ffi::c_int = 67;
    
    
    pub const OP_Function_1: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
    
    
    pub const OP_Return: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
    
    
    pub const OP_Return_1: ::core::ffi::c_int = 68;
    
    
    pub const OP_EndCoroutine: ::core::ffi::c_int = 69;
    
    
    pub const OP_EndCoroutine_1: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
    
    
    pub const OP_HaltIfNull: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
    
    
    pub const OP_HaltIfNull_1: ::core::ffi::c_int = 70;
    
    
    pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
    
    
    pub const OP_Halt_1: ::core::ffi::c_int = 71;
    
    
    pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
    
    
    pub const OP_Integer_1: ::core::ffi::c_int = 72;
    
    
    pub const OP_Int64: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
    
    
    pub const OP_Int64_1: ::core::ffi::c_int = 73;
    
    
    pub const OP_String: ::core::ffi::c_int = 74;
    
    
    pub const OP_BeginSubrtn: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
    
    
    pub const OP_BeginSubrtn_1: ::core::ffi::c_int = 75;
    
    
    pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
    
    
    pub const OP_Null_1: ::core::ffi::c_int = 76;
    
    
    pub const OP_SoftNull: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
    
    
    pub const OP_SoftNull_1: ::core::ffi::c_int = 77;
    
    
    pub const OP_Blob: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
    
    
    pub const OP_Blob_1: ::core::ffi::c_int = 78;
    
    
    pub const OP_Variable: ::core::ffi::c_int = 79 as ::core::ffi::c_int;
    
    
    pub const OP_Variable_1: ::core::ffi::c_int = 79;
    
    
    pub const OP_Move: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
    
    
    pub const OP_Move_1: ::core::ffi::c_int = 80;
    
    
    pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
    
    
    pub const OP_Copy_1: ::core::ffi::c_int = 81;
    
    
    pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
    
    
    pub const OP_SCopy_1: ::core::ffi::c_int = 82;
    
    
    pub const OP_IntCopy: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
    
    
    pub const OP_IntCopy_1: ::core::ffi::c_int = 83;
    
    
    pub const OP_FkCheck: ::core::ffi::c_int = 84 as ::core::ffi::c_int;
    
    
    pub const OP_FkCheck_1: ::core::ffi::c_int = 84;
    
    
    pub const OP_ResultRow: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
    
    
    pub const OP_ResultRow_1: ::core::ffi::c_int = 85;
    
    
    pub const OP_CollSeq: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
    
    
    pub const OP_CollSeq_1: ::core::ffi::c_int = 86;
    
    
    pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
    
    
    pub const OP_AddImm_1: ::core::ffi::c_int = 87;
    
    
    pub const OP_RealAffinity: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
    
    
    pub const OP_RealAffinity_1: ::core::ffi::c_int = 88;
    
    
    pub const OP_Cast: ::core::ffi::c_int = 89 as ::core::ffi::c_int;
    
    
    pub const OP_Cast_1: ::core::ffi::c_int = 89;
    
    
    pub const OP_Permutation: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
    
    
    pub const OP_Permutation_1: ::core::ffi::c_int = 90;
    
    
    pub const OP_Compare: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
    
    
    pub const OP_Compare_1: ::core::ffi::c_int = 91;
    
    
    pub const OP_IsTrue: ::core::ffi::c_int = 92 as ::core::ffi::c_int;
    
    
    pub const OP_IsTrue_1: ::core::ffi::c_int = 92;
    
    
    pub const OP_ZeroOrNull: ::core::ffi::c_int = 93 as ::core::ffi::c_int;
    
    
    pub const OP_ZeroOrNull_1: ::core::ffi::c_int = 93;
    
    
    pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
    
    
    pub const OP_Column_1: ::core::ffi::c_int = 95;
    
    
    pub const OP_TypeCheck: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
    
    
    pub const OP_TypeCheck_1: ::core::ffi::c_int = 96;
    
    
    pub const OP_Affinity: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
    
    
    pub const OP_Affinity_1: ::core::ffi::c_int = 97;
    
    
    pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
    
    
    pub const OP_MakeRecord_1: ::core::ffi::c_int = 98;
    
    
    pub const OP_Count: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    
    
    pub const OP_Count_1: ::core::ffi::c_int = 99;
    
    
    pub const OP_ReadCookie: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    
    
    pub const OP_ReadCookie_1: ::core::ffi::c_int = 100;
    
    
    pub const OP_SetCookie: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
    
    
    pub const OP_SetCookie_1: ::core::ffi::c_int = 101;
    
    
    pub const OP_ReopenIdx: ::core::ffi::c_int = 102;
    
    
    pub const OP_ReopenIdx_1: ::core::ffi::c_int = 102 as ::core::ffi::c_int;
    
    
    pub const OP_BitAnd: ::core::ffi::c_int = 103 as ::core::ffi::c_int;
    
    
    pub const OP_BitOr: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
    
    
    pub const OP_ShiftLeft: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
    
    
    pub const OP_ShiftRight: ::core::ffi::c_int = 106;
    
    
    pub const OP_Add: ::core::ffi::c_int = 107;
    
    
    pub const OP_Add_1: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
    
    
    pub const OP_Subtract: ::core::ffi::c_int = 108 as ::core::ffi::c_int;
    
    
    pub const OP_Subtract_1: ::core::ffi::c_int = 108;
    
    
    pub const OP_Multiply: ::core::ffi::c_int = 109;
    
    
    pub const OP_Divide: ::core::ffi::c_int = 110;
    
    
    pub const OP_Remainder: ::core::ffi::c_int = 111;
    
    
    pub const OP_Concat: ::core::ffi::c_int = 112 as ::core::ffi::c_int;
    
    
    pub const OP_Concat_1: ::core::ffi::c_int = 112;
    
    
    pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
    
    
    pub const OP_OpenRead_1: ::core::ffi::c_int = 113;
    
    
    pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
    
    
    pub const OP_BitNot: ::core::ffi::c_int = 115;
    
    
    pub const OP_OpenDup: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
    
    
    pub const OP_OpenDup_1: ::core::ffi::c_int = 116;
    
    
    pub const OP_OpenAutoindex: ::core::ffi::c_int = 117;
    
    
    pub const OP_OpenAutoindex_1: ::core::ffi::c_int = 117 as ::core::ffi::c_int;
    
    
    pub const OP_String8: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
    
    
    pub const OP_String8_1: ::core::ffi::c_int = 118;
    
    
    pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
    
    
    pub const OP_OpenEphemeral_1: ::core::ffi::c_int = 119;
    
    
    pub const OP_SorterOpen: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
    
    
    pub const OP_SorterOpen_1: ::core::ffi::c_int = 120;
    
    
    pub const OP_SequenceTest: ::core::ffi::c_int = 121 as ::core::ffi::c_int;
    
    
    pub const OP_SequenceTest_1: ::core::ffi::c_int = 121;
    
    
    pub const OP_OpenPseudo: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
    
    
    pub const OP_OpenPseudo_1: ::core::ffi::c_int = 122;
    
    
    pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
    
    
    pub const OP_Close_1: ::core::ffi::c_int = 123;
    
    
    pub const OP_SeekScan: ::core::ffi::c_int = 125;
    
    
    pub const OP_SeekScan_1: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
    
    
    pub const OP_SeekHit: ::core::ffi::c_int = 126;
    
    
    pub const OP_SeekHit_1: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
    
    
    pub const OP_Sequence: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
    
    
    pub const OP_Sequence_1: ::core::ffi::c_int = 127;
    
    
    pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
    
    
    pub const OP_NewRowid_1: ::core::ffi::c_int = 128;
    
    
    pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
    
    
    pub const OP_Insert_1: ::core::ffi::c_int = 129;
    
    
    pub const OP_RowCell: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
    
    
    pub const OP_RowCell_1: ::core::ffi::c_int = 130;
    
    
    pub const OP_Delete: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
    
    
    pub const OP_Delete_1: ::core::ffi::c_int = 131;
    
    
    pub const OP_ResetCount: ::core::ffi::c_int = 132 as ::core::ffi::c_int;
    
    
    pub const OP_ResetCount_1: ::core::ffi::c_int = 132;
    
    
    pub const OP_SorterCompare: ::core::ffi::c_int = 133 as ::core::ffi::c_int;
    
    
    pub const OP_SorterCompare_1: ::core::ffi::c_int = 133;
    
    
    pub const OP_SorterData: ::core::ffi::c_int = 134 as ::core::ffi::c_int;
    
    
    pub const OP_SorterData_1: ::core::ffi::c_int = 134;
    
    
    pub const OP_RowData: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
    
    
    pub const OP_RowData_1: ::core::ffi::c_int = 135;
    
    
    pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
    
    
    pub const OP_Rowid_1: ::core::ffi::c_int = 136;
    
    
    pub const OP_NullRow: ::core::ffi::c_int = 137 as ::core::ffi::c_int;
    
    
    pub const OP_NullRow_1: ::core::ffi::c_int = 137;
    
    
    pub const OP_SeekEnd: ::core::ffi::c_int = 138 as ::core::ffi::c_int;
    
    
    pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
    
    
    pub const OP_IdxInsert_1: ::core::ffi::c_int = 139;
    
    
    pub const OP_SorterInsert: ::core::ffi::c_int = 140 as ::core::ffi::c_int;
    
    
    pub const OP_SorterInsert_1: ::core::ffi::c_int = 140;
    
    
    pub const OP_IdxDelete: ::core::ffi::c_int = 141 as ::core::ffi::c_int;
    
    
    pub const OP_IdxDelete_1: ::core::ffi::c_int = 141;
    
    
    pub const OP_DeferredSeek: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
    
    
    pub const OP_IdxRowid: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
    
    
    pub const OP_IdxRowid_1: ::core::ffi::c_int = 143;
    
    
    pub const OP_FinishSeek: ::core::ffi::c_int = 144 as ::core::ffi::c_int;
    
    
    pub const OP_FinishSeek_1: ::core::ffi::c_int = 144;
    
    
    pub const OP_Destroy: ::core::ffi::c_int = 145 as ::core::ffi::c_int;
    
    
    pub const OP_Destroy_1: ::core::ffi::c_int = 145;
    
    
    pub const OP_Clear: ::core::ffi::c_int = 146 as ::core::ffi::c_int;
    
    
    pub const OP_Clear_1: ::core::ffi::c_int = 146;
    
    
    pub const OP_ResetSorter: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
    
    
    pub const OP_ResetSorter_1: ::core::ffi::c_int = 147;
    
    
    pub const OP_CreateBtree: ::core::ffi::c_int = 148 as ::core::ffi::c_int;
    
    
    pub const OP_CreateBtree_1: ::core::ffi::c_int = 148;
    
    
    pub const OP_SqlExec: ::core::ffi::c_int = 149 as ::core::ffi::c_int;
    
    
    pub const OP_SqlExec_1: ::core::ffi::c_int = 149;
    
    
    pub const OP_ParseSchema: ::core::ffi::c_int = 150;
    
    
    pub const OP_ParseSchema_1: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
    
    
    pub const OP_LoadAnalysis: ::core::ffi::c_int = 151 as ::core::ffi::c_int;
    
    
    pub const OP_LoadAnalysis_1: ::core::ffi::c_int = 151;
    
    
    pub const OP_DropTable: ::core::ffi::c_int = 152 as ::core::ffi::c_int;
    
    
    pub const OP_DropTable_1: ::core::ffi::c_int = 152;
    
    
    pub const OP_DropIndex: ::core::ffi::c_int = 153 as ::core::ffi::c_int;
    
    
    pub const OP_DropIndex_1: ::core::ffi::c_int = 153;
    
    
    pub const OP_Real: ::core::ffi::c_int = 154 as ::core::ffi::c_int;
    
    
    pub const OP_Real_1: ::core::ffi::c_int = 154;
    
    
    pub const OP_DropTrigger: ::core::ffi::c_int = 155 as ::core::ffi::c_int;
    
    
    pub const OP_DropTrigger_1: ::core::ffi::c_int = 155;
    
    
    pub const OP_IntegrityCk: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
    
    
    pub const OP_IntegrityCk_1: ::core::ffi::c_int = 156;
    
    
    pub const OP_RowSetAdd: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
    
    
    pub const OP_RowSetAdd_1: ::core::ffi::c_int = 157;
    
    
    pub const OP_Param: ::core::ffi::c_int = 158 as ::core::ffi::c_int;
    
    
    pub const OP_Param_1: ::core::ffi::c_int = 158;
    
    
    pub const OP_FkCounter: ::core::ffi::c_int = 159 as ::core::ffi::c_int;
    
    
    pub const OP_FkCounter_1: ::core::ffi::c_int = 159;
    
    
    pub const OP_MemMax: ::core::ffi::c_int = 160 as ::core::ffi::c_int;
    
    
    pub const OP_MemMax_1: ::core::ffi::c_int = 160;
    
    
    pub const OP_OffsetLimit: ::core::ffi::c_int = 161 as ::core::ffi::c_int;
    
    
    pub const OP_OffsetLimit_1: ::core::ffi::c_int = 161;
    
    
    pub const OP_AggInverse: ::core::ffi::c_int = 162;
    
    
    pub const OP_AggInverse_1: ::core::ffi::c_int = 162 as ::core::ffi::c_int;
    
    
    pub const OP_AggStep: ::core::ffi::c_int = 163 as ::core::ffi::c_int;
    
    
    pub const OP_AggStep_1: ::core::ffi::c_int = 163;
    
    
    pub const OP_AggStep1: ::core::ffi::c_int = 164;
    
    
    pub const OP_AggValue: ::core::ffi::c_int = 165;
    
    
    pub const OP_AggValue_1: ::core::ffi::c_int = 165 as ::core::ffi::c_int;
    
    
    pub const OP_AggFinal: ::core::ffi::c_int = 166 as ::core::ffi::c_int;
    
    
    pub const OP_AggFinal_1: ::core::ffi::c_int = 166;
    
    
    pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
    
    
    pub const OP_Expire_1: ::core::ffi::c_int = 167;
    
    
    pub const OP_CursorLock: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
    
    
    pub const OP_CursorLock_1: ::core::ffi::c_int = 168;
    
    
    pub const OP_CursorUnlock: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
    
    
    pub const OP_CursorUnlock_1: ::core::ffi::c_int = 169;
    
    
    pub const OP_TableLock: ::core::ffi::c_int = 170 as ::core::ffi::c_int;
    
    
    pub const OP_TableLock_1: ::core::ffi::c_int = 170;
    
    
    pub const OP_VBegin: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
    
    
    pub const OP_VBegin_1: ::core::ffi::c_int = 171;
    
    
    pub const OP_VCreate: ::core::ffi::c_int = 172;
    
    
    pub const OP_VCreate_1: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
    
    
    pub const OP_VDestroy: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
    
    
    pub const OP_VDestroy_1: ::core::ffi::c_int = 173;
    
    
    pub const OP_VOpen: ::core::ffi::c_int = 174 as ::core::ffi::c_int;
    
    
    pub const OP_VOpen_1: ::core::ffi::c_int = 174;
    
    
    pub const OP_VCheck: ::core::ffi::c_int = 175 as ::core::ffi::c_int;
    
    
    pub const OP_VCheck_1: ::core::ffi::c_int = 175;
    
    
    pub const OP_VInitIn: ::core::ffi::c_int = 176;
    
    
    pub const OP_VInitIn_1: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
    
    
    pub const OP_VColumn: ::core::ffi::c_int = 177 as ::core::ffi::c_int;
    
    
    pub const OP_VColumn_1: ::core::ffi::c_int = 177;
    
    
    pub const OP_VRename: ::core::ffi::c_int = 178 as ::core::ffi::c_int;
    
    
    pub const OP_VRename_1: ::core::ffi::c_int = 178;
    
    
    pub const OP_Pagecount: ::core::ffi::c_int = 179 as ::core::ffi::c_int;
    
    
    pub const OP_Pagecount_1: ::core::ffi::c_int = 179;
    
    
    pub const OP_MaxPgcnt: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
    
    
    pub const OP_MaxPgcnt_1: ::core::ffi::c_int = 180;
    
    
    pub const OP_ClrSubtype: ::core::ffi::c_int = 181 as ::core::ffi::c_int;
    
    
    pub const OP_ClrSubtype_1: ::core::ffi::c_int = 181;
    
    
    pub const OP_GetSubtype: ::core::ffi::c_int = 182 as ::core::ffi::c_int;
    
    
    pub const OP_GetSubtype_1: ::core::ffi::c_int = 182;
    
    
    pub const OP_SetSubtype: ::core::ffi::c_int = 183 as ::core::ffi::c_int;
    
    
    pub const OP_SetSubtype_1: ::core::ffi::c_int = 183;
    
    
    pub const OP_FilterAdd: ::core::ffi::c_int = 184 as ::core::ffi::c_int;
    
    
    pub const OP_FilterAdd_1: ::core::ffi::c_int = 184;
    
    
    pub const OP_Trace: ::core::ffi::c_int = 185 as ::core::ffi::c_int;
    
    
    pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
    
    
    pub const OP_Explain: ::core::ffi::c_int = 189 as ::core::ffi::c_int;
    
    
    pub const OPFLG_JUMP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MX_JUMP_OPCODE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
}pub mod sqliteInt_h {
    extern "C" {
        pub type CCurHint;
    }
    pub use crate::src::src::alter::RenameToken;
    
    pub use crate::src::src::build::TableLock;
    
    pub use crate::src::src::vtab::VtabCtx;
    pub use crate::src::src::alter::RenameCtx;
    pub use crate::src::src::select::CheckOnCtx;
    
    pub use crate::src::src::r#where::CoveringIndexCheck;
    pub use crate::src::src::select::WhereConst;
    
    pub use crate::src::src::window::WindowRewrite;
    pub use crate::src::src::expr::IdxCover;
    pub use crate::src::src::expr::RefSrcList;
    
    
    pub const SQLITE_MUTEX_STATIC_TEMPDIR: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_MUTEX_STATIC_VFS1;
    
    
    pub const SQLITE_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_MEMSTATUS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MALLOC_SOFT_LIMIT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DIGIT_SEPARATOR: ::core::ffi::c_int = '_' as i32;
    
    
    pub const SQLITE_BIG_DBL: ::core::ffi::c_double = 1e99f64;
    
    
    pub const OMIT_TEMPDB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_FILE_FORMAT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TEMP_STORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_WORKER_THREADS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_WORKER_THREADS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_PCACHE_INITSZ: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_SORTERREF_SIZE: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
    
    
    pub type i8_0 = crate::stdlib::int8_t;
    
    
    pub type bft = ::core::ffi::c_uint;
    
    
    pub const SQLITE_MAX_U32: crate::src::ext::rtree::rtree::u64_0 =
        ((1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int).wrapping_sub(1 as crate::src::ext::rtree::rtree::u64_0);
    
    
    pub type tRowcnt = crate::src::ext::rtree::rtree::u64_0;
    
    
    pub type LogEst = crate::stdlib::int16_t;
    
    
    pub const LOGEST_MIN: ::core::ffi::c_int = -(32768 as ::core::ffi::c_int);
    
    
    pub const SQLITE_PTRSIZE: ::core::ffi::c_int = crate::internal::__SIZEOF_POINTER__;
    
    
    pub type uptr = crate::stdlib::uintptr_t;
    
    
    pub const SQLITE_BYTEORDER: ::core::ffi::c_int = 1234 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BIGENDIAN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LITTLEENDIAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_UTF16LE;
    
    
    pub const LARGEST_UINT64: crate::src::ext::rtree::rtree::u64_0 = 0xffffffff as crate::src::ext::rtree::rtree::u64_0
        | (0xffffffff as ::core::ffi::c_uint as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_MMAP_SIZE: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_MMAP_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct BusyHandler {
        pub xBusyHandler: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub pBusyArg: *mut ::core::ffi::c_void,
        pub nBusy: ::core::ffi::c_int,
    }
    
    
    pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0")
    };
    
    
    pub const LEGACY_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
        ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"sqlite_temp_master\0")
    };
    
    
    pub const PREFERRED_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_schema\0")
    };
    
    
    pub const PREFERRED_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
        ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"sqlite_temp_schema\0")
    };
    
    
    pub const SCHEMA_ROOT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub type StrAccum = crate::sqliteInt_h::sqlite3_str;
    
    
    pub type Bitmask = crate::src::ext::rtree::rtree::u64_0;
    
    
    pub const BMS: ::core::ffi::c_int =
        (::core::mem::size_of::<crate::sqliteInt_h::Bitmask>() as usize).wrapping_mul(8 as usize) as ::core::ffi::c_int;
    
    
    pub const ALLBITS: crate::sqliteInt_h::Bitmask = -(1 as ::core::ffi::c_int) as crate::sqliteInt_h::Bitmask;
    
    
    pub const TOPBIT: crate::sqliteInt_h::Bitmask =
        (1 as ::core::ffi::c_int as crate::sqliteInt_h::Bitmask) << crate::sqliteInt_h::BMS - 1 as ::core::ffi::c_int;
    
    
    pub type VList = ::core::ffi::c_int;
    
    
    pub const SQLITE_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Db {
        pub zDbSName: *mut ::core::ffi::c_char,
        pub pBt: *mut crate::btreeInt_h::Btree,
        pub safety_level: crate::src::ext::rtree::rtree::u8_0,
        pub bSyncSet: crate::src::ext::rtree::rtree::u8_0,
        pub pSchema: *mut crate::sqliteInt_h::Schema,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Schema {
        pub schema_cookie: ::core::ffi::c_int,
        pub iGeneration: ::core::ffi::c_int,
        pub tblHash: crate::src::src::hash::Hash,
        pub idxHash: crate::src::src::hash::Hash,
        pub trigHash: crate::src::src::hash::Hash,
        pub fkeyHash: crate::src::src::hash::Hash,
        pub pSeqTab: *mut crate::sqliteInt_h::Table,
        pub file_format: crate::src::ext::rtree::rtree::u8_0,
        pub enc: crate::src::ext::rtree::rtree::u8_0,
        pub schemaFlags: crate::src::fts5::u16_0,
        pub cache_size: ::core::ffi::c_int,
    }
    
    
    pub const DB_SchemaLoaded: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const DB_UnresetViews: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const DB_ResetWanted: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_N_LIMIT: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_LIMIT_WORKER_THREADS + 1 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Lookaside {
        pub bDisable: crate::src::ext::rtree::rtree::u32_0,
        pub sz: crate::src::fts5::u16_0,
        pub szTrue: crate::src::fts5::u16_0,
        pub bMalloced: crate::src::ext::rtree::rtree::u8_0,
        pub nSlot: crate::src::ext::rtree::rtree::u32_0,
        pub anStat: [crate::src::ext::rtree::rtree::u32_0; 3],
        pub pInit: *mut crate::sqliteInt_h::LookasideSlot,
        pub pFree: *mut crate::sqliteInt_h::LookasideSlot,
        pub pSmallInit: *mut crate::sqliteInt_h::LookasideSlot,
        pub pSmallFree: *mut crate::sqliteInt_h::LookasideSlot,
        pub pMiddle: *mut ::core::ffi::c_void,
        pub pStart: *mut ::core::ffi::c_void,
        pub pEnd: *mut ::core::ffi::c_void,
        pub pTrueEnd: *mut ::core::ffi::c_void,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct LookasideSlot {
        pub pNext: *mut crate::sqliteInt_h::LookasideSlot,
    }
    
    
    pub const LOOKASIDE_SMALL: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_HASH_SZ: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct FuncDefHash {
        pub a: [*mut crate::sqliteInt_h::FuncDef; 23],
    }
    
    
    pub type sqlite3_xauth = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >;
    
    
    pub const SQLITE_TRACE_LEGACY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRACE_XPROFILE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRACE_NONLEGACY_MASK: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
    
    
    pub const SQLITE_MAX_DB: ::core::ffi::c_int = crate::sqliteLimit_h::SQLITE_MAX_ATTACHED + 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3 {
        pub pVfs: *mut crate::sqlite3_h::sqlite3_vfs,
        pub pVdbe: *mut crate::vdbeInt_h::Vdbe,
        pub pDfltColl: *mut crate::sqliteInt_h::CollSeq,
        pub mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
        pub aDb: *mut crate::sqliteInt_h::Db,
        pub nDb: ::core::ffi::c_int,
        pub mDbFlags: crate::src::ext::rtree::rtree::u32_0,
        pub flags: crate::src::ext::rtree::rtree::u64_0,
        pub lastRowid: crate::src::ext::rtree::rtree::i64_0,
        pub szMmap: crate::src::ext::rtree::rtree::i64_0,
        pub nSchemaLock: crate::src::ext::rtree::rtree::u32_0,
        pub openFlags: ::core::ffi::c_uint,
        pub errCode: ::core::ffi::c_int,
        pub errByteOffset: ::core::ffi::c_int,
        pub errMask: ::core::ffi::c_int,
        pub iSysErrno: ::core::ffi::c_int,
        pub dbOptFlags: crate::src::ext::rtree::rtree::u32_0,
        pub enc: crate::src::ext::rtree::rtree::u8_0,
        pub autoCommit: crate::src::ext::rtree::rtree::u8_0,
        pub temp_store: crate::src::ext::rtree::rtree::u8_0,
        pub mallocFailed: crate::src::ext::rtree::rtree::u8_0,
        pub bBenignMalloc: crate::src::ext::rtree::rtree::u8_0,
        pub dfltLockMode: crate::src::ext::rtree::rtree::u8_0,
        pub nextAutovac: ::core::ffi::c_schar,
        pub suppressErr: crate::src::ext::rtree::rtree::u8_0,
        pub vtabOnConflict: crate::src::ext::rtree::rtree::u8_0,
        pub isTransactionSavepoint: crate::src::ext::rtree::rtree::u8_0,
        pub mTrace: crate::src::ext::rtree::rtree::u8_0,
        pub noSharedCache: crate::src::ext::rtree::rtree::u8_0,
        pub nSqlExec: crate::src::ext::rtree::rtree::u8_0,
        pub eOpenState: crate::src::ext::rtree::rtree::u8_0,
        pub nextPagesize: ::core::ffi::c_int,
        pub nChange: crate::src::ext::rtree::rtree::i64_0,
        pub nTotalChange: crate::src::ext::rtree::rtree::i64_0,
        pub aLimit: [::core::ffi::c_int; 12],
        pub nMaxSorterMmap: ::core::ffi::c_int,
        pub init: crate::sqliteInt_h::sqlite3InitInfo,
        pub nVdbeActive: ::core::ffi::c_int,
        pub nVdbeRead: ::core::ffi::c_int,
        pub nVdbeWrite: ::core::ffi::c_int,
        pub nVdbeExec: ::core::ffi::c_int,
        pub nVDestroy: ::core::ffi::c_int,
        pub nExtension: ::core::ffi::c_int,
        pub aExtension: *mut *mut ::core::ffi::c_void,
        pub trace: crate::sqliteInt_h::__anon_union_0,
        pub pTraceArg: *mut ::core::ffi::c_void,
        pub xProfile: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, crate::src::ext::rtree::rtree::u64_0) -> (),
        >,
        pub pProfileArg: *mut ::core::ffi::c_void,
        pub pCommitArg: *mut ::core::ffi::c_void,
        pub xCommitCallback:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub pRollbackArg: *mut ::core::ffi::c_void,
        pub xRollbackCallback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub pUpdateArg: *mut ::core::ffi::c_void,
        pub xUpdateCallback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite_int64,
            ) -> (),
        >,
        pub pAutovacPagesArg: *mut ::core::ffi::c_void,
        pub xAutovacDestr: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub xAutovacPages: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                crate::src::ext::rtree::rtree::u32_0,
                crate::src::ext::rtree::rtree::u32_0,
                crate::src::ext::rtree::rtree::u32_0,
            ) -> ::core::ffi::c_uint,
        >,
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub pPreUpdateArg: *mut ::core::ffi::c_void,
        pub xPreUpdateCallback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_int64,
                crate::sqlite3_h::sqlite3_int64,
            ) -> (),
        >,
        pub pPreUpdate: *mut crate::vdbeInt_h::PreUpdate,
        pub xWalCallback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut crate::sqliteInt_h::sqlite3,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub pWalArg: *mut ::core::ffi::c_void,
        pub xCollNeeded: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
        pub xCollNeeded16: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut crate::sqliteInt_h::sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
            ) -> (),
        >,
        pub pCollNeededArg: *mut ::core::ffi::c_void,
        pub pErr: *mut crate::vdbeInt_h::sqlite3_value,
        pub u1: crate::sqliteInt_h::__anon_union_1,
        pub lookaside: crate::sqliteInt_h::Lookaside,
        pub xAuth: crate::sqliteInt_h::sqlite3_xauth,
        pub pAuthArg: *mut ::core::ffi::c_void,
        pub xProgress: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub pProgressArg: *mut ::core::ffi::c_void,
        pub nProgressOps: ::core::ffi::c_uint,
        pub nVTrans: ::core::ffi::c_int,
        pub aModule: crate::src::src::hash::Hash,
        pub pVtabCtx: *mut crate::sqliteInt_h::VtabCtx,
        pub aVTrans: *mut *mut crate::sqliteInt_h::VTable,
        pub pDisconnect: *mut crate::sqliteInt_h::VTable,
        pub aFunc: crate::src::src::hash::Hash,
        pub aCollSeq: crate::src::src::hash::Hash,
        pub busyHandler: crate::sqliteInt_h::BusyHandler,
        pub aDbStatic: [crate::sqliteInt_h::Db; 2],
        pub pSavepoint: *mut crate::sqliteInt_h::Savepoint,
        pub nAnalysisLimit: ::core::ffi::c_int,
        pub busyTimeout: ::core::ffi::c_int,
        pub nSavepoint: ::core::ffi::c_int,
        pub nStatement: ::core::ffi::c_int,
        pub nDeferredCons: crate::src::ext::rtree::rtree::i64_0,
        pub nDeferredImmCons: crate::src::ext::rtree::rtree::i64_0,
        pub pnBytesFreed: *mut ::core::ffi::c_int,
        pub pDbData: *mut crate::sqliteInt_h::DbClientData,
        pub nSpill: crate::src::ext::rtree::rtree::u64_0,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct sqlite3InitInfo {
        pub newTnum: crate::src::src::pager::Pgno,
        pub iDb: crate::src::ext::rtree::rtree::u8_0,
        pub busy: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "orphanTrigger", ty = "::core::ffi::c_uint", bits = "0..=0")]
        #[bitfield(name = "imposterTable", ty = "::core::ffi::c_uint", bits = "1..=2")]
        #[bitfield(name = "reopenMemdb", ty = "::core::ffi::c_uint", bits = "3..=3")]
        pub orphanTrigger_imposterTable_reopenMemdb: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 1],
        pub azInit: *mut *const ::core::ffi::c_char,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_0 {
        pub xLegacy: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> (),
        >,
        pub xV2: Option<
            unsafe extern "C" fn(
                crate::src::ext::rtree::rtree::u32_0,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_1 {
        pub isInterrupted: ::core::ffi::c_int,
        pub notUsed1: ::core::ffi::c_double,
    }
    
    
    pub const SQLITE_WriteSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LegacyFileFmt: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FullColNames: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FullFSync: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CkptFullFSync: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CacheSpill: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ShortColNames: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NullCallback: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IgnoreChecks: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const SQLITE_StmtScanStatus: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NoCkptOnClose: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ReverseOrder: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_RecTriggers: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AutoIndex: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LoadExtension: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LoadExtFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_EnableTrigger: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_QueryOnly: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CellSizeCk: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_Fts3Tokenizer: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_EnableQPSG: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TriggerEQP: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ResetDatabase: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LegacyAlter: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NoSchemaError: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DqsDDL: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DqsDML: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_EnableView: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
    
    
    pub const SQLITE_CountRows: crate::src::ext::rtree::rtree::u64_0 =
        (0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CorruptRdOnly: crate::src::ext::rtree::rtree::u64_0 =
        (0x2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ReadUncommit: crate::src::ext::rtree::rtree::u64_0 =
        (0x4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FkNoAction: crate::src::ext::rtree::rtree::u64_0 =
        (0x8 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AttachCreate: crate::src::ext::rtree::rtree::u64_0 =
        (0x10 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AttachWrite: crate::src::ext::rtree::rtree::u64_0 =
        (0x20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_Comments: crate::src::ext::rtree::rtree::u64_0 =
        (0x40 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_PreferBuiltin: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_Vacuum: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_VacuumInto: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_InternalFunc: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const DBFLAG_EncodingFixed: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATE_OPEN: ::core::ffi::c_int = 0x76 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATE_CLOSED: ::core::ffi::c_int = 0xce as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATE_SICK: ::core::ffi::c_int = 0xba as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATE_BUSY: ::core::ffi::c_int = 0x6d as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATE_ERROR: ::core::ffi::c_int = 0xd5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATE_ZOMBIE: ::core::ffi::c_int = 0xa7 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct FuncDef {
        pub nArg: crate::src::fts5::i16_0,
        pub funcFlags: crate::src::ext::rtree::rtree::u32_0,
        pub pUserData: *mut ::core::ffi::c_void,
        pub pNext: *mut crate::sqliteInt_h::FuncDef,
        pub xSFunc: Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        pub xFinalize: Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
        pub xValue: Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
        pub xInverse: Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        pub zName: *const ::core::ffi::c_char,
        pub u: crate::sqliteInt_h::__anon_union_2,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_2 {
        pub pHash: *mut crate::sqliteInt_h::FuncDef,
        pub pDestructor: *mut crate::sqliteInt_h::FuncDestructor,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct FuncDestructor {
        pub nRef: ::core::ffi::c_int,
        pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub pUserData: *mut ::core::ffi::c_void,
    }
    
    
    pub const SQLITE_FUNC_ENCMASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_LIKE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_CASE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_EPHEM: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_LENGTH: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_TYPEOF: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_COUNT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_UNLIKELY: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_MINMAX: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_SLOCHNG: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_TEST: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_RUNONLY: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_WINDOW: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_INTERNAL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_DIRECT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_UNSAFE: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_INLINE: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNC_ANYORDER: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
    
    
    pub const INLINEFUNC_coalesce: ::core::ffi::c_int = 0;
    
    
    pub const INLINEFUNC_implies_nonnull_row: ::core::ffi::c_int = 1;
    
    
    pub const INLINEFUNC_expr_implies_expr: ::core::ffi::c_int = 2;
    
    
    pub const INLINEFUNC_expr_compare: ::core::ffi::c_int = 3;
    
    
    pub const INLINEFUNC_affinity: ::core::ffi::c_int = 4;
    
    
    pub const INLINEFUNC_iif: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Savepoint {
        pub zName: *mut ::core::ffi::c_char,
        pub nDeferredCons: crate::src::ext::rtree::rtree::i64_0,
        pub nDeferredImmCons: crate::src::ext::rtree::rtree::i64_0,
        pub pNext: *mut crate::sqliteInt_h::Savepoint,
    }
    
    
    pub const SAVEPOINT_BEGIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SAVEPOINT_BEGIN_1: ::core::ffi::c_int = 0;
    
    
    pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SAVEPOINT_ROLLBACK_1: ::core::ffi::c_int = 2;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Module {
        pub pModule: *const crate::sqlite3_h::sqlite3_module,
        pub zName: *const ::core::ffi::c_char,
        pub nRefModule: ::core::ffi::c_int,
        pub pAux: *mut ::core::ffi::c_void,
        pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub pEpoTab: *mut crate::sqliteInt_h::Table,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct Column {
        pub zCnName: *mut ::core::ffi::c_char,
        #[bitfield(name = "notNull", ty = "::core::ffi::c_uint", bits = "0..=3")]
        #[bitfield(name = "eCType", ty = "::core::ffi::c_uint", bits = "4..=7")]
        pub notNull_eCType: [u8; 1],
        pub affinity: ::core::ffi::c_char,
        pub szEst: crate::src::ext::rtree::rtree::u8_0,
        pub hName: crate::src::ext::rtree::rtree::u8_0,
        pub iDflt: crate::src::fts5::u16_0,
        pub colFlags: crate::src::fts5::u16_0,
    }
    
    
    pub const COLTYPE_CUSTOM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const COLTYPE_ANY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const COLTYPE_BLOB: ::core::ffi::c_int = 2;
    
    
    pub const COLTYPE_INT: ::core::ffi::c_int = 3;
    
    
    pub const COLTYPE_INTEGER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const COLTYPE_INTEGER_1: ::core::ffi::c_int = 4;
    
    
    pub const COLTYPE_REAL: ::core::ffi::c_int = 5;
    
    
    pub const COLTYPE_TEXT: ::core::ffi::c_int = 6;
    
    
    pub const SQLITE_N_STDTYPE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_HASTYPE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_UNIQUE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_STORED: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_NOTAVAIL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_BUSY: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_HASCOLL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_NOEXPAND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
    
    
    pub const COLFLAG_NOINSERT: ::core::ffi::c_int = 0x62 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct CollSeq {
        pub zName: *mut ::core::ffi::c_char,
        pub enc: crate::src::ext::rtree::rtree::u8_0,
        pub pUser: *mut ::core::ffi::c_void,
        pub xCmp: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    
    
    pub const SQLITE_SO_ASC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SO_DESC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SO_UNDEFINED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    
    pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_INTEGER_1: ::core::ffi::c_int = 68;
    
    
    pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_REAL_1: ::core::ffi::c_int = 69;
    
    
    pub const SQLITE_AFF_FLEXNUM: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_DEFER: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AFF_MASK: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;
    
    
    pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTNULL: ::core::ffi::c_int = 0x90 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VTable {
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub pMod: *mut crate::sqliteInt_h::Module,
        pub pVtab: *mut crate::sqlite3_h::sqlite3_vtab,
        pub nRef: ::core::ffi::c_int,
        pub bConstraint: crate::src::ext::rtree::rtree::u8_0,
        pub bAllSchemas: crate::src::ext::rtree::rtree::u8_0,
        pub eVtabRisk: crate::src::ext::rtree::rtree::u8_0,
        pub iSavepoint: ::core::ffi::c_int,
        pub pNext: *mut crate::sqliteInt_h::VTable,
    }
    
    
    pub const SQLITE_VTABRISK_Low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTABRISK_Normal: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTABRISK_High: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Table {
        pub zName: *mut ::core::ffi::c_char,
        pub aCol: *mut crate::sqliteInt_h::Column,
        pub pIndex: *mut crate::sqliteInt_h::Index,
        pub zColAff: *mut ::core::ffi::c_char,
        pub pCheck: *mut crate::sqliteInt_h::ExprList,
        pub tnum: crate::src::src::pager::Pgno,
        pub nTabRef: crate::src::ext::rtree::rtree::u32_0,
        pub tabFlags: crate::src::ext::rtree::rtree::u32_0,
        pub iPKey: crate::src::fts5::i16_0,
        pub nCol: crate::src::fts5::i16_0,
        pub nNVCol: crate::src::fts5::i16_0,
        pub nRowLogEst: crate::sqliteInt_h::LogEst,
        pub szTabRow: crate::sqliteInt_h::LogEst,
        pub keyConf: crate::src::ext::rtree::rtree::u8_0,
        pub eTabType: crate::src::ext::rtree::rtree::u8_0,
        pub u: crate::sqliteInt_h::__anon_union_3,
        pub pTrigger: *mut crate::sqliteInt_h::Trigger,
        pub pSchema: *mut crate::sqliteInt_h::Schema,
        pub aHx: [crate::src::ext::rtree::rtree::u8_0; 16],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_3 {
        pub tab: crate::sqliteInt_h::__anon_struct_0,
        pub view: crate::sqliteInt_h::__anon_struct_1,
        pub vtab: crate::sqliteInt_h::__anon_struct_2,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_0 {
        pub addColOffset: ::core::ffi::c_int,
        pub pFKey: *mut crate::sqliteInt_h::FKey,
        pub pDfltList: *mut crate::sqliteInt_h::ExprList,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_1 {
        pub pSelect: *mut crate::sqliteInt_h::Select,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_2 {
        pub nArg: ::core::ffi::c_int,
        pub azArg: *mut *mut ::core::ffi::c_char,
        pub p: *mut crate::sqliteInt_h::VTable,
    }
    
    
    pub const TF_Readonly: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const TF_HasHidden: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const TF_HasPrimaryKey: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const TF_Autoincrement: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const TF_HasStat1: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const TF_HasVirtual: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const TF_HasStored: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
    
    
    pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const TF_MaybeReanalyze: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const TF_OOOHidden: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const TF_HasNotNull: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const TF_Ephemeral: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const TF_Eponymous: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const TF_Imposter: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const ViewCanHaveRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct FKey {
        pub pFrom: *mut crate::sqliteInt_h::Table,
        pub pNextFrom: *mut crate::sqliteInt_h::FKey,
        pub zTo: *mut ::core::ffi::c_char,
        pub pNextTo: *mut crate::sqliteInt_h::FKey,
        pub pPrevTo: *mut crate::sqliteInt_h::FKey,
        pub nCol: ::core::ffi::c_int,
        pub isDeferred: crate::src::ext::rtree::rtree::u8_0,
        pub aAction: [crate::src::ext::rtree::rtree::u8_0; 2],
        pub apTrigger: [*mut crate::sqliteInt_h::Trigger; 2],
        pub aCol: [crate::sqliteInt_h::sColMap; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sColMap {
        pub iFrom: ::core::ffi::c_int,
        pub zCol: *mut ::core::ffi::c_char,
    }
    
    
    pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const OE_Rollback: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const OE_Rollback_1: ::core::ffi::c_int = 1;
    
    
    pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const OE_Fail: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const OE_Fail_1: ::core::ffi::c_int = 3;
    
    
    pub const OE_Ignore: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const OE_Ignore_1: ::core::ffi::c_int = 4;
    
    
    pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const OE_Update: ::core::ffi::c_int = 6;
    
    
    pub const OE_Restrict: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const OE_Restrict_1: ::core::ffi::c_int = 7;
    
    
    pub const OE_SetNull: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const OE_SetNull_1: ::core::ffi::c_int = 8;
    
    
    pub const OE_SetDflt: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const OE_SetDflt_1: ::core::ffi::c_int = 9;
    
    
    pub const OE_Cascade: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const OE_Cascade_1: ::core::ffi::c_int = 10;
    
    
    pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct KeyInfo {
        pub nRef: crate::src::ext::rtree::rtree::u32_0,
        pub enc: crate::src::ext::rtree::rtree::u8_0,
        pub nKeyField: crate::src::fts5::u16_0,
        pub nAllField: crate::src::fts5::u16_0,
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub aSortFlags: *mut crate::src::ext::rtree::rtree::u8_0,
        pub aColl: [*mut crate::sqliteInt_h::CollSeq; 0],
    }
    
    
    pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct UnpackedRecord {
        pub pKeyInfo: *mut crate::sqliteInt_h::KeyInfo,
        pub aMem: *mut crate::src::src::vdbe::Mem,
        pub u: crate::sqliteInt_h::__anon_union_4,
        pub n: ::core::ffi::c_int,
        pub nField: crate::src::fts5::u16_0,
        pub default_rc: crate::sqliteInt_h::i8_0,
        pub errCode: crate::src::ext::rtree::rtree::u8_0,
        pub r1: crate::sqliteInt_h::i8_0,
        pub r2: crate::sqliteInt_h::i8_0,
        pub eqSeen: crate::src::ext::rtree::rtree::u8_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_4 {
        pub z: *mut ::core::ffi::c_char,
        pub i: crate::src::ext::rtree::rtree::i64_0,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct Index {
        pub zName: *mut ::core::ffi::c_char,
        pub aiColumn: *mut crate::src::fts5::i16_0,
        pub aiRowLogEst: *mut crate::sqliteInt_h::LogEst,
        pub pTable: *mut crate::sqliteInt_h::Table,
        pub zColAff: *mut ::core::ffi::c_char,
        pub pNext: *mut crate::sqliteInt_h::Index,
        pub pSchema: *mut crate::sqliteInt_h::Schema,
        pub aSortOrder: *mut crate::src::ext::rtree::rtree::u8_0,
        pub azColl: *mut *const ::core::ffi::c_char,
        pub pPartIdxWhere: *mut crate::sqliteInt_h::Expr,
        pub aColExpr: *mut crate::sqliteInt_h::ExprList,
        pub tnum: crate::src::src::pager::Pgno,
        pub szIdxRow: crate::sqliteInt_h::LogEst,
        pub nKeyCol: crate::src::fts5::u16_0,
        pub nColumn: crate::src::fts5::u16_0,
        pub onError: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "idxType", ty = "::core::ffi::c_uint", bits = "0..=1")]
        #[bitfield(name = "bUnordered", ty = "::core::ffi::c_uint", bits = "2..=2")]
        #[bitfield(name = "uniqNotNull", ty = "::core::ffi::c_uint", bits = "3..=3")]
        #[bitfield(name = "isResized", ty = "::core::ffi::c_uint", bits = "4..=4")]
        #[bitfield(name = "isCovering", ty = "::core::ffi::c_uint", bits = "5..=5")]
        #[bitfield(name = "noSkipScan", ty = "::core::ffi::c_uint", bits = "6..=6")]
        #[bitfield(name = "hasStat1", ty = "::core::ffi::c_uint", bits = "7..=7")]
        #[bitfield(name = "bNoQuery", ty = "::core::ffi::c_uint", bits = "8..=8")]
        #[bitfield(name = "bAscKeyBug", ty = "::core::ffi::c_uint", bits = "9..=9")]
        #[bitfield(name = "bHasVCol", ty = "::core::ffi::c_uint", bits = "10..=10")]
        #[bitfield(name = "bHasExpr", ty = "::core::ffi::c_uint", bits = "11..=11")]
        pub idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr:
            [u8; 2],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
        pub colNotIdxed: crate::sqliteInt_h::Bitmask,
    }
    
    
    pub const SQLITE_IDXTYPE_APPDEF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IDXTYPE_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IDXTYPE_IPK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const XN_ROWID: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    
    pub const XN_EXPR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Token {
        pub z: *const ::core::ffi::c_char,
        pub n: ::core::ffi::c_uint,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AggInfo {
        pub directMode: crate::src::ext::rtree::rtree::u8_0,
        pub useSortingIdx: crate::src::ext::rtree::rtree::u8_0,
        pub nSortingColumn: crate::src::ext::rtree::rtree::u32_0,
        pub sortingIdx: ::core::ffi::c_int,
        pub sortingIdxPTab: ::core::ffi::c_int,
        pub iFirstReg: ::core::ffi::c_int,
        pub pGroupBy: *mut crate::sqliteInt_h::ExprList,
        pub aCol: *mut crate::sqliteInt_h::AggInfo_col,
        pub nColumn: ::core::ffi::c_int,
        pub nAccumulator: ::core::ffi::c_int,
        pub aFunc: *mut crate::sqliteInt_h::AggInfo_func,
        pub nFunc: ::core::ffi::c_int,
        pub selId: crate::src::ext::rtree::rtree::u32_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AggInfo_col {
        pub pTab: *mut crate::sqliteInt_h::Table,
        pub pCExpr: *mut crate::sqliteInt_h::Expr,
        pub iTable: ::core::ffi::c_int,
        pub iColumn: ::core::ffi::c_int,
        pub iSorterColumn: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AggInfo_func {
        pub pFExpr: *mut crate::sqliteInt_h::Expr,
        pub pFunc: *mut crate::sqliteInt_h::FuncDef,
        pub iDistinct: ::core::ffi::c_int,
        pub iDistAddr: ::core::ffi::c_int,
        pub iOBTab: ::core::ffi::c_int,
        pub bOBPayload: crate::src::ext::rtree::rtree::u8_0,
        pub bOBUnique: crate::src::ext::rtree::rtree::u8_0,
        pub bUseSubtype: crate::src::ext::rtree::rtree::u8_0,
    }
    
    
    pub type ynVar = crate::src::fts5::i16_0;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Expr {
        pub op: crate::src::ext::rtree::rtree::u8_0,
        pub affExpr: ::core::ffi::c_char,
        pub op2: crate::src::ext::rtree::rtree::u8_0,
        pub flags: crate::src::ext::rtree::rtree::u32_0,
        pub u: crate::sqliteInt_h::__anon_union_5,
        pub pLeft: *mut crate::sqliteInt_h::Expr,
        pub pRight: *mut crate::sqliteInt_h::Expr,
        pub x: crate::sqliteInt_h::__anon_union_6,
        pub nHeight: ::core::ffi::c_int,
        pub iTable: ::core::ffi::c_int,
        pub iColumn: crate::sqliteInt_h::ynVar,
        pub iAgg: crate::src::fts5::i16_0,
        pub w: crate::sqliteInt_h::__anon_union_7,
        pub pAggInfo: *mut crate::sqliteInt_h::AggInfo,
        pub y: crate::sqliteInt_h::__anon_union_8,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_5 {
        pub zToken: *mut ::core::ffi::c_char,
        pub iValue: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_6 {
        pub pList: *mut crate::sqliteInt_h::ExprList,
        pub pSelect: *mut crate::sqliteInt_h::Select,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_7 {
        pub iJoin: ::core::ffi::c_int,
        pub iOfst: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_8 {
        pub pTab: *mut crate::sqliteInt_h::Table,
        pub pWin: *mut crate::sqliteInt_h::Window,
        pub nReg: ::core::ffi::c_int,
        pub sub: crate::sqliteInt_h::__anon_struct_3,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_3 {
        pub iAddr: ::core::ffi::c_int,
        pub regReturn: ::core::ffi::c_int,
    }
    
    
    pub const EP_OuterON: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const EP_InnerON: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const EP_Distinct: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const EP_HasFunc: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const EP_FixedCol: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const EP_DblQuoted: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const EP_InfixFunc: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const EP_Collate: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const EP_Commuted: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const EP_Skip: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const EP_Reduced: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const EP_TokenOnly: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const EP_IfNullRow: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const EP_Subquery: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
    
    
    pub const EP_Leaf: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
    
    
    pub const EP_WinFunc: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    
    
    pub const EP_Subrtn: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
    
    
    pub const EP_Quoted: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
    
    
    pub const EP_Static: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
    
    
    pub const EP_IsTrue: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
    
    
    pub const EP_IsFalse: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
    
    
    pub const EP_Propagate: ::core::ffi::c_int = crate::sqliteInt_h::EP_Collate | crate::sqliteInt_h::EP_Subquery | crate::sqliteInt_h::EP_HasFunc;
    
    
    pub const EXPR_FULLSIZE: usize = ::core::mem::size_of::<crate::sqliteInt_h::Expr>();
    
    
    pub const EXPR_REDUCEDSIZE: ::core::ffi::c_ulong = 44 as ::core::ffi::c_ulong;
    
    
    pub const EXPR_TOKENONLYSIZE: ::core::ffi::c_ulong = 16 as ::core::ffi::c_ulong;
    
    
    pub const EXPRDUP_REDUCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct ExprList {
        pub nExpr: ::core::ffi::c_int,
        pub nAlloc: ::core::ffi::c_int,
        pub a: [crate::sqliteInt_h::ExprList_item; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct ExprList_item {
        pub pExpr: *mut crate::sqliteInt_h::Expr,
        pub zEName: *mut ::core::ffi::c_char,
        pub fg: crate::sqliteInt_h::__anon_struct_4,
        pub u: crate::sqliteInt_h::__anon_union_9,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct __anon_struct_4 {
        pub sortFlags: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "eEName", ty = "::core::ffi::c_uint", bits = "0..=1")]
        #[bitfield(name = "done", ty = "::core::ffi::c_uint", bits = "2..=2")]
        #[bitfield(name = "reusable", ty = "::core::ffi::c_uint", bits = "3..=3")]
        #[bitfield(name = "bSorterRef", ty = "::core::ffi::c_uint", bits = "4..=4")]
        #[bitfield(name = "bNulls", ty = "::core::ffi::c_uint", bits = "5..=5")]
        #[bitfield(name = "bUsed", ty = "::core::ffi::c_uint", bits = "6..=6")]
        #[bitfield(name = "bUsingTerm", ty = "::core::ffi::c_uint", bits = "7..=7")]
        #[bitfield(name = "bNoExpand", ty = "::core::ffi::c_uint", bits = "8..=8")]
        pub eEName_done_reusable_bSorterRef_bNulls_bUsed_bUsingTerm_bNoExpand: [u8; 2],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 1],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_9 {
        pub x: crate::sqliteInt_h::__anon_struct_5,
        pub iConstExprReg: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_5 {
        pub iOrderByCol: crate::src::fts5::u16_0,
        pub iAlias: crate::src::fts5::u16_0,
    }
    
    
    pub const ENAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const ENAME_SPAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const ENAME_TAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const ENAME_ROWID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct IdList {
        pub nId: ::core::ffi::c_int,
        pub a: [crate::sqliteInt_h::IdList_item; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct IdList_item {
        pub zName: *mut ::core::ffi::c_char,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Subquery {
        pub pSelect: *mut crate::sqliteInt_h::Select,
        pub addrFillSub: ::core::ffi::c_int,
        pub regReturn: ::core::ffi::c_int,
        pub regResult: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct SrcItem {
        pub zName: *mut ::core::ffi::c_char,
        pub zAlias: *mut ::core::ffi::c_char,
        pub pSTab: *mut crate::sqliteInt_h::Table,
        pub fg: crate::sqliteInt_h::__anon_struct_6,
        pub iCursor: ::core::ffi::c_int,
        pub colUsed: crate::sqliteInt_h::Bitmask,
        pub u1: crate::sqliteInt_h::__anon_union_10,
        pub u2: crate::sqliteInt_h::__anon_union_11,
        pub u3: crate::sqliteInt_h::__anon_union_12,
        pub u4: crate::sqliteInt_h::__anon_union_13,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct __anon_struct_6 {
        pub jointype: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "notIndexed", ty = "::core::ffi::c_uint", bits = "0..=0")]
        #[bitfield(name = "isIndexedBy", ty = "::core::ffi::c_uint", bits = "1..=1")]
        #[bitfield(name = "isSubquery", ty = "::core::ffi::c_uint", bits = "2..=2")]
        #[bitfield(name = "isTabFunc", ty = "::core::ffi::c_uint", bits = "3..=3")]
        #[bitfield(name = "isCorrelated", ty = "::core::ffi::c_uint", bits = "4..=4")]
        #[bitfield(name = "isMaterialized", ty = "::core::ffi::c_uint", bits = "5..=5")]
        #[bitfield(name = "viaCoroutine", ty = "::core::ffi::c_uint", bits = "6..=6")]
        #[bitfield(name = "isRecursive", ty = "::core::ffi::c_uint", bits = "7..=7")]
        #[bitfield(name = "fromDDL", ty = "::core::ffi::c_uint", bits = "8..=8")]
        #[bitfield(name = "isCte", ty = "::core::ffi::c_uint", bits = "9..=9")]
        #[bitfield(name = "notCte", ty = "::core::ffi::c_uint", bits = "10..=10")]
        #[bitfield(name = "isUsing", ty = "::core::ffi::c_uint", bits = "11..=11")]
        #[bitfield(name = "isOn", ty = "::core::ffi::c_uint", bits = "12..=12")]
        #[bitfield(name = "isSynthUsing", ty = "::core::ffi::c_uint", bits = "13..=13")]
        #[bitfield(name = "isNestedFrom", ty = "::core::ffi::c_uint", bits = "14..=14")]
        #[bitfield(name = "rowidUsed", ty = "::core::ffi::c_uint", bits = "15..=15")]
        #[bitfield(name = "fixedSchema", ty = "::core::ffi::c_uint", bits = "16..=16")]
        #[bitfield(name = "hadSchema", ty = "::core::ffi::c_uint", bits = "17..=17")]
        #[bitfield(name = "fromExists", ty = "::core::ffi::c_uint", bits = "18..=18")]
        pub notIndexed_isIndexedBy_isSubquery_isTabFunc_isCorrelated_isMaterialized_viaCoroutine_isRecursive_fromDDL_isCte_notCte_isUsing_isOn_isSynthUsing_isNestedFrom_rowidUsed_fixedSchema_hadSchema_fromExists:
            [u8; 3],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_10 {
        pub zIndexedBy: *mut ::core::ffi::c_char,
        pub pFuncArg: *mut crate::sqliteInt_h::ExprList,
        pub nRow: crate::src::ext::rtree::rtree::u32_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_11 {
        pub pIBIndex: *mut crate::sqliteInt_h::Index,
        pub pCteUse: *mut crate::sqliteInt_h::CteUse,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_12 {
        pub pOn: *mut crate::sqliteInt_h::Expr,
        pub pUsing: *mut crate::sqliteInt_h::IdList,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_13 {
        pub pSchema: *mut crate::sqliteInt_h::Schema,
        pub zDatabase: *mut ::core::ffi::c_char,
        pub pSubq: *mut crate::sqliteInt_h::Subquery,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct OnOrUsing {
        pub pOn: *mut crate::sqliteInt_h::Expr,
        pub pUsing: *mut crate::sqliteInt_h::IdList,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct SrcList {
        pub nSrc: ::core::ffi::c_int,
        pub nAlloc: crate::src::ext::rtree::rtree::u32_0,
        pub a: [crate::sqliteInt_h::SrcItem; 0],
    }
    
    
    pub const SZ_SRCLIST_1: usize =
        (8 as usize).wrapping_add(::core::mem::size_of::<crate::sqliteInt_h::SrcItem>() as usize);
    
    
    pub const JT_INNER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const JT_CROSS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const JT_NATURAL: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const JT_OUTER: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const JT_ERROR: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const WHERE_ORDERBY_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const WHERE_ORDERBY_MIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const WHERE_ORDERBY_MAX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const WHERE_ONEPASS_DESIRED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const WHERE_ONEPASS_MULTIROW: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const WHERE_DUPLICATES_OK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const WHERE_OR_SUBCLAUSE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const WHERE_GROUPBY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const WHERE_DISTINCTBY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const WHERE_WANT_DISTINCT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const WHERE_SORTBYGROUP: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const WHERE_AGG_DISTINCT: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const WHERE_ORDERBY_LIMIT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const WHERE_RIGHT_JOIN: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const WHERE_KEEP_ALL_JOINS: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const WHERE_USE_LIMIT: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const WHERE_DISTINCT_NOOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const WHERE_DISTINCT_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const WHERE_DISTINCT_ORDERED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const WHERE_DISTINCT_UNORDERED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct NameContext {
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub pSrcList: *mut crate::sqliteInt_h::SrcList,
        pub uNC: crate::sqliteInt_h::__anon_union_14,
        pub pNext: *mut crate::sqliteInt_h::NameContext,
        pub nRef: ::core::ffi::c_int,
        pub nNcErr: ::core::ffi::c_int,
        pub ncFlags: ::core::ffi::c_int,
        pub nNestedSelect: crate::src::ext::rtree::rtree::u32_0,
        pub pWinSelect: *mut crate::sqliteInt_h::Select,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_14 {
        pub pEList: *mut crate::sqliteInt_h::ExprList,
        pub pAggInfo: *mut crate::sqliteInt_h::AggInfo,
        pub pUpsert: *mut crate::sqliteInt_h::Upsert,
        pub iBaseReg: ::core::ffi::c_int,
    }
    
    
    pub const NC_AllowAgg: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const NC_PartIdx: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const NC_IsCheck: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const NC_GenCol: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const NC_HasAgg: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const NC_IdxExpr: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const NC_SelfRef: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;
    
    
    pub const NC_Subquery: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const NC_UEList: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const NC_UUpsert: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const NC_UBaseReg: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const NC_MinMaxAgg: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const NC_AllowWin: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const NC_HasWin: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const NC_IsDDL: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const NC_InAggFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const NC_FromDDL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const NC_NoSelect: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
    
    
    pub const NC_Where: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
    
    
    pub const NC_OrderAgg: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Upsert {
        pub pUpsertTarget: *mut crate::sqliteInt_h::ExprList,
        pub pUpsertTargetWhere: *mut crate::sqliteInt_h::Expr,
        pub pUpsertSet: *mut crate::sqliteInt_h::ExprList,
        pub pUpsertWhere: *mut crate::sqliteInt_h::Expr,
        pub pNextUpsert: *mut crate::sqliteInt_h::Upsert,
        pub isDoUpdate: crate::src::ext::rtree::rtree::u8_0,
        pub isDup: crate::src::ext::rtree::rtree::u8_0,
        pub pToFree: *mut ::core::ffi::c_void,
        pub pUpsertIdx: *mut crate::sqliteInt_h::Index,
        pub pUpsertSrc: *mut crate::sqliteInt_h::SrcList,
        pub regData: ::core::ffi::c_int,
        pub iDataCur: ::core::ffi::c_int,
        pub iIdxCur: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Select {
        pub op: crate::src::ext::rtree::rtree::u8_0,
        pub nSelectRow: crate::sqliteInt_h::LogEst,
        pub selFlags: crate::src::ext::rtree::rtree::u32_0,
        pub iLimit: ::core::ffi::c_int,
        pub iOffset: ::core::ffi::c_int,
        pub selId: crate::src::ext::rtree::rtree::u32_0,
        pub addrOpenEphm: [::core::ffi::c_int; 2],
        pub pEList: *mut crate::sqliteInt_h::ExprList,
        pub pSrc: *mut crate::sqliteInt_h::SrcList,
        pub pWhere: *mut crate::sqliteInt_h::Expr,
        pub pGroupBy: *mut crate::sqliteInt_h::ExprList,
        pub pHaving: *mut crate::sqliteInt_h::Expr,
        pub pOrderBy: *mut crate::sqliteInt_h::ExprList,
        pub pPrior: *mut crate::sqliteInt_h::Select,
        pub pNext: *mut crate::sqliteInt_h::Select,
        pub pLimit: *mut crate::sqliteInt_h::Expr,
        pub pWith: *mut crate::sqliteInt_h::With,
        pub pWin: *mut crate::sqliteInt_h::Window,
        pub pWinDefn: *mut crate::sqliteInt_h::Window,
    }
    
    
    pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SF_All: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SF_Resolved: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const SF_UsesEphemeral: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const SF_Expanded: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SF_HasTypeInfo: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const SF_Compound: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const SF_Recursive: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const SF_FixedLimit: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const SF_Converted: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const SF_IncludeHidden: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const SF_ComplexResult: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const SF_WinRewrite: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
    
    
    pub const SF_View: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
    
    
    pub const SF_NoopOrderBy: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
    
    
    pub const SF_UFSrcCheck: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
    
    
    pub const SF_PushDown: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    
    
    pub const SF_MultiPart: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
    
    
    pub const SF_CopyCte: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
    
    
    pub const SF_OrderByReqd: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
    
    
    pub const SF_UpdateFrom: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
    
    
    pub const SF_Correlated: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
    
    
    pub const SF_OnToWhere: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
    
    
    pub const SRT_Union: ::core::ffi::c_int = 1;
    
    
    pub const SRT_Except: ::core::ffi::c_int = 2;
    
    
    pub const SRT_Exists: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SRT_Exists_1: ::core::ffi::c_int = 3;
    
    
    pub const SRT_Discard: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SRT_DistFifo: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SRT_DistQueue: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SRT_Queue: ::core::ffi::c_int = 7;
    
    
    pub const SRT_Fifo: ::core::ffi::c_int = 8;
    
    
    pub const SRT_Output: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const SRT_Mem: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SRT_Mem_1: ::core::ffi::c_int = 10;
    
    
    pub const SRT_Set: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SRT_EphemTab: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    
    pub const SRT_EphemTab_1: ::core::ffi::c_int = 12;
    
    
    pub const SRT_Coroutine: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    
    
    pub const SRT_Table: ::core::ffi::c_int = 14;
    
    
    pub const SRT_Table_1: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    
    
    pub const SRT_Upfrom: ::core::ffi::c_int = 15;
    
    
    pub const SRT_Upfrom_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct SelectDest {
        pub eDest: crate::src::ext::rtree::rtree::u8_0,
        pub iSDParm: ::core::ffi::c_int,
        pub iSDParm2: ::core::ffi::c_int,
        pub iSdst: ::core::ffi::c_int,
        pub nSdst: ::core::ffi::c_int,
        pub zAffSdst: *mut ::core::ffi::c_char,
        pub pOrderBy: *mut crate::sqliteInt_h::ExprList,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AutoincInfo {
        pub pNext: *mut crate::sqliteInt_h::AutoincInfo,
        pub pTab: *mut crate::sqliteInt_h::Table,
        pub iDb: ::core::ffi::c_int,
        pub regCtr: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct TriggerPrg {
        pub pTrigger: *mut crate::sqliteInt_h::Trigger,
        pub pNext: *mut crate::sqliteInt_h::TriggerPrg,
        pub pProgram: *mut crate::src::src::vdbe::SubProgram,
        pub orconf: ::core::ffi::c_int,
        pub aColmask: [crate::src::ext::rtree::rtree::u32_0; 2],
    }
    
    
    pub type yDbMask = ::core::ffi::c_uint;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct IndexedExpr {
        pub pExpr: *mut crate::sqliteInt_h::Expr,
        pub iDataCur: ::core::ffi::c_int,
        pub iIdxCur: ::core::ffi::c_int,
        pub iIdxCol: ::core::ffi::c_int,
        pub bMaybeNullRow: crate::src::ext::rtree::rtree::u8_0,
        pub aff: crate::src::ext::rtree::rtree::u8_0,
        pub pIENext: *mut crate::sqliteInt_h::IndexedExpr,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct ParseCleanup {
        pub pNext: *mut crate::sqliteInt_h::ParseCleanup,
        pub pPtr: *mut ::core::ffi::c_void,
        pub xCleanup: Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::sqlite3, *mut ::core::ffi::c_void) -> ()>,
    }
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct Parse {
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub zErrMsg: *mut ::core::ffi::c_char,
        pub pVdbe: *mut crate::vdbeInt_h::Vdbe,
        pub rc: ::core::ffi::c_int,
        pub nQueryLoop: crate::sqliteInt_h::LogEst,
        pub nested: crate::src::ext::rtree::rtree::u8_0,
        pub nTempReg: crate::src::ext::rtree::rtree::u8_0,
        pub isMultiWrite: crate::src::ext::rtree::rtree::u8_0,
        pub mayAbort: crate::src::ext::rtree::rtree::u8_0,
        pub hasCompound: crate::src::ext::rtree::rtree::u8_0,
        pub disableLookaside: crate::src::ext::rtree::rtree::u8_0,
        pub prepFlags: crate::src::ext::rtree::rtree::u8_0,
        pub withinRJSubrtn: crate::src::ext::rtree::rtree::u8_0,
        pub bHasExists: crate::src::ext::rtree::rtree::u8_0,
        pub mSubrtnSig: crate::src::ext::rtree::rtree::u8_0,
        pub eTriggerOp: crate::src::ext::rtree::rtree::u8_0,
        pub bReturning: crate::src::ext::rtree::rtree::u8_0,
        pub eOrconf: crate::src::ext::rtree::rtree::u8_0,
        pub disableTriggers: crate::src::ext::rtree::rtree::u8_0,
        #[bitfield(name = "colNamesSet", ty = "bft", bits = "0..=0")]
        #[bitfield(name = "bHasWith", ty = "bft", bits = "1..=1")]
        #[bitfield(name = "okConstFactor", ty = "bft", bits = "2..=2")]
        #[bitfield(name = "checkSchema", ty = "bft", bits = "3..=3")]
        pub colNamesSet_bHasWith_okConstFactor_checkSchema: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
        pub nRangeReg: ::core::ffi::c_int,
        pub iRangeReg: ::core::ffi::c_int,
        pub nErr: ::core::ffi::c_int,
        pub nTab: ::core::ffi::c_int,
        pub nMem: ::core::ffi::c_int,
        pub szOpAlloc: ::core::ffi::c_int,
        pub iSelfTab: ::core::ffi::c_int,
        pub nLabel: ::core::ffi::c_int,
        pub nLabelAlloc: ::core::ffi::c_int,
        pub aLabel: *mut ::core::ffi::c_int,
        pub pConstExpr: *mut crate::sqliteInt_h::ExprList,
        pub pIdxEpr: *mut crate::sqliteInt_h::IndexedExpr,
        pub pIdxPartExpr: *mut crate::sqliteInt_h::IndexedExpr,
        pub writeMask: crate::sqliteInt_h::yDbMask,
        pub cookieMask: crate::sqliteInt_h::yDbMask,
        pub nMaxArg: ::core::ffi::c_int,
        pub nSelect: ::core::ffi::c_int,
        pub nProgressSteps: crate::src::ext::rtree::rtree::u32_0,
        pub nTableLock: ::core::ffi::c_int,
        pub aTableLock: *mut crate::sqliteInt_h::TableLock,
        pub pAinc: *mut crate::sqliteInt_h::AutoincInfo,
        pub pToplevel: *mut crate::sqliteInt_h::Parse,
        pub pTriggerTab: *mut crate::sqliteInt_h::Table,
        pub pTriggerPrg: *mut crate::sqliteInt_h::TriggerPrg,
        pub pCleanup: *mut crate::sqliteInt_h::ParseCleanup,
        pub aTempReg: [::core::ffi::c_int; 8],
        pub pOuterParse: *mut crate::sqliteInt_h::Parse,
        pub sNameToken: crate::sqliteInt_h::Token,
        pub oldmask: crate::src::ext::rtree::rtree::u32_0,
        pub newmask: crate::src::ext::rtree::rtree::u32_0,
        pub u1: crate::sqliteInt_h::__anon_union_15,
        pub sLastToken: crate::sqliteInt_h::Token,
        pub nVar: crate::sqliteInt_h::ynVar,
        pub iPkSortOrder: crate::src::ext::rtree::rtree::u8_0,
        pub explain: crate::src::ext::rtree::rtree::u8_0,
        pub eParseMode: crate::src::ext::rtree::rtree::u8_0,
        pub nVtabLock: ::core::ffi::c_int,
        pub nHeight: ::core::ffi::c_int,
        pub addrExplain: ::core::ffi::c_int,
        pub pVList: *mut crate::sqliteInt_h::VList,
        pub pReprepare: *mut crate::vdbeInt_h::Vdbe,
        pub zTail: *const ::core::ffi::c_char,
        pub pNewTable: *mut crate::sqliteInt_h::Table,
        pub pNewIndex: *mut crate::sqliteInt_h::Index,
        pub pNewTrigger: *mut crate::sqliteInt_h::Trigger,
        pub zAuthContext: *const ::core::ffi::c_char,
        pub sArg: crate::sqliteInt_h::Token,
        pub apVtabLock: *mut *mut crate::sqliteInt_h::Table,
        pub pWith: *mut crate::sqliteInt_h::With,
        pub pRename: *mut crate::sqliteInt_h::RenameToken,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_15 {
        pub cr: crate::sqliteInt_h::__anon_struct_7,
        pub d: crate::sqliteInt_h::__anon_struct_8,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_7 {
        pub addrCrTab: ::core::ffi::c_int,
        pub regRowid: ::core::ffi::c_int,
        pub regRoot: ::core::ffi::c_int,
        pub constraintName: crate::sqliteInt_h::Token,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_8 {
        pub pReturning: *mut crate::sqliteInt_h::Returning,
    }
    
    
    pub const PARSE_MODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const PARSE_MODE_DECLARE_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const PARSE_MODE_UNMAP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const PARSE_HDR_SZ: ::core::ffi::c_ulong =
        (192 as ::core::ffi::c_ulong).wrapping_sub(8 as ::core::ffi::c_ulong);
    
    
    pub const PARSE_RECURSE_SZ: ::core::ffi::c_ulong = 288 as ::core::ffi::c_ulong;
    
    
    pub const PARSE_TAIL_SZ: usize =
        (::core::mem::size_of::<crate::sqliteInt_h::Parse>() as usize).wrapping_sub(crate::sqliteInt_h::PARSE_RECURSE_SZ as usize);
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AuthContext {
        pub zAuthContext: *const ::core::ffi::c_char,
        pub pParse: *mut crate::sqliteInt_h::Parse,
    }
    
    
    pub const OPFLAG_NCHANGE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_NOCHNG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_LASTROWID: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_ISUPDATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_ISNOOP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_TYPEOFARG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_BYTELENARG: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_BULKCSR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_SEEKEQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_FORDELETE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_P2ISREG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_PERMUTE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_AUXDELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const OPFLAG_PREFORMAT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Trigger {
        pub zName: *mut ::core::ffi::c_char,
        pub table: *mut ::core::ffi::c_char,
        pub op: crate::src::ext::rtree::rtree::u8_0,
        pub tr_tm: crate::src::ext::rtree::rtree::u8_0,
        pub bReturning: crate::src::ext::rtree::rtree::u8_0,
        pub pWhen: *mut crate::sqliteInt_h::Expr,
        pub pColumns: *mut crate::sqliteInt_h::IdList,
        pub pSchema: *mut crate::sqliteInt_h::Schema,
        pub pTabSchema: *mut crate::sqliteInt_h::Schema,
        pub step_list: *mut crate::sqliteInt_h::TriggerStep,
        pub pNext: *mut crate::sqliteInt_h::Trigger,
    }
    
    
    pub const TRIGGER_BEFORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct TriggerStep {
        pub op: crate::src::ext::rtree::rtree::u8_0,
        pub orconf: crate::src::ext::rtree::rtree::u8_0,
        pub pTrig: *mut crate::sqliteInt_h::Trigger,
        pub pSelect: *mut crate::sqliteInt_h::Select,
        pub zTarget: *mut ::core::ffi::c_char,
        pub pFrom: *mut crate::sqliteInt_h::SrcList,
        pub pWhere: *mut crate::sqliteInt_h::Expr,
        pub pExprList: *mut crate::sqliteInt_h::ExprList,
        pub pIdList: *mut crate::sqliteInt_h::IdList,
        pub pUpsert: *mut crate::sqliteInt_h::Upsert,
        pub zSpan: *mut ::core::ffi::c_char,
        pub pNext: *mut crate::sqliteInt_h::TriggerStep,
        pub pLast: *mut crate::sqliteInt_h::TriggerStep,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Returning {
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub pReturnEL: *mut crate::sqliteInt_h::ExprList,
        pub retTrig: crate::sqliteInt_h::Trigger,
        pub retTStep: crate::sqliteInt_h::TriggerStep,
        pub iRetCur: ::core::ffi::c_int,
        pub nRetCol: ::core::ffi::c_int,
        pub iRetReg: ::core::ffi::c_int,
        pub zName: [::core::ffi::c_char; 40],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_str {
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub zText: *mut ::core::ffi::c_char,
        pub nAlloc: crate::src::ext::rtree::rtree::u32_0,
        pub mxAlloc: crate::src::ext::rtree::rtree::u32_0,
        pub nChar: crate::src::ext::rtree::rtree::u32_0,
        pub accError: crate::src::ext::rtree::rtree::u8_0,
        pub printfFlags: crate::src::ext::rtree::rtree::u8_0,
    }
    
    
    pub const SQLITE_PRINTF_INTERNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PRINTF_SQLFUNC: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PRINTF_MALLOCED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct RCStr {
        pub nRCRef: crate::src::ext::rtree::rtree::u64_0,
    }
    
    
    pub type InitData = crate::sqliteInt_h::__anon_struct_9;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __anon_struct_9 {
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub pzErrMsg: *mut *mut ::core::ffi::c_char,
        pub iDb: ::core::ffi::c_int,
        pub rc: ::core::ffi::c_int,
        pub mInitFlags: crate::src::ext::rtree::rtree::u32_0,
        pub nInitRow: crate::src::ext::rtree::rtree::u32_0,
        pub mxPage: crate::src::src::pager::Pgno,
    }
    
    
    pub const INITFLAG_AlterMask: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    
    
    pub const INITFLAG_AlterRename: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const INITFLAG_AlterDrop: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const INITFLAG_AlterAdd: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Sqlite3Config {
        pub bMemstat: ::core::ffi::c_int,
        pub bCoreMutex: crate::src::ext::rtree::rtree::u8_0,
        pub bFullMutex: crate::src::ext::rtree::rtree::u8_0,
        pub bOpenUri: crate::src::ext::rtree::rtree::u8_0,
        pub bUseCis: crate::src::ext::rtree::rtree::u8_0,
        pub bSmallMalloc: crate::src::ext::rtree::rtree::u8_0,
        pub bExtraSchemaChecks: crate::src::ext::rtree::rtree::u8_0,
        pub mxStrlen: ::core::ffi::c_int,
        pub neverCorrupt: ::core::ffi::c_int,
        pub szLookaside: ::core::ffi::c_int,
        pub nLookaside: ::core::ffi::c_int,
        pub nStmtSpill: ::core::ffi::c_int,
        pub m: crate::sqlite3_h::sqlite3_mem_methods,
        pub mutex: crate::sqlite3_h::sqlite3_mutex_methods,
        pub pcache2: crate::sqlite3_h::sqlite3_pcache_methods2,
        pub pHeap: *mut ::core::ffi::c_void,
        pub nHeap: ::core::ffi::c_int,
        pub mnReq: ::core::ffi::c_int,
        pub mxReq: ::core::ffi::c_int,
        pub szMmap: crate::sqlite3_h::sqlite3_int64,
        pub mxMmap: crate::sqlite3_h::sqlite3_int64,
        pub pPage: *mut ::core::ffi::c_void,
        pub szPage: ::core::ffi::c_int,
        pub nPage: ::core::ffi::c_int,
        pub mxParserStack: ::core::ffi::c_int,
        pub sharedCacheEnabled: ::core::ffi::c_int,
        pub szPma: crate::src::ext::rtree::rtree::u32_0,
        pub isInit: ::core::ffi::c_int,
        pub inProgress: ::core::ffi::c_int,
        pub isMutexInit: ::core::ffi::c_int,
        pub isMallocInit: ::core::ffi::c_int,
        pub isPCacheInit: ::core::ffi::c_int,
        pub nRefInitMutex: ::core::ffi::c_int,
        pub pInitMutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
        pub xLog: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
        pub pLogArg: *mut ::core::ffi::c_void,
        pub mxMemdbSize: crate::sqlite3_h::sqlite3_int64,
        pub xTestCallback: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub bLocaltimeFault: ::core::ffi::c_int,
        pub xAltLocaltime: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub iOnceResetThreshold: ::core::ffi::c_int,
        pub szSorterRef: crate::src::ext::rtree::rtree::u32_0,
        pub iPrngSeed: ::core::ffi::c_uint,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Walker {
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub xExprCallback:
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::Walker, *mut crate::sqliteInt_h::Expr) -> ::core::ffi::c_int>,
        pub xSelectCallback:
            Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::Walker, *mut crate::sqliteInt_h::Select) -> ::core::ffi::c_int>,
        pub xSelectCallback2: Option<unsafe extern "C" fn(*mut crate::sqliteInt_h::Walker, *mut crate::sqliteInt_h::Select) -> ()>,
        pub walkerDepth: ::core::ffi::c_int,
        pub eCode: crate::src::fts5::u16_0,
        pub mWFlags: crate::src::fts5::u16_0,
        pub u: crate::sqliteInt_h::__anon_union_16,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union __anon_union_16 {
        pub pNC: *mut crate::sqliteInt_h::NameContext,
        pub n: ::core::ffi::c_int,
        pub iCur: ::core::ffi::c_int,
        pub pSrcList: *mut crate::sqliteInt_h::SrcList,
        pub pCCurHint: *mut crate::sqliteInt_h::CCurHint,
        pub pRefSrcList: *mut crate::sqliteInt_h::RefSrcList,
        pub aiCol: *mut ::core::ffi::c_int,
        pub pIdxCover: *mut crate::sqliteInt_h::IdxCover,
        pub pGroupBy: *mut crate::sqliteInt_h::ExprList,
        pub pSelect: *mut crate::sqliteInt_h::Select,
        pub pRewrite: *mut crate::sqliteInt_h::WindowRewrite,
        pub pConst: *mut crate::sqliteInt_h::WhereConst,
        pub pRename: *mut RenameCtx,
        pub pTab: *mut crate::sqliteInt_h::Table,
        pub pCovIdxCk: *mut crate::sqliteInt_h::CoveringIndexCheck,
        pub pSrcItem: *mut crate::sqliteInt_h::SrcItem,
        pub pFix: *mut crate::sqliteInt_h::DbFixer,
        pub aMem: *mut crate::src::src::vdbe::Mem,
        pub pCheckOnCtx: *mut crate::sqliteInt_h::CheckOnCtx,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct DbFixer {
        pub pParse: *mut crate::sqliteInt_h::Parse,
        pub w: crate::sqliteInt_h::Walker,
        pub pSchema: *mut crate::sqliteInt_h::Schema,
        pub bTemp: crate::src::ext::rtree::rtree::u8_0,
        pub zDb: *const ::core::ffi::c_char,
        pub zType: *const ::core::ffi::c_char,
        pub pName: *const crate::sqliteInt_h::Token,
    }
    
    
    pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Cte {
        pub zName: *mut ::core::ffi::c_char,
        pub pCols: *mut crate::sqliteInt_h::ExprList,
        pub pSelect: *mut crate::sqliteInt_h::Select,
        pub zCteErr: *const ::core::ffi::c_char,
        pub pUse: *mut crate::sqliteInt_h::CteUse,
        pub eM10d: crate::src::ext::rtree::rtree::u8_0,
    }
    
    
    pub const M10d_Yes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const M10d_Any: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const M10d_No: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct With {
        pub nCte: ::core::ffi::c_int,
        pub bView: ::core::ffi::c_int,
        pub pOuter: *mut crate::sqliteInt_h::With,
        pub a: [crate::sqliteInt_h::Cte; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct CteUse {
        pub nUse: ::core::ffi::c_int,
        pub addrM9e: ::core::ffi::c_int,
        pub regRtn: ::core::ffi::c_int,
        pub iCur: ::core::ffi::c_int,
        pub nRowEst: crate::sqliteInt_h::LogEst,
        pub eM10d: crate::src::ext::rtree::rtree::u8_0,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct DbClientData {
        pub pNext: *mut crate::sqliteInt_h::DbClientData,
        pub pData: *mut ::core::ffi::c_void,
        pub xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub zName: [::core::ffi::c_char; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Window {
        pub zName: *mut ::core::ffi::c_char,
        pub zBase: *mut ::core::ffi::c_char,
        pub pPartition: *mut crate::sqliteInt_h::ExprList,
        pub pOrderBy: *mut crate::sqliteInt_h::ExprList,
        pub eFrmType: crate::src::ext::rtree::rtree::u8_0,
        pub eStart: crate::src::ext::rtree::rtree::u8_0,
        pub eEnd: crate::src::ext::rtree::rtree::u8_0,
        pub bImplicitFrame: crate::src::ext::rtree::rtree::u8_0,
        pub eExclude: crate::src::ext::rtree::rtree::u8_0,
        pub pStart: *mut crate::sqliteInt_h::Expr,
        pub pEnd: *mut crate::sqliteInt_h::Expr,
        pub ppThis: *mut *mut crate::sqliteInt_h::Window,
        pub pNextWin: *mut crate::sqliteInt_h::Window,
        pub pFilter: *mut crate::sqliteInt_h::Expr,
        pub pWFunc: *mut crate::sqliteInt_h::FuncDef,
        pub iEphCsr: ::core::ffi::c_int,
        pub regAccum: ::core::ffi::c_int,
        pub regResult: ::core::ffi::c_int,
        pub csrApp: ::core::ffi::c_int,
        pub regApp: ::core::ffi::c_int,
        pub regPart: ::core::ffi::c_int,
        pub pOwner: *mut crate::sqliteInt_h::Expr,
        pub nBufferCol: ::core::ffi::c_int,
        pub iArgCol: ::core::ffi::c_int,
        pub regOne: ::core::ffi::c_int,
        pub regStartRowid: ::core::ffi::c_int,
        pub regEndRowid: ::core::ffi::c_int,
        pub bExprArgs: crate::src::ext::rtree::rtree::u8_0,
    }
    
    
    pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_NOMEM;
    
    
    pub const SQLITE_IOERR_NOMEM_BKPT: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_IOERR_NOMEM;
    
    
    pub const EXP754: crate::src::ext::rtree::rtree::u64_0 = (0x7ff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 52 as ::core::ffi::c_int;
    
    
    pub const MAN754: crate::src::ext::rtree::rtree::u64_0 =
        ((1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 52 as ::core::ffi::c_int).wrapping_sub(1 as crate::src::ext::rtree::rtree::u64_0);
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PrintfArguments {
        pub nArg: ::core::ffi::c_int,
        pub nUsed: ::core::ffi::c_int,
        pub apArg: *mut *mut crate::vdbeInt_h::sqlite3_value,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct FpDecode {
        pub sign: ::core::ffi::c_char,
        pub isSpecial: ::core::ffi::c_char,
        pub n: ::core::ffi::c_int,
        pub iDP: ::core::ffi::c_int,
        pub z: *mut ::core::ffi::c_char,
        pub zBuf: [::core::ffi::c_char; 24],
    }
    
    
    pub const ONEPASS_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const ONEPASS_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const ONEPASS_MULTI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ECEL_DUP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ECEL_FACTOR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ECEL_REF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ECEL_OMITREF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const LOCATE_VIEW: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const LOCATE_NOERR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_ROWID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_EPH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_INDEX_ASC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_INDEX_DESC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_NOOP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_NOOP_OK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_MEMBERSHIP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const IN_INDEX_LOOP: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
}pub mod geopoly_c {
    pub type GeoCoord =  ::core::ffi::c_float;
    use crate::src::ext::rtree::rtree::RtreeCoord;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct GeoPoly {
        pub nVertex: ::core::ffi::c_int,
        pub hdr: [::core::ffi::c_uchar; 4],
        pub a: [crate::geopoly_c::GeoCoord; 8],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct GeoParse {
        pub z: *const ::core::ffi::c_uchar,
        pub nVertex: ::core::ffi::c_int,
        pub nAlloc: ::core::ffi::c_int,
        pub nErr: ::core::ffi::c_int,
        pub a: *mut crate::geopoly_c::GeoCoord,
    }
    
    
    pub const GEOPOLY_PI: ::core::ffi::c_double = 3.1415926535897932385f64;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct GeoBBox {
        pub isInit: ::core::ffi::c_int,
        pub a: [RtreeCoord; 4],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct GeoEvent {
        pub x: ::core::ffi::c_double,
        pub eType: ::core::ffi::c_int,
        pub pSeg: *mut crate::geopoly_c::GeoSegment,
        pub pNext: *mut crate::geopoly_c::GeoEvent,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct GeoSegment {
        pub C: ::core::ffi::c_double,
        pub B: ::core::ffi::c_double,
        pub y: ::core::ffi::c_double,
        pub y0: ::core::ffi::c_float,
        pub side: ::core::ffi::c_uchar,
        pub idx: ::core::ffi::c_uint,
        pub pNext: *mut crate::geopoly_c::GeoSegment,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct GeoOverlap {
        pub aEvent: *mut crate::geopoly_c::GeoEvent,
        pub aSegment: *mut crate::geopoly_c::GeoSegment,
        pub nEvent: ::core::ffi::c_int,
        pub nSegment: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_2 {
        pub xFunc: Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        pub nArg: ::core::ffi::c_schar,
        pub bPure: ::core::ffi::c_uchar,
        pub zName: *const ::core::ffi::c_char,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_1 {
        pub xStep: Option<
            unsafe extern "C" fn(
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        pub xFinal: Option<unsafe extern "C" fn(*mut crate::vdbeInt_h::sqlite3_context) -> ()>,
        pub zName: *const ::core::ffi::c_char,
    }
}pub mod __stddef_null_h {
    pub const NULL:  *mut ::core::ffi::c_void =
         ::core::ptr::null_mut::<::core::ffi::c_void>();
    
    
    pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
}pub mod fts3Int_h {
    #[cfg(feature = "test")]
    extern "C" {
        pub fn sqlite3Fts3InitTerm(db:  *mut crate::sqliteInt_h::sqlite3)
        ->  ::core::ffi::c_int;
    }
    pub use crate::src::ext::fts3::fts3_snippet::MatchinfoBuffer;
    pub use crate::src::ext::fts3::fts3_write::Fts3DeferredToken;
    pub use crate::src::ext::fts3::fts3_write::Fts3SegReader;
    
    
    pub const SQLITE_FTS3_MAX_EXPR_DEPTH: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    
    pub const FTS3_MERGE_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    
    
    pub const FTS3_MAX_PENDING_DATA: ::core::ffi::c_int =
        1 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
    
    
    pub const FTS3_VARINT_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const FTS3_BUFFER_PADDING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGDIR_MAXLEVEL: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
    
    
    pub const POS_COLUMN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const POS_END: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const LARGEST_INT64: crate::src::ext::rtree::rtree::i64_0 = 0xffffffff as crate::src::ext::rtree::rtree::i64_0
        | (0x7fffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0) << 32 as ::core::ffi::c_int;
    
    
    pub const SMALLEST_INT64: crate::src::ext::rtree::rtree::i64_0 = -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 - crate::fts3Int_h::LARGEST_INT64;
    
    
    pub const FTS_CORRUPT_VTAB: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_CORRUPT_VTAB;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3Table {
        pub base: crate::sqlite3_h::sqlite3_vtab,
        pub db: *mut crate::sqliteInt_h::sqlite3,
        pub zDb: *const ::core::ffi::c_char,
        pub zName: *const ::core::ffi::c_char,
        pub nColumn: ::core::ffi::c_int,
        pub azColumn: *mut *mut ::core::ffi::c_char,
        pub abNotindexed: *mut crate::src::ext::rtree::rtree::u8_0,
        pub pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        pub zContentTbl: *mut ::core::ffi::c_char,
        pub zLanguageid: *mut ::core::ffi::c_char,
        pub nAutoincrmerge: ::core::ffi::c_int,
        pub nLeafAdd: crate::src::ext::rtree::rtree::u32_0,
        pub bLock: ::core::ffi::c_int,
        pub aStmt: [*mut crate::sqlite3_h::sqlite3_stmt; 40],
        pub pSeekStmt: *mut crate::sqlite3_h::sqlite3_stmt,
        pub zReadExprlist: *mut ::core::ffi::c_char,
        pub zWriteExprlist: *mut ::core::ffi::c_char,
        pub nNodeSize: ::core::ffi::c_int,
        pub bFts4: crate::src::ext::rtree::rtree::u8_0,
        pub bHasStat: crate::src::ext::rtree::rtree::u8_0,
        pub bHasDocsize: crate::src::ext::rtree::rtree::u8_0,
        pub bDescIdx: crate::src::ext::rtree::rtree::u8_0,
        pub bIgnoreSavepoint: crate::src::ext::rtree::rtree::u8_0,
        pub nPgsz: ::core::ffi::c_int,
        pub zSegmentsTbl: *mut ::core::ffi::c_char,
        pub pSegments: *mut crate::sqlite3_h::sqlite3_blob,
        pub iSavepoint: ::core::ffi::c_int,
        pub nIndex: ::core::ffi::c_int,
        pub aIndex: *mut crate::fts3Int_h::Fts3Index,
        pub nMaxPendingData: ::core::ffi::c_int,
        pub nPendingData: ::core::ffi::c_int,
        pub iPrevDocid: crate::sqlite3_h::sqlite_int64,
        pub iPrevLangid: ::core::ffi::c_int,
        pub bPrevDelete: ::core::ffi::c_int,
        pub bNoIncrDoclist: ::core::ffi::c_int,
        pub nMergeCount: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3Index {
        pub nPrefix: ::core::ffi::c_int,
        pub hPending: crate::src::ext::fts3::fts3_hash::Fts3Hash,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3Cursor {
        pub base: crate::sqlite3_h::sqlite3_vtab_cursor,
        pub eSearch: crate::src::fts5::i16_0,
        pub isEof: crate::src::ext::rtree::rtree::u8_0,
        pub isRequireSeek: crate::src::ext::rtree::rtree::u8_0,
        pub bSeekStmt: crate::src::ext::rtree::rtree::u8_0,
        pub pStmt: *mut crate::sqlite3_h::sqlite3_stmt,
        pub pExpr: *mut crate::fts3Int_h::Fts3Expr,
        pub iLangid: ::core::ffi::c_int,
        pub nPhrase: ::core::ffi::c_int,
        pub pDeferred: *mut crate::fts3Int_h::Fts3DeferredToken,
        pub iPrevId: crate::sqlite3_h::sqlite3_int64,
        pub pNextId: *mut ::core::ffi::c_char,
        pub aDoclist: *mut ::core::ffi::c_char,
        pub nDoclist: ::core::ffi::c_int,
        pub bDesc: crate::src::ext::rtree::rtree::u8_0,
        pub eEvalmode: ::core::ffi::c_int,
        pub nRowAvg: ::core::ffi::c_int,
        pub nDoc: crate::sqlite3_h::sqlite3_int64,
        pub iMinDocid: crate::src::ext::rtree::rtree::i64_0,
        pub iMaxDocid: crate::src::ext::rtree::rtree::i64_0,
        pub isMatchinfoNeeded: ::core::ffi::c_int,
        pub pMIBuffer: *mut crate::fts3Int_h::MatchinfoBuffer,
    }
    
    
    pub const FTS3_FULLSCAN_SEARCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const FTS3_DOCID_SEARCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const FTS3_FULLTEXT_SEARCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const FTS3_HAVE_LANGID: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const FTS3_HAVE_DOCID_GE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const FTS3_HAVE_DOCID_LE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3Doclist {
        pub aAll: *mut ::core::ffi::c_char,
        pub nAll: ::core::ffi::c_int,
        pub pNextDocid: *mut ::core::ffi::c_char,
        pub iDocid: crate::sqlite3_h::sqlite3_int64,
        pub bFreeList: ::core::ffi::c_int,
        pub pList: *mut ::core::ffi::c_char,
        pub nList: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3PhraseToken {
        pub z: *mut ::core::ffi::c_char,
        pub n: ::core::ffi::c_int,
        pub isPrefix: ::core::ffi::c_int,
        pub bFirst: ::core::ffi::c_int,
        pub pDeferred: *mut crate::fts3Int_h::Fts3DeferredToken,
        pub pSegcsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3Phrase {
        pub doclist: crate::fts3Int_h::Fts3Doclist,
        pub bIncr: ::core::ffi::c_int,
        pub iDoclistToken: ::core::ffi::c_int,
        pub pOrPoslist: *mut ::core::ffi::c_char,
        pub iOrDocid: crate::src::ext::rtree::rtree::i64_0,
        pub nToken: ::core::ffi::c_int,
        pub iColumn: ::core::ffi::c_int,
        pub aToken: [crate::fts3Int_h::Fts3PhraseToken; 0],
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3Expr {
        pub eType: ::core::ffi::c_int,
        pub nNear: ::core::ffi::c_int,
        pub pParent: *mut crate::fts3Int_h::Fts3Expr,
        pub pLeft: *mut crate::fts3Int_h::Fts3Expr,
        pub pRight: *mut crate::fts3Int_h::Fts3Expr,
        pub pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
        pub iDocid: crate::sqlite3_h::sqlite3_int64,
        pub bEof: crate::src::ext::rtree::rtree::u8_0,
        pub bStart: crate::src::ext::rtree::rtree::u8_0,
        pub bDeferred: crate::src::ext::rtree::rtree::u8_0,
        pub iPhrase: ::core::ffi::c_int,
        pub aMI: *mut crate::src::ext::rtree::rtree::u32_0,
    }
    
    
    pub const FTSQUERY_NEAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const FTSQUERY_NOT: ::core::ffi::c_int = 2;
    
    
    pub const FTSQUERY_NOT_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const FTSQUERY_AND: ::core::ffi::c_int = 3;
    
    
    pub const FTSQUERY_AND_1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const FTSQUERY_OR: ::core::ffi::c_int = 4;
    
    
    pub const FTSQUERY_OR_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const FTSQUERY_PHRASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGCURSOR_PENDING: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    
    pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    
    
    pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGMENT_COLUMN_FILTER: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGMENT_PREFIX: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGMENT_SCAN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const FTS3_SEGMENT_FIRST: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3SegFilter {
        pub zTerm: *const ::core::ffi::c_char,
        pub nTerm: ::core::ffi::c_int,
        pub iCol: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3MultiSegReader {
        pub apSegment: *mut *mut crate::fts3Int_h::Fts3SegReader,
        pub nSegment: ::core::ffi::c_int,
        pub nAdvance: ::core::ffi::c_int,
        pub pFilter: *mut crate::fts3Int_h::Fts3SegFilter,
        pub aBuffer: *mut ::core::ffi::c_char,
        pub nBuffer: crate::src::ext::rtree::rtree::i64_0,
        pub iColFilter: ::core::ffi::c_int,
        pub bRestart: ::core::ffi::c_int,
        pub nCost: ::core::ffi::c_int,
        pub bLookup: ::core::ffi::c_int,
        pub zTerm: *mut ::core::ffi::c_char,
        pub nTerm: ::core::ffi::c_int,
        pub aDoclist: *mut ::core::ffi::c_char,
        pub nDoclist: ::core::ffi::c_int,
    }
}pub mod sqlite3_h {
    extern "C" {
        pub type sqlite3_stmt;
        
        
        pub type sqlite3_blob;
        
        
        pub type sqlite3_pcache;
    }
    pub const SQLITE_VERSION:  [::core::ffi::c_char; 7] =
        
        unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"3.51.2\0") };
    
    
    pub const SQLITE_VERSION_NUMBER: ::core::ffi::c_int = 3051002 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SOURCE_ID: [::core::ffi::c_char; 85] = unsafe {
        ::core::mem::transmute::<
            [u8; 85],
            [::core::ffi::c_char; 85],
        >(
            *b"2026-01-09 17:27:48 b270f8339eb13b504d0b2ba154ebca966b7dde08e40c3ed7d559749818cbalt1\0",
        )
    };
    
    
    pub type sqlite_int64 = ::core::ffi::c_longlong;
    
    
    pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
    
    
    pub type sqlite3_int64 = crate::sqlite3_h::sqlite_int64;
    
    
    pub type sqlite3_uint64 = crate::sqlite3_h::sqlite_uint64;
    
    
    pub type sqlite3_callback = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >;
    
    
    pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INTERNAL: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_INTERNAL_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PERM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INTERRUPT_1: ::core::ffi::c_int = 9;
    
    
    pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTFOUND_1: ::core::ffi::c_int = 12;
    
    
    pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FULL_1: ::core::ffi::c_int = 13;
    
    
    pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PROTOCOL: ::core::ffi::c_int = 15;
    
    
    pub const SQLITE_PROTOCOL_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    
    
    pub const SQLITE_EMPTY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    
    
    pub const SQLITE_EMPTY_1: ::core::ffi::c_int = 16;
    
    
    pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SCHEMA_1: ::core::ffi::c_int = 17;
    
    
    pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TOOBIG_1: ::core::ffi::c_int = 18;
    
    
    pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MISMATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MISMATCH_1: ::core::ffi::c_int = 20;
    
    
    pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOLFS: ::core::ffi::c_int = 22;
    
    
    pub const SQLITE_AUTH: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    
    
    pub const SQLITE_AUTH_1: ::core::ffi::c_int = 23;
    
    
    pub const SQLITE_FORMAT: ::core::ffi::c_int = 24;
    
    
    pub const SQLITE_RANGE: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTADB: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTADB_1: ::core::ffi::c_int = 26;
    
    
    pub const SQLITE_NOTICE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
    
    
    pub const SQLITE_WARNING: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ROW_1: ::core::ffi::c_int = 100;
    
    
    pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DONE_1: ::core::ffi::c_int = 101;
    
    
    pub const SQLITE_ERROR_MISSING_COLLSEQ: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_ERROR | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ERROR_MISSING_COLLSEQ_1: ::core::ffi::c_int = 257;
    
    
    pub const SQLITE_ERROR_RETRY: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_ERROR | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ERROR_RETRY_1: ::core::ffi::c_int = 513;
    
    
    pub const SQLITE_ERROR_SNAPSHOT: ::core::ffi::c_int = 769;
    
    
    pub const SQLITE_IOERR_READ: ::core::ffi::c_int = 266;
    
    
    pub const SQLITE_IOERR_READ_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = 522;
    
    
    pub const SQLITE_IOERR_SHORT_READ_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_WRITE: ::core::ffi::c_int = 778;
    
    
    pub const SQLITE_IOERR_WRITE_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_FSYNC: ::core::ffi::c_int = 1034;
    
    
    pub const SQLITE_IOERR_DIR_FSYNC: ::core::ffi::c_int = 1290;
    
    
    pub const SQLITE_IOERR_TRUNCATE: ::core::ffi::c_int = 1546;
    
    
    pub const SQLITE_IOERR_FSTAT: ::core::ffi::c_int = 1802;
    
    
    pub const SQLITE_IOERR_FSTAT_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_UNLOCK: ::core::ffi::c_int = 2058;
    
    
    pub const SQLITE_IOERR_UNLOCK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_RDLOCK: ::core::ffi::c_int = 2314;
    
    
    pub const SQLITE_IOERR_RDLOCK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (9 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_DELETE: ::core::ffi::c_int = 2570;
    
    
    pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_ACCESS: ::core::ffi::c_int = 3338;
    
    
    pub const SQLITE_IOERR_ACCESS_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_CHECKRESERVEDLOCK: ::core::ffi::c_int = 3594;
    
    
    pub const SQLITE_IOERR_CHECKRESERVEDLOCK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_LOCK: ::core::ffi::c_int = 3850;
    
    
    pub const SQLITE_IOERR_LOCK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (15 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_CLOSE: ::core::ffi::c_int = 4106;
    
    
    pub const SQLITE_IOERR_CLOSE_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_DIR_CLOSE: ::core::ffi::c_int = 4362;
    
    
    pub const SQLITE_IOERR_SHMOPEN: ::core::ffi::c_int = 4618;
    
    
    pub const SQLITE_IOERR_SHMSIZE: ::core::ffi::c_int = 4874;
    
    
    pub const SQLITE_IOERR_SHMSIZE_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (19 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_SHMLOCK: ::core::ffi::c_int = 5130;
    
    
    pub const SQLITE_IOERR_SHMLOCK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (20 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_SHMMAP: ::core::ffi::c_int = 5386;
    
    
    pub const SQLITE_IOERR_SEEK: ::core::ffi::c_int = 5642;
    
    
    pub const SQLITE_IOERR_DELETE_NOENT: ::core::ffi::c_int = 5898;
    
    
    pub const SQLITE_IOERR_DELETE_NOENT_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (23 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_MMAP: ::core::ffi::c_int = 6154;
    
    
    pub const SQLITE_IOERR_GETTEMPPATH: ::core::ffi::c_int = 6410;
    
    
    pub const SQLITE_IOERR_GETTEMPPATH_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (25 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOERR_CONVPATH: ::core::ffi::c_int = 6666;
    
    
    pub const SQLITE_IOERR_CORRUPTFS: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_IOERR | (33 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LOCKED_SHAREDCACHE: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_LOCKED | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LOCKED_SHAREDCACHE_1: ::core::ffi::c_int = 262;
    
    
    pub const SQLITE_LOCKED_VTAB: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_LOCKED | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BUSY_RECOVERY: ::core::ffi::c_int = 261;
    
    
    pub const SQLITE_BUSY_RECOVERY_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_BUSY | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BUSY_SNAPSHOT: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_BUSY | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BUSY_SNAPSHOT_1: ::core::ffi::c_int = 517;
    
    
    pub const SQLITE_CANTOPEN_NOTEMPDIR: ::core::ffi::c_int = 270;
    
    
    pub const SQLITE_CANTOPEN_ISDIR: ::core::ffi::c_int = 526;
    
    
    pub const SQLITE_CANTOPEN_FULLPATH: ::core::ffi::c_int = 782;
    
    
    pub const SQLITE_CANTOPEN_CONVPATH: ::core::ffi::c_int = 1038;
    
    
    pub const SQLITE_CANTOPEN_SYMLINK: ::core::ffi::c_int = 1550;
    
    
    pub const SQLITE_CANTOPEN_SYMLINK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CANTOPEN | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CORRUPT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CORRUPT_VTAB_1: ::core::ffi::c_int = 267;
    
    
    pub const SQLITE_CORRUPT_SEQUENCE: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CORRUPT | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CORRUPT_INDEX: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CORRUPT | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READONLY_RECOVERY: ::core::ffi::c_int = 264;
    
    
    pub const SQLITE_READONLY_RECOVERY_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_READONLY | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READONLY_ROLLBACK: ::core::ffi::c_int = 776;
    
    
    pub const SQLITE_READONLY_ROLLBACK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_READONLY | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READONLY_DBMOVED: ::core::ffi::c_int = 1032;
    
    
    pub const SQLITE_READONLY_DBMOVED_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_READONLY | (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READONLY_CANTINIT: ::core::ffi::c_int = 1288;
    
    
    pub const SQLITE_READONLY_CANTINIT_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_READONLY | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READONLY_DIRECTORY: ::core::ffi::c_int = 1544;
    
    
    pub const SQLITE_READONLY_DIRECTORY_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_READONLY | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ABORT_ROLLBACK: ::core::ffi::c_int = 516;
    
    
    pub const SQLITE_ABORT_ROLLBACK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_ABORT | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_CHECK: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_CHECK_1: ::core::ffi::c_int = 275;
    
    
    pub const SQLITE_CONSTRAINT_COMMITHOOK: ::core::ffi::c_int = 531;
    
    
    pub const SQLITE_CONSTRAINT_COMMITHOOK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_FOREIGNKEY: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_FOREIGNKEY_1: ::core::ffi::c_int = 787;
    
    
    pub const SQLITE_CONSTRAINT_FUNCTION: ::core::ffi::c_int = 1043;
    
    
    pub const SQLITE_CONSTRAINT_NOTNULL: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_NOTNULL_1: ::core::ffi::c_int = 1299;
    
    
    pub const SQLITE_CONSTRAINT_PRIMARYKEY: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_PRIMARYKEY_1: ::core::ffi::c_int = 1555;
    
    
    pub const SQLITE_CONSTRAINT_TRIGGER: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_TRIGGER_1: ::core::ffi::c_int = 1811;
    
    
    pub const SQLITE_CONSTRAINT_UNIQUE: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_UNIQUE_1: ::core::ffi::c_int = 2067;
    
    
    pub const SQLITE_CONSTRAINT_VTAB: ::core::ffi::c_int = 2323;
    
    
    pub const SQLITE_CONSTRAINT_ROWID: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_ROWID_1: ::core::ffi::c_int = 2579;
    
    
    pub const SQLITE_CONSTRAINT_PINNED: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (11 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONSTRAINT_DATATYPE: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_CONSTRAINT | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTICE_RECOVER_WAL: ::core::ffi::c_int = 283;
    
    
    pub const SQLITE_NOTICE_RECOVER_WAL_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_NOTICE | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTICE_RECOVER_ROLLBACK: ::core::ffi::c_int = 539;
    
    
    pub const SQLITE_NOTICE_RECOVER_ROLLBACK_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_NOTICE | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NOTICE_RBU: ::core::ffi::c_int = 795;
    
    
    pub const SQLITE_WARNING_AUTOINDEX: ::core::ffi::c_int = 284;
    
    
    pub const SQLITE_WARNING_AUTOINDEX_1: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_WARNING | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OK_LOAD_PERMANENTLY: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_OK | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OK_SYMLINK: ::core::ffi::c_int =
        crate::sqlite3_h::SQLITE_OK | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_MEMORY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_TRANSIENT_DB: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_TEMP_JOURNAL: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_SUBJOURNAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_NOMUTEX: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_FULLMUTEX: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_SHAREDCACHE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_PRIVATECACHE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_NOFOLLOW: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_OPEN_EXRESCODE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_ATOMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_SAFE_APPEND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_IMMUTABLE: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IOCAP_SUBPAGE_READ: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LOCK_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LOCK_RESERVED: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_LOCK_PENDING: ::core::ffi::c_int = 3;
    
    
    pub const SQLITE_LOCK_EXCLUSIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SYNC_NORMAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SYNC_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SYNC_DATAONLY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_file {
        pub pMethods: *const crate::sqlite3_h::sqlite3_io_methods,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_io_methods {
        pub iVersion: ::core::ffi::c_int,
        pub xClose: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int>,
        pub xRead: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                crate::sqlite3_h::sqlite3_int64,
            ) -> ::core::ffi::c_int,
        >,
        pub xWrite: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                crate::sqlite3_h::sqlite3_int64,
            ) -> ::core::ffi::c_int,
        >,
        pub xTruncate:
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, crate::sqlite3_h::sqlite3_int64) -> ::core::ffi::c_int>,
        pub xSync: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xFileSize: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, *mut crate::sqlite3_h::sqlite3_int64) -> ::core::ffi::c_int,
        >,
        pub xLock: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xUnlock: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xCheckReservedLock: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, *mut ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xFileControl: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub xSectorSize: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int>,
        pub xDeviceCharacteristics:
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int>,
        pub xShmMap: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub xShmLock: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub xShmBarrier: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file) -> ()>,
        pub xShmUnmap: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xFetch: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                crate::sqlite3_h::sqlite3_int64,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub xUnfetch: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_file,
                crate::sqlite3_h::sqlite3_int64,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    }
    
    
    pub const SQLITE_FCNTL_LOCKSTATE: ::core::ffi::c_int = 1;
    
    
    pub const SQLITE_FCNTL_LOCKSTATE_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_LAST_ERRNO: ::core::ffi::c_int = 4;
    
    
    pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5;
    
    
    pub const SQLITE_FCNTL_SIZE_HINT_1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_CHUNK_SIZE: ::core::ffi::c_int = 6;
    
    
    pub const SQLITE_FCNTL_CHUNK_SIZE_1: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_PERSIST_WAL: ::core::ffi::c_int = 10;
    
    
    pub const SQLITE_FCNTL_PERSIST_WAL_1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_OVERWRITE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_VFSNAME_1: ::core::ffi::c_int = 12;
    
    
    pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 13;
    
    
    pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_BUSYHANDLER: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_TEMPFILENAME: ::core::ffi::c_int = 16;
    
    
    pub const SQLITE_FCNTL_MMAP_SIZE: ::core::ffi::c_int = 18;
    
    
    pub const SQLITE_FCNTL_MMAP_SIZE_1: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_HAS_MOVED: ::core::ffi::c_int = 20;
    
    
    pub const SQLITE_FCNTL_HAS_MOVED_1: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_SYNC: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_COMMIT_PHASETWO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_VFS_POINTER: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_JOURNAL_POINTER: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_PDB: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_LOCK_TIMEOUT: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_DATA_VERSION: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_SIZE_LIMIT: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_CKPT_DONE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_RESERVE_BYTES: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_CKPT_START: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_EXTERNAL_READER: ::core::ffi::c_int = 40;
    
    
    pub const SQLITE_FCNTL_RESET_CACHE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FCNTL_NULL_IO: ::core::ffi::c_int = 43;
    
    
    pub type sqlite3_filename = *const ::core::ffi::c_char;
    
    
    pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn() -> ()>;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_vfs {
        pub iVersion: ::core::ffi::c_int,
        pub szOsFile: ::core::ffi::c_int,
        pub mxPathname: ::core::ffi::c_int,
        pub pNext: *mut crate::sqlite3_h::sqlite3_vfs,
        pub zName: *const ::core::ffi::c_char,
        pub pAppData: *mut ::core::ffi::c_void,
        pub xOpen: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                crate::sqlite3_h::sqlite3_filename,
                *mut crate::sqlite3_h::sqlite3_file,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub xDelete: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub xAccess: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub xFullPathname: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub xDlOpen: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub xDlError: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> (),
        >,
        pub xDlSym: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> Option<unsafe extern "C" fn() -> ()>,
        >,
        pub xDlClose:
            Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vfs, *mut ::core::ffi::c_void) -> ()>,
        pub xRandomness: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub xSleep: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xCurrentTime: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *mut ::core::ffi::c_double,
            ) -> ::core::ffi::c_int,
        >,
        pub xGetLastError: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub xCurrentTimeInt64: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vfs, *mut crate::sqlite3_h::sqlite3_int64) -> ::core::ffi::c_int,
        >,
        pub xSetSystemCall: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
                crate::sqlite3_h::sqlite3_syscall_ptr,
            ) -> ::core::ffi::c_int,
        >,
        pub xGetSystemCall: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
            ) -> crate::sqlite3_h::sqlite3_syscall_ptr,
        >,
        pub xNextSystemCall: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vfs,
                *const ::core::ffi::c_char,
            ) -> *const ::core::ffi::c_char,
        >,
    }
    
    
    pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ACCESS_READWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SHM_UNLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SHM_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SHM_SHARED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SHM_EXCLUSIVE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SHM_NLOCK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_mem_methods {
        pub xMalloc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void>,
        pub xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub xRealloc: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub xSize: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub xRoundup: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub xInit: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub pAppData: *mut ::core::ffi::c_void,
    }
    
    
    pub const SQLITE_CONFIG_SINGLETHREAD: ::core::ffi::c_int = 1;
    
    
    pub const SQLITE_CONFIG_MULTITHREAD: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_CONFIG_SERIALIZED: ::core::ffi::c_int = 3;
    
    
    pub const SQLITE_CONFIG_MALLOC: ::core::ffi::c_int = 4;
    
    
    pub const SQLITE_CONFIG_MALLOC_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONFIG_GETMALLOC: ::core::ffi::c_int = 5;
    
    
    pub const SQLITE_CONFIG_PAGECACHE: ::core::ffi::c_int = 7;
    
    
    pub const SQLITE_CONFIG_MEMSTATUS: ::core::ffi::c_int = 9;
    
    
    pub const SQLITE_CONFIG_MUTEX: ::core::ffi::c_int = 10;
    
    
    pub const SQLITE_CONFIG_GETMUTEX: ::core::ffi::c_int = 11;
    
    
    pub const SQLITE_CONFIG_LOOKASIDE: ::core::ffi::c_int = 13;
    
    
    pub const SQLITE_CONFIG_PCACHE: ::core::ffi::c_int = 14;
    
    
    pub const SQLITE_CONFIG_GETPCACHE: ::core::ffi::c_int = 15;
    
    
    pub const SQLITE_CONFIG_LOG: ::core::ffi::c_int = 16;
    
    
    pub const SQLITE_CONFIG_URI: ::core::ffi::c_int = 17;
    
    
    pub const SQLITE_CONFIG_PCACHE2: ::core::ffi::c_int = 18;
    
    
    pub const SQLITE_CONFIG_PCACHE2_1: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CONFIG_GETPCACHE2: ::core::ffi::c_int = 19;
    
    
    pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: ::core::ffi::c_int = 20;
    
    
    pub const SQLITE_CONFIG_MMAP_SIZE: ::core::ffi::c_int = 22;
    
    
    pub const SQLITE_CONFIG_PCACHE_HDRSZ: ::core::ffi::c_int = 24;
    
    
    pub const SQLITE_CONFIG_PMASZ: ::core::ffi::c_int = 25;
    
    
    pub const SQLITE_CONFIG_STMTJRNL_SPILL: ::core::ffi::c_int = 26;
    
    
    pub const SQLITE_CONFIG_SMALL_MALLOC: ::core::ffi::c_int = 27;
    
    
    pub const SQLITE_CONFIG_MEMDB_MAXSIZE: ::core::ffi::c_int = 29;
    
    
    pub const SQLITE_CONFIG_ROWID_IN_VIEW: ::core::ffi::c_int = 30;
    
    
    pub const SQLITE_DBCONFIG_MAINDBNAME: ::core::ffi::c_int = 1000;
    
    
    pub const SQLITE_DBCONFIG_LOOKASIDE: ::core::ffi::c_int = 1001;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_FKEY: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: ::core::ffi::c_int = 1003 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: ::core::ffi::c_int =
        1004 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: ::core::ffi::c_int =
        1005 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE: ::core::ffi::c_int = 1006 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_QPSG: ::core::ffi::c_int = 1007 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_TRIGGER_EQP: ::core::ffi::c_int = 1008 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_RESET_DATABASE: ::core::ffi::c_int = 1009 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_DEFENSIVE: ::core::ffi::c_int = 1010 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA: ::core::ffi::c_int = 1011 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE: ::core::ffi::c_int = 1012 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_DQS_DML: ::core::ffi::c_int = 1013 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_DQS_DDL: ::core::ffi::c_int = 1014 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_VIEW: ::core::ffi::c_int = 1015 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT: ::core::ffi::c_int = 1016 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA: ::core::ffi::c_int = 1017 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_STMT_SCANSTATUS: ::core::ffi::c_int = 1018 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_REVERSE_SCANORDER: ::core::ffi::c_int = 1019 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE: ::core::ffi::c_int = 1020 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE: ::core::ffi::c_int = 1021 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBCONFIG_ENABLE_COMMENTS: ::core::ffi::c_int = 1022 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DENY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_INDEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_TABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_TEMP_INDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_TEMP_TABLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_TEMP_TRIGGER: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_TEMP_VIEW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_TRIGGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_VIEW: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_INDEX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_TABLE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_TEMP_INDEX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_TEMP_TABLE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_TEMP_TRIGGER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_TEMP_VIEW: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_TRIGGER: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_VIEW: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PRAGMA: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
    
    
    pub const SQLITE_READ: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SELECT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRANSACTION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    
    
    pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ATTACH: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DETACH: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ALTER_TABLE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
    
    
    pub const SQLITE_REINDEX: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ANALYZE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CREATE_VTABLE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DROP_VTABLE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FUNCTION: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SAVEPOINT: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_RECURSIVE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRACE_STMT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRACE_PROFILE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRACE_ROW: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TRACE_CLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_SQL_LENGTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_EXPR_DEPTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_COMPOUND_SELECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_VDBE_OP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_FUNCTION_ARG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_ATTACHED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_TRIGGER_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_LIMIT_WORKER_THREADS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PREPARE_PERSISTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PREPARE_NO_VTAB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_PREPARE_DONT_LOG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INTEGER_1: ::core::ffi::c_int = 1;
    
    
    pub const SQLITE_FLOAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FLOAT_1: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_BLOB_1: ::core::ffi::c_int = 4;
    
    
    pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_NULL_1: ::core::ffi::c_int = 5;
    
    
    pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TEXT_1: ::core::ffi::c_int = 3;
    
    
    pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_UTF16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_ANY_1: ::core::ffi::c_int = 5;
    
    
    pub const SQLITE_UTF16_ALIGNED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SELFORDER1: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
    
    
    pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    
    
    pub const SQLITE_STATIC: crate::sqlite3_h::sqlite3_destructor_type = None;
    
    
    pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TXN_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_module {
        pub iVersion: ::core::ffi::c_int,
        pub xCreate: Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const *const ::core::ffi::c_char,
                *mut *mut crate::sqlite3_h::sqlite3_vtab,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub xConnect: Option<
            unsafe extern "C" fn(
                *mut crate::sqliteInt_h::sqlite3,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const *const ::core::ffi::c_char,
                *mut *mut crate::sqlite3_h::sqlite3_vtab,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub xBestIndex: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab, *mut crate::sqlite3_h::sqlite3_index_info) -> ::core::ffi::c_int,
        >,
        pub xDisconnect: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
        pub xDestroy: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
        pub xOpen: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab,
                *mut *mut crate::sqlite3_h::sqlite3_vtab_cursor,
            ) -> ::core::ffi::c_int,
        >,
        pub xClose: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
        pub xFilter: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >,
        pub xNext: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
        pub xEof: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
        pub xColumn: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                *mut crate::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub xRowid: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                *mut crate::sqlite3_h::sqlite3_int64,
            ) -> ::core::ffi::c_int,
        >,
        pub xUpdate: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab,
                ::core::ffi::c_int,
                *mut *mut crate::vdbeInt_h::sqlite3_value,
                *mut crate::sqlite3_h::sqlite3_int64,
            ) -> ::core::ffi::c_int,
        >,
        pub xBegin: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
        pub xSync: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
        pub xCommit: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
        pub xRollback: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
        pub xFindFunction: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *mut Option<
                    unsafe extern "C" fn(
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >,
                *mut *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pub xRename: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub xSavepoint: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xRelease: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xRollbackTo: Option<
            unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub xShadowName:
            Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
        pub xIntegrity: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_vtab,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_index_info {
        pub nConstraint: ::core::ffi::c_int,
        pub aConstraint: *mut crate::sqlite3_h::sqlite3_index_constraint,
        pub nOrderBy: ::core::ffi::c_int,
        pub aOrderBy: *mut crate::sqlite3_h::sqlite3_index_orderby,
        pub aConstraintUsage: *mut crate::sqlite3_h::sqlite3_index_constraint_usage,
        pub idxNum: ::core::ffi::c_int,
        pub idxStr: *mut ::core::ffi::c_char,
        pub needToFreeIdxStr: ::core::ffi::c_int,
        pub orderByConsumed: ::core::ffi::c_int,
        pub estimatedCost: ::core::ffi::c_double,
        pub estimatedRows: crate::sqlite3_h::sqlite3_int64,
        pub idxFlags: ::core::ffi::c_int,
        pub colUsed: crate::sqlite3_h::sqlite3_uint64,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_index_constraint {
        pub iColumn: ::core::ffi::c_int,
        pub op: ::core::ffi::c_uchar,
        pub usable: ::core::ffi::c_uchar,
        pub iTermOffset: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_index_orderby {
        pub iColumn: ::core::ffi::c_int,
        pub desc: ::core::ffi::c_uchar,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_index_constraint_usage {
        pub argvIndex: ::core::ffi::c_int,
        pub omit: ::core::ffi::c_uchar,
    }
    
    
    pub const SQLITE_INDEX_SCAN_UNIQUE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_SCAN_HEX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_EQ_1: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_GT_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_LE_1: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_LT_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_GE_1: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_MATCH_1: ::core::ffi::c_int = 64;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_LIKE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_GLOB: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_REGEXP: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_NE: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_ISNOT: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_ISNOTNULL: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_ISNULL: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_IS: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_LIMIT: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_OFFSET: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
    
    
    pub const SQLITE_INDEX_CONSTRAINT_FUNCTION: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_vtab {
        pub pModule: *const crate::sqlite3_h::sqlite3_module,
        pub nRef: ::core::ffi::c_int,
        pub zErrMsg: *mut ::core::ffi::c_char,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_vtab_cursor {
        pub pVtab: *mut crate::sqlite3_h::sqlite3_vtab,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_mutex_methods {
        pub xMutexInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        pub xMutexEnd: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        pub xMutexAlloc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex>,
        pub xMutexFree: Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
        pub xMutexEnter: Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
        pub xMutexTry: Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int>,
        pub xMutexLeave: Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
        pub xMutexHeld: Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int>,
        pub xMutexNotheld: Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int>,
    }
    
    
    pub const SQLITE_MUTEX_FAST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_FAST_1: ::core::ffi::c_int = 0;
    
    
    pub const SQLITE_MUTEX_RECURSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_RECURSIVE_1: ::core::ffi::c_int = 1;
    
    
    pub const SQLITE_MUTEX_STATIC_MAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_STATIC_MEM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_STATIC_OPEN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_STATIC_PRNG: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_STATIC_LRU: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_STATIC_PMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_MUTEX_STATIC_VFS1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SQLITE_TESTCTRL_PRNG_SAVE: ::core::ffi::c_int = 5;
    
    
    pub const SQLITE_TESTCTRL_PRNG_RESTORE: ::core::ffi::c_int = 6;
    
    
    pub const SQLITE_TESTCTRL_FK_NO_ACTION: ::core::ffi::c_int = 7;
    
    
    pub const SQLITE_TESTCTRL_BITVEC_TEST: ::core::ffi::c_int = 8;
    
    
    pub const SQLITE_TESTCTRL_FAULT_INSTALL: ::core::ffi::c_int = 9;
    
    
    pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS: ::core::ffi::c_int = 10;
    
    
    pub const SQLITE_TESTCTRL_PENDING_BYTE: ::core::ffi::c_int = 11;
    
    
    pub const SQLITE_TESTCTRL_ASSERT: ::core::ffi::c_int = 12;
    
    
    pub const SQLITE_TESTCTRL_ALWAYS: ::core::ffi::c_int = 13;
    
    
    pub const SQLITE_TESTCTRL_JSON_SELFCHECK: ::core::ffi::c_int = 14;
    
    
    pub const SQLITE_TESTCTRL_OPTIMIZATIONS: ::core::ffi::c_int = 15;
    
    
    pub const SQLITE_TESTCTRL_GETOPT: ::core::ffi::c_int = 16;
    
    
    pub const SQLITE_TESTCTRL_INTERNAL_FUNCTIONS: ::core::ffi::c_int = 17;
    
    
    pub const SQLITE_TESTCTRL_LOCALTIME_FAULT: ::core::ffi::c_int = 18;
    
    
    pub const SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD: ::core::ffi::c_int = 19;
    
    
    pub const SQLITE_TESTCTRL_NEVER_CORRUPT: ::core::ffi::c_int = 20;
    
    
    pub const SQLITE_TESTCTRL_VDBE_COVERAGE: ::core::ffi::c_int = 21;
    
    
    pub const SQLITE_TESTCTRL_BYTEORDER: ::core::ffi::c_int = 22;
    
    
    pub const SQLITE_TESTCTRL_ISINIT: ::core::ffi::c_int = 23;
    
    
    pub const SQLITE_TESTCTRL_SORTER_MMAP: ::core::ffi::c_int = 24;
    
    
    pub const SQLITE_TESTCTRL_IMPOSTER: ::core::ffi::c_int = 25;
    
    
    pub const SQLITE_TESTCTRL_RESULT_INTREAL: ::core::ffi::c_int = 27;
    
    
    pub const SQLITE_TESTCTRL_PRNG_SEED: ::core::ffi::c_int = 28;
    
    
    pub const SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS: ::core::ffi::c_int = 29;
    
    
    pub const SQLITE_TESTCTRL_SEEK_COUNT: ::core::ffi::c_int = 30;
    
    
    pub const SQLITE_TESTCTRL_TRACEFLAGS: ::core::ffi::c_int = 31;
    
    
    pub const SQLITE_TESTCTRL_LOGEST: ::core::ffi::c_int = 33;
    
    
    pub const SQLITE_STATUS_MEMORY_USED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATUS_PAGECACHE_USED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATUS_MALLOC_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATUS_PAGECACHE_SIZE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STATUS_MALLOC_COUNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBSTATUS_LOOKASIDE_USED: ::core::ffi::c_int = 0;
    
    
    pub const SQLITE_DBSTATUS_CACHE_USED: ::core::ffi::c_int = 1;
    
    
    pub const SQLITE_DBSTATUS_SCHEMA_USED: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_DBSTATUS_STMT_USED: ::core::ffi::c_int = 3;
    
    
    pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: ::core::ffi::c_int = 5;
    
    
    pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: ::core::ffi::c_int = 6;
    
    
    pub const SQLITE_DBSTATUS_CACHE_HIT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBSTATUS_CACHE_HIT_1: ::core::ffi::c_int = 7;
    
    
    pub const SQLITE_DBSTATUS_CACHE_MISS: ::core::ffi::c_int = 8;
    
    
    pub const SQLITE_DBSTATUS_CACHE_WRITE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBSTATUS_DEFERRED_FKS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBSTATUS_DEFERRED_FKS_1: ::core::ffi::c_int = 10;
    
    
    pub const SQLITE_DBSTATUS_CACHE_USED_SHARED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DBSTATUS_CACHE_SPILL: ::core::ffi::c_int = 12;
    
    
    pub const SQLITE_DBSTATUS_TEMPBUF_SPILL: ::core::ffi::c_int = 13;
    
    
    pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_SORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_AUTOINDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_VM_STEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_REPREPARE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_RUN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_FILTER_MISS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_FILTER_HIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    
    pub const SQLITE_STMTSTATUS_MEMUSED: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_pcache_page {
        pub pBuf: *mut ::core::ffi::c_void,
        pub pExtra: *mut ::core::ffi::c_void,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_pcache_methods2 {
        pub iVersion: ::core::ffi::c_int,
        pub pArg: *mut ::core::ffi::c_void,
        pub xInit: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub xCreate: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> *mut crate::sqlite3_h::sqlite3_pcache,
        >,
        pub xCachesize: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_pcache, ::core::ffi::c_int) -> ()>,
        pub xPagecount: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_pcache) -> ::core::ffi::c_int>,
        pub xFetch: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_pcache,
                ::core::ffi::c_uint,
                ::core::ffi::c_int,
            ) -> *mut crate::sqlite3_h::sqlite3_pcache_page,
        >,
        pub xUnpin: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_pcache,
                *mut crate::sqlite3_h::sqlite3_pcache_page,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub xRekey: Option<
            unsafe extern "C" fn(
                *mut crate::sqlite3_h::sqlite3_pcache,
                *mut crate::sqlite3_h::sqlite3_pcache_page,
                ::core::ffi::c_uint,
                ::core::ffi::c_uint,
            ) -> (),
        >,
        pub xTruncate: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_pcache, ::core::ffi::c_uint) -> ()>,
        pub xDestroy: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_pcache) -> ()>,
        pub xShrink: Option<unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_pcache) -> ()>,
    }
    
    
    pub const SQLITE_CHECKPOINT_NOOP: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    
    pub const SQLITE_CHECKPOINT_PASSIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CHECKPOINT_FULL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CHECKPOINT_RESTART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_CHECKPOINT_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTAB_CONSTRAINT_SUPPORT_1: ::core::ffi::c_int = 1;
    
    
    pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTAB_INNOCUOUS_1: ::core::ffi::c_int = 2;
    
    
    pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTAB_DIRECTONLY_1: ::core::ffi::c_int = 3;
    
    
    pub const SQLITE_VTAB_USES_ALL_SCHEMAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const SQLITE_VTAB_USES_ALL_SCHEMAS_1: ::core::ffi::c_int = 4;
    
    
    pub const SQLITE_ROLLBACK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_FAIL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const SQLITE_REPLACE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const SQLITE_SERIALIZE_NOCOPY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DESERIALIZE_FREEONCLOSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DESERIALIZE_RESIZEABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    
    pub const SQLITE_DESERIALIZE_READONLY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub const CARRAY_INT32: ::core::ffi::c_int = 0;
    
    
    pub const CARRAY_INT64: ::core::ffi::c_int = 1;
    
    
    pub const CARRAY_DOUBLE: ::core::ffi::c_int = 2;
    
    
    pub const CARRAY_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    
    pub const CARRAY_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    
    pub type sqlite3_rtree_dbl = ::core::ffi::c_double;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_rtree_geometry {
        pub pContext: *mut ::core::ffi::c_void,
        pub nParam: ::core::ffi::c_int,
        pub aParam: *mut crate::sqlite3_h::sqlite3_rtree_dbl,
        pub pUser: *mut ::core::ffi::c_void,
        pub xDelUser: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sqlite3_rtree_query_info {
        pub pContext: *mut ::core::ffi::c_void,
        pub nParam: ::core::ffi::c_int,
        pub aParam: *mut crate::sqlite3_h::sqlite3_rtree_dbl,
        pub pUser: *mut ::core::ffi::c_void,
        pub xDelUser: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub aCoord: *mut crate::sqlite3_h::sqlite3_rtree_dbl,
        pub anQueue: *mut ::core::ffi::c_uint,
        pub nCoord: ::core::ffi::c_int,
        pub iLevel: ::core::ffi::c_int,
        pub mxLevel: ::core::ffi::c_int,
        pub iRowid: crate::sqlite3_h::sqlite3_int64,
        pub rParentScore: crate::sqlite3_h::sqlite3_rtree_dbl,
        pub eParentWithin: ::core::ffi::c_int,
        pub eWithin: ::core::ffi::c_int,
        pub rScore: crate::sqlite3_h::sqlite3_rtree_dbl,
        pub apSqlParam: *mut *mut crate::vdbeInt_h::sqlite3_value,
    }
    
    
    pub const NOT_WITHIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    pub const PARTLY_WITHIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    
    pub const FULLY_WITHIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
}pub mod stdlib {
    extern "C" {
        pub fn __ctype_b_loc()
        ->  *mut *const ::core::ffi::c_ushort;
        pub fn acos(__x:  ::core::ffi::c_double)
        ->  ::core::ffi::c_double;
        
        
        pub fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn atan2(
            __y: ::core::ffi::c_double,
            __x: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;
        
        
        pub fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn cosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn sinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn tanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn acosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn asinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn atanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double)
            -> ::core::ffi::c_double;
        
        
        pub fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        
        
        pub fn fmod(
            __x: ::core::ffi::c_double,
            __y: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;
        
        
        pub fn trunc(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn pthread_create(__newthread:  *mut crate::stdlib::pthread_t,
        __attr:  *const crate::stdlib::pthread_attr_t,
        __start_routine:
             Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            >, __arg:  *mut ::core::ffi::c_void)
        ->  ::core::ffi::c_int;
        
        
        pub fn pthread_mutex_init(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
            __mutexattr: *const crate::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutex_destroy(__mutex: *mut crate::stdlib::pthread_mutex_t) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutex_trylock(__mutex: *mut crate::stdlib::pthread_mutex_t) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutex_lock(__mutex: *mut crate::stdlib::pthread_mutex_t) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutex_unlock(__mutex: *mut crate::stdlib::pthread_mutex_t) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutexattr_init(__attr: *mut crate::stdlib::pthread_mutexattr_t) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutexattr_destroy(__attr: *mut crate::stdlib::pthread_mutexattr_t) -> ::core::ffi::c_int;
        
        
        pub fn pthread_mutexattr_settype(
            __attr: *mut crate::stdlib::pthread_mutexattr_t,
            __kind: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn stat(__file:  *const ::core::ffi::c_char,
        __buf:  *mut crate::stdlib::stat)
        ->  ::core::ffi::c_int;
        
        
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut crate::stdlib::stat) -> ::core::ffi::c_int;
        
        
        pub fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut crate::stdlib::stat) -> ::core::ffi::c_int;
        pub static mut stdout:  *mut crate::stdlib::FILE;
        
        
        pub fn fflush(__stream: *mut crate::stdlib::FILE) -> ::core::ffi::c_int;
        
        
        pub fn fprintf(
            __stream: *mut crate::stdlib::FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn strcspn(__s:  *const ::core::ffi::c_char,
        __reject:  *const ::core::ffi::c_char)
        ->  ::core::ffi::c_ulong;
        
        
        pub fn strspn(
            __s: *const ::core::ffi::c_char,
            __accept: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        pub type _IO_marker;
        
        
        pub type _IO_codecvt;
        
        
        pub type _IO_wide_data;
    
        #[cfg(feature = "test")]
        pub fn TclFreeObj(objPtr:  *mut crate::stdlib::Tcl_Obj);
        
        #[cfg(feature = "test")]
        pub fn Tcl_ListObjAppendElement(
            interp: *mut crate::stdlib::Tcl_Interp,
            listPtr: *mut crate::stdlib::Tcl_Obj,
            objPtr: *mut crate::stdlib::Tcl_Obj,
        ) -> ::core::ffi::c_int;
        
        #[cfg(feature = "test")]
        pub fn Tcl_NewObj() -> *mut crate::stdlib::Tcl_Obj;
        
        #[cfg(feature = "test")]
        pub fn Tcl_NewStringObj(
            bytes: *const ::core::ffi::c_char,
            length: crate::stdlib::Tcl_Size,
        ) -> *mut crate::stdlib::Tcl_Obj;
        
        #[cfg(feature = "test")]
        pub fn Tcl_NewWideIntObj(wideValue: crate::stdlib::Tcl_WideInt) -> *mut crate::stdlib::Tcl_Obj;
        
        #[cfg(feature = "test")]
        pub fn Tcl_GetStringFromObj(
            objPtr: *mut crate::stdlib::Tcl_Obj,
            lengthPtr: *mut crate::stdlib::Tcl_Size,
        ) -> *mut ::core::ffi::c_char;
        pub type Tcl_Interp;
    
        pub fn gettimeofday(__tv:  *mut ::libc::timeval,
        __tz:  *mut ::core::ffi::c_void)
        ->  ::core::ffi::c_int;
        pub fn read(__fd:  ::core::ffi::c_int, __buf:  *mut ::core::ffi::c_void,
        __nbytes:  crate::__stddef_size_t_h::size_t)
        ->  crate::stdlib::ssize_t;
        
        
        pub fn write(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;
        
        
        pub fn pread64(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
            __offset: crate::stdlib::__off64_t,
        ) -> crate::stdlib::ssize_t;
        
        
        pub fn pwrite64(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
            __offset: crate::stdlib::__off64_t,
        ) -> crate::stdlib::ssize_t;
        
        
        pub fn readlink(
            __path: *const ::core::ffi::c_char,
            __buf: *mut ::core::ffi::c_char,
            __len: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;
    }
    pub type FILE =  crate::stdlib::_IO_FILE;
    pub const __S_IFMT:  ::core::ffi::c_int =  0o170000 as ::core::ffi::c_int;
    pub const _SC_ARG_MAX:  crate::stdlib::C2RustUnnamed_1 =  0;
    
    
    pub const _SC_CHILD_MAX: crate::stdlib::C2RustUnnamed_1 = 1;
    
    
    pub const _SC_CLK_TCK: crate::stdlib::C2RustUnnamed_1 = 2;
    
    
    pub const _SC_NGROUPS_MAX: crate::stdlib::C2RustUnnamed_1 = 3;
    
    
    pub const _SC_OPEN_MAX: crate::stdlib::C2RustUnnamed_1 = 4;
    
    
    pub const _SC_STREAM_MAX: crate::stdlib::C2RustUnnamed_1 = 5;
    
    
    pub const _SC_TZNAME_MAX: crate::stdlib::C2RustUnnamed_1 = 6;
    
    
    pub const _SC_JOB_CONTROL: crate::stdlib::C2RustUnnamed_1 = 7;
    
    
    pub const _SC_SAVED_IDS: crate::stdlib::C2RustUnnamed_1 = 8;
    
    
    pub const _SC_REALTIME_SIGNALS: crate::stdlib::C2RustUnnamed_1 = 9;
    
    
    pub const _SC_PRIORITY_SCHEDULING: crate::stdlib::C2RustUnnamed_1 = 10;
    
    
    pub const _SC_TIMERS: crate::stdlib::C2RustUnnamed_1 = 11;
    
    
    pub const _SC_ASYNCHRONOUS_IO: crate::stdlib::C2RustUnnamed_1 = 12;
    
    
    pub const _SC_PRIORITIZED_IO: crate::stdlib::C2RustUnnamed_1 = 13;
    
    
    pub const _SC_SYNCHRONIZED_IO: crate::stdlib::C2RustUnnamed_1 = 14;
    
    
    pub const _SC_FSYNC: crate::stdlib::C2RustUnnamed_1 = 15;
    
    
    pub const _SC_MAPPED_FILES: crate::stdlib::C2RustUnnamed_1 = 16;
    
    
    pub const _SC_MEMLOCK: crate::stdlib::C2RustUnnamed_1 = 17;
    
    
    pub const _SC_MEMLOCK_RANGE: crate::stdlib::C2RustUnnamed_1 = 18;
    
    
    pub const _SC_MEMORY_PROTECTION: crate::stdlib::C2RustUnnamed_1 = 19;
    
    
    pub const _SC_MESSAGE_PASSING: crate::stdlib::C2RustUnnamed_1 = 20;
    
    
    pub const _SC_SEMAPHORES: crate::stdlib::C2RustUnnamed_1 = 21;
    
    
    pub const _SC_SHARED_MEMORY_OBJECTS: crate::stdlib::C2RustUnnamed_1 = 22;
    
    
    pub const _SC_AIO_LISTIO_MAX: crate::stdlib::C2RustUnnamed_1 = 23;
    
    
    pub const _SC_AIO_MAX: crate::stdlib::C2RustUnnamed_1 = 24;
    
    
    pub const _SC_AIO_PRIO_DELTA_MAX: crate::stdlib::C2RustUnnamed_1 = 25;
    
    
    pub const _SC_DELAYTIMER_MAX: crate::stdlib::C2RustUnnamed_1 = 26;
    
    
    pub const _SC_MQ_OPEN_MAX: crate::stdlib::C2RustUnnamed_1 = 27;
    
    
    pub const _SC_MQ_PRIO_MAX: crate::stdlib::C2RustUnnamed_1 = 28;
    
    
    pub const _SC_VERSION: crate::stdlib::C2RustUnnamed_1 = 29;
    
    
    pub const _SC_PAGESIZE: crate::stdlib::C2RustUnnamed_1 = 30;
    
    
    pub const _SC_RTSIG_MAX: crate::stdlib::C2RustUnnamed_1 = 31;
    
    
    pub const _SC_SEM_NSEMS_MAX: crate::stdlib::C2RustUnnamed_1 = 32;
    
    
    pub const _SC_SEM_VALUE_MAX: crate::stdlib::C2RustUnnamed_1 = 33;
    
    
    pub const _SC_SIGQUEUE_MAX: crate::stdlib::C2RustUnnamed_1 = 34;
    
    
    pub const _SC_TIMER_MAX: crate::stdlib::C2RustUnnamed_1 = 35;
    
    
    pub const _SC_BC_BASE_MAX: crate::stdlib::C2RustUnnamed_1 = 36;
    
    
    pub const _SC_BC_DIM_MAX: crate::stdlib::C2RustUnnamed_1 = 37;
    
    
    pub const _SC_BC_SCALE_MAX: crate::stdlib::C2RustUnnamed_1 = 38;
    
    
    pub const _SC_BC_STRING_MAX: crate::stdlib::C2RustUnnamed_1 = 39;
    
    
    pub const _SC_COLL_WEIGHTS_MAX: crate::stdlib::C2RustUnnamed_1 = 40;
    
    
    pub const _SC_EQUIV_CLASS_MAX: crate::stdlib::C2RustUnnamed_1 = 41;
    
    
    pub const _SC_EXPR_NEST_MAX: crate::stdlib::C2RustUnnamed_1 = 42;
    
    
    pub const _SC_LINE_MAX: crate::stdlib::C2RustUnnamed_1 = 43;
    
    
    pub const _SC_RE_DUP_MAX: crate::stdlib::C2RustUnnamed_1 = 44;
    
    
    pub const _SC_CHARCLASS_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 45;
    
    
    pub const _SC_2_VERSION: crate::stdlib::C2RustUnnamed_1 = 46;
    
    
    pub const _SC_2_C_BIND: crate::stdlib::C2RustUnnamed_1 = 47;
    
    
    pub const _SC_2_C_DEV: crate::stdlib::C2RustUnnamed_1 = 48;
    
    
    pub const _SC_2_FORT_DEV: crate::stdlib::C2RustUnnamed_1 = 49;
    
    
    pub const _SC_2_FORT_RUN: crate::stdlib::C2RustUnnamed_1 = 50;
    
    
    pub const _SC_2_SW_DEV: crate::stdlib::C2RustUnnamed_1 = 51;
    
    
    pub const _SC_2_LOCALEDEF: crate::stdlib::C2RustUnnamed_1 = 52;
    
    
    pub const _SC_PII: crate::stdlib::C2RustUnnamed_1 = 53;
    
    
    pub const _SC_PII_XTI: crate::stdlib::C2RustUnnamed_1 = 54;
    
    
    pub const _SC_PII_SOCKET: crate::stdlib::C2RustUnnamed_1 = 55;
    
    
    pub const _SC_PII_INTERNET: crate::stdlib::C2RustUnnamed_1 = 56;
    
    
    pub const _SC_PII_OSI: crate::stdlib::C2RustUnnamed_1 = 57;
    
    
    pub const _SC_POLL: crate::stdlib::C2RustUnnamed_1 = 58;
    
    
    pub const _SC_SELECT: crate::stdlib::C2RustUnnamed_1 = 59;
    
    
    pub const _SC_UIO_MAXIOV: crate::stdlib::C2RustUnnamed_1 = 60;
    
    
    pub const _SC_IOV_MAX: crate::stdlib::C2RustUnnamed_1 = 60;
    
    
    pub const _SC_PII_INTERNET_STREAM: crate::stdlib::C2RustUnnamed_1 = 61;
    
    
    pub const _SC_PII_INTERNET_DGRAM: crate::stdlib::C2RustUnnamed_1 = 62;
    
    
    pub const _SC_PII_OSI_COTS: crate::stdlib::C2RustUnnamed_1 = 63;
    
    
    pub const _SC_PII_OSI_CLTS: crate::stdlib::C2RustUnnamed_1 = 64;
    
    
    pub const _SC_PII_OSI_M: crate::stdlib::C2RustUnnamed_1 = 65;
    
    
    pub const _SC_T_IOV_MAX: crate::stdlib::C2RustUnnamed_1 = 66;
    
    
    pub const _SC_THREADS: crate::stdlib::C2RustUnnamed_1 = 67;
    
    
    pub const _SC_THREAD_SAFE_FUNCTIONS: crate::stdlib::C2RustUnnamed_1 = 68;
    
    
    pub const _SC_GETGR_R_SIZE_MAX: crate::stdlib::C2RustUnnamed_1 = 69;
    
    
    pub const _SC_GETPW_R_SIZE_MAX: crate::stdlib::C2RustUnnamed_1 = 70;
    
    
    pub const _SC_LOGIN_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 71;
    
    
    pub const _SC_TTY_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 72;
    
    
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: crate::stdlib::C2RustUnnamed_1 = 73;
    
    
    pub const _SC_THREAD_KEYS_MAX: crate::stdlib::C2RustUnnamed_1 = 74;
    
    
    pub const _SC_THREAD_STACK_MIN: crate::stdlib::C2RustUnnamed_1 = 75;
    
    
    pub const _SC_THREAD_THREADS_MAX: crate::stdlib::C2RustUnnamed_1 = 76;
    
    
    pub const _SC_THREAD_ATTR_STACKADDR: crate::stdlib::C2RustUnnamed_1 = 77;
    
    
    pub const _SC_THREAD_ATTR_STACKSIZE: crate::stdlib::C2RustUnnamed_1 = 78;
    
    
    pub const _SC_THREAD_PRIORITY_SCHEDULING: crate::stdlib::C2RustUnnamed_1 = 79;
    
    
    pub const _SC_THREAD_PRIO_INHERIT: crate::stdlib::C2RustUnnamed_1 = 80;
    
    
    pub const _SC_THREAD_PRIO_PROTECT: crate::stdlib::C2RustUnnamed_1 = 81;
    
    
    pub const _SC_THREAD_PROCESS_SHARED: crate::stdlib::C2RustUnnamed_1 = 82;
    
    
    pub const _SC_NPROCESSORS_CONF: crate::stdlib::C2RustUnnamed_1 = 83;
    
    
    pub const _SC_NPROCESSORS_ONLN: crate::stdlib::C2RustUnnamed_1 = 84;
    
    
    pub const _SC_PHYS_PAGES: crate::stdlib::C2RustUnnamed_1 = 85;
    
    
    pub const _SC_AVPHYS_PAGES: crate::stdlib::C2RustUnnamed_1 = 86;
    
    
    pub const _SC_ATEXIT_MAX: crate::stdlib::C2RustUnnamed_1 = 87;
    
    
    pub const _SC_PASS_MAX: crate::stdlib::C2RustUnnamed_1 = 88;
    
    
    pub const _SC_XOPEN_VERSION: crate::stdlib::C2RustUnnamed_1 = 89;
    
    
    pub const _SC_XOPEN_XCU_VERSION: crate::stdlib::C2RustUnnamed_1 = 90;
    
    
    pub const _SC_XOPEN_UNIX: crate::stdlib::C2RustUnnamed_1 = 91;
    
    
    pub const _SC_XOPEN_CRYPT: crate::stdlib::C2RustUnnamed_1 = 92;
    
    
    pub const _SC_XOPEN_ENH_I18N: crate::stdlib::C2RustUnnamed_1 = 93;
    
    
    pub const _SC_XOPEN_SHM: crate::stdlib::C2RustUnnamed_1 = 94;
    
    
    pub const _SC_2_CHAR_TERM: crate::stdlib::C2RustUnnamed_1 = 95;
    
    
    pub const _SC_2_C_VERSION: crate::stdlib::C2RustUnnamed_1 = 96;
    
    
    pub const _SC_2_UPE: crate::stdlib::C2RustUnnamed_1 = 97;
    
    
    pub const _SC_XOPEN_XPG2: crate::stdlib::C2RustUnnamed_1 = 98;
    
    
    pub const _SC_XOPEN_XPG3: crate::stdlib::C2RustUnnamed_1 = 99;
    
    
    pub const _SC_XOPEN_XPG4: crate::stdlib::C2RustUnnamed_1 = 100;
    
    
    pub const _SC_CHAR_BIT: crate::stdlib::C2RustUnnamed_1 = 101;
    
    
    pub const _SC_CHAR_MAX: crate::stdlib::C2RustUnnamed_1 = 102;
    
    
    pub const _SC_CHAR_MIN: crate::stdlib::C2RustUnnamed_1 = 103;
    
    
    pub const _SC_INT_MAX: crate::stdlib::C2RustUnnamed_1 = 104;
    
    
    pub const _SC_INT_MIN: crate::stdlib::C2RustUnnamed_1 = 105;
    
    
    pub const _SC_LONG_BIT: crate::stdlib::C2RustUnnamed_1 = 106;
    
    
    pub const _SC_WORD_BIT: crate::stdlib::C2RustUnnamed_1 = 107;
    
    
    pub const _SC_MB_LEN_MAX: crate::stdlib::C2RustUnnamed_1 = 108;
    
    
    pub const _SC_NZERO: crate::stdlib::C2RustUnnamed_1 = 109;
    
    
    pub const _SC_SSIZE_MAX: crate::stdlib::C2RustUnnamed_1 = 110;
    
    
    pub const _SC_SCHAR_MAX: crate::stdlib::C2RustUnnamed_1 = 111;
    
    
    pub const _SC_SCHAR_MIN: crate::stdlib::C2RustUnnamed_1 = 112;
    
    
    pub const _SC_SHRT_MAX: crate::stdlib::C2RustUnnamed_1 = 113;
    
    
    pub const _SC_SHRT_MIN: crate::stdlib::C2RustUnnamed_1 = 114;
    
    
    pub const _SC_UCHAR_MAX: crate::stdlib::C2RustUnnamed_1 = 115;
    
    
    pub const _SC_UINT_MAX: crate::stdlib::C2RustUnnamed_1 = 116;
    
    
    pub const _SC_ULONG_MAX: crate::stdlib::C2RustUnnamed_1 = 117;
    
    
    pub const _SC_USHRT_MAX: crate::stdlib::C2RustUnnamed_1 = 118;
    
    
    pub const _SC_NL_ARGMAX: crate::stdlib::C2RustUnnamed_1 = 119;
    
    
    pub const _SC_NL_LANGMAX: crate::stdlib::C2RustUnnamed_1 = 120;
    
    
    pub const _SC_NL_MSGMAX: crate::stdlib::C2RustUnnamed_1 = 121;
    
    
    pub const _SC_NL_NMAX: crate::stdlib::C2RustUnnamed_1 = 122;
    
    
    pub const _SC_NL_SETMAX: crate::stdlib::C2RustUnnamed_1 = 123;
    
    
    pub const _SC_NL_TEXTMAX: crate::stdlib::C2RustUnnamed_1 = 124;
    
    
    pub const _SC_XBS5_ILP32_OFF32: crate::stdlib::C2RustUnnamed_1 = 125;
    
    
    pub const _SC_XBS5_ILP32_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 126;
    
    
    pub const _SC_XBS5_LP64_OFF64: crate::stdlib::C2RustUnnamed_1 = 127;
    
    
    pub const _SC_XBS5_LPBIG_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 128;
    
    
    pub const _SC_XOPEN_LEGACY: crate::stdlib::C2RustUnnamed_1 = 129;
    
    
    pub const _SC_XOPEN_REALTIME: crate::stdlib::C2RustUnnamed_1 = 130;
    
    
    pub const _SC_XOPEN_REALTIME_THREADS: crate::stdlib::C2RustUnnamed_1 = 131;
    
    
    pub const _SC_ADVISORY_INFO: crate::stdlib::C2RustUnnamed_1 = 132;
    
    
    pub const _SC_BARRIERS: crate::stdlib::C2RustUnnamed_1 = 133;
    
    
    pub const _SC_BASE: crate::stdlib::C2RustUnnamed_1 = 134;
    
    
    pub const _SC_C_LANG_SUPPORT: crate::stdlib::C2RustUnnamed_1 = 135;
    
    
    pub const _SC_C_LANG_SUPPORT_R: crate::stdlib::C2RustUnnamed_1 = 136;
    
    
    pub const _SC_CLOCK_SELECTION: crate::stdlib::C2RustUnnamed_1 = 137;
    
    
    pub const _SC_CPUTIME: crate::stdlib::C2RustUnnamed_1 = 138;
    
    
    pub const _SC_THREAD_CPUTIME: crate::stdlib::C2RustUnnamed_1 = 139;
    
    
    pub const _SC_DEVICE_IO: crate::stdlib::C2RustUnnamed_1 = 140;
    
    
    pub const _SC_DEVICE_SPECIFIC: crate::stdlib::C2RustUnnamed_1 = 141;
    
    
    pub const _SC_DEVICE_SPECIFIC_R: crate::stdlib::C2RustUnnamed_1 = 142;
    
    
    pub const _SC_FD_MGMT: crate::stdlib::C2RustUnnamed_1 = 143;
    
    
    pub const _SC_FIFO: crate::stdlib::C2RustUnnamed_1 = 144;
    
    
    pub const _SC_PIPE: crate::stdlib::C2RustUnnamed_1 = 145;
    
    
    pub const _SC_FILE_ATTRIBUTES: crate::stdlib::C2RustUnnamed_1 = 146;
    
    
    pub const _SC_FILE_LOCKING: crate::stdlib::C2RustUnnamed_1 = 147;
    
    
    pub const _SC_FILE_SYSTEM: crate::stdlib::C2RustUnnamed_1 = 148;
    
    
    pub const _SC_MONOTONIC_CLOCK: crate::stdlib::C2RustUnnamed_1 = 149;
    
    
    pub const _SC_MULTI_PROCESS: crate::stdlib::C2RustUnnamed_1 = 150;
    
    
    pub const _SC_SINGLE_PROCESS: crate::stdlib::C2RustUnnamed_1 = 151;
    
    
    pub const _SC_NETWORKING: crate::stdlib::C2RustUnnamed_1 = 152;
    
    
    pub const _SC_READER_WRITER_LOCKS: crate::stdlib::C2RustUnnamed_1 = 153;
    
    
    pub const _SC_SPIN_LOCKS: crate::stdlib::C2RustUnnamed_1 = 154;
    
    
    pub const _SC_REGEXP: crate::stdlib::C2RustUnnamed_1 = 155;
    
    
    pub const _SC_REGEX_VERSION: crate::stdlib::C2RustUnnamed_1 = 156;
    
    
    pub const _SC_SHELL: crate::stdlib::C2RustUnnamed_1 = 157;
    
    
    pub const _SC_SIGNALS: crate::stdlib::C2RustUnnamed_1 = 158;
    
    
    pub const _SC_SPAWN: crate::stdlib::C2RustUnnamed_1 = 159;
    
    
    pub const _SC_SPORADIC_SERVER: crate::stdlib::C2RustUnnamed_1 = 160;
    
    
    pub const _SC_THREAD_SPORADIC_SERVER: crate::stdlib::C2RustUnnamed_1 = 161;
    
    
    pub const _SC_SYSTEM_DATABASE: crate::stdlib::C2RustUnnamed_1 = 162;
    
    
    pub const _SC_SYSTEM_DATABASE_R: crate::stdlib::C2RustUnnamed_1 = 163;
    
    
    pub const _SC_TIMEOUTS: crate::stdlib::C2RustUnnamed_1 = 164;
    
    
    pub const _SC_TYPED_MEMORY_OBJECTS: crate::stdlib::C2RustUnnamed_1 = 165;
    
    
    pub const _SC_USER_GROUPS: crate::stdlib::C2RustUnnamed_1 = 166;
    
    
    pub const _SC_USER_GROUPS_R: crate::stdlib::C2RustUnnamed_1 = 167;
    
    
    pub const _SC_2_PBS: crate::stdlib::C2RustUnnamed_1 = 168;
    
    
    pub const _SC_2_PBS_ACCOUNTING: crate::stdlib::C2RustUnnamed_1 = 169;
    
    
    pub const _SC_2_PBS_LOCATE: crate::stdlib::C2RustUnnamed_1 = 170;
    
    
    pub const _SC_2_PBS_MESSAGE: crate::stdlib::C2RustUnnamed_1 = 171;
    
    
    pub const _SC_2_PBS_TRACK: crate::stdlib::C2RustUnnamed_1 = 172;
    
    
    pub const _SC_SYMLOOP_MAX: crate::stdlib::C2RustUnnamed_1 = 173;
    
    
    pub const _SC_STREAMS: crate::stdlib::C2RustUnnamed_1 = 174;
    
    
    pub const _SC_2_PBS_CHECKPOINT: crate::stdlib::C2RustUnnamed_1 = 175;
    
    
    pub const _SC_V6_ILP32_OFF32: crate::stdlib::C2RustUnnamed_1 = 176;
    
    
    pub const _SC_V6_ILP32_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 177;
    
    
    pub const _SC_V6_LP64_OFF64: crate::stdlib::C2RustUnnamed_1 = 178;
    
    
    pub const _SC_V6_LPBIG_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 179;
    
    
    pub const _SC_HOST_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 180;
    
    
    pub const _SC_TRACE: crate::stdlib::C2RustUnnamed_1 = 181;
    
    
    pub const _SC_TRACE_EVENT_FILTER: crate::stdlib::C2RustUnnamed_1 = 182;
    
    
    pub const _SC_TRACE_INHERIT: crate::stdlib::C2RustUnnamed_1 = 183;
    
    
    pub const _SC_TRACE_LOG: crate::stdlib::C2RustUnnamed_1 = 184;
    
    
    pub const _SC_LEVEL1_ICACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 185;
    
    
    pub const _SC_LEVEL1_ICACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 186;
    
    
    pub const _SC_LEVEL1_ICACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 187;
    
    
    pub const _SC_LEVEL1_DCACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 188;
    
    
    pub const _SC_LEVEL1_DCACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 189;
    
    
    pub const _SC_LEVEL1_DCACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 190;
    
    
    pub const _SC_LEVEL2_CACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 191;
    
    
    pub const _SC_LEVEL2_CACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 192;
    
    
    pub const _SC_LEVEL2_CACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 193;
    
    
    pub const _SC_LEVEL3_CACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 194;
    
    
    pub const _SC_LEVEL3_CACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 195;
    
    
    pub const _SC_LEVEL3_CACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 196;
    
    
    pub const _SC_LEVEL4_CACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 197;
    
    
    pub const _SC_LEVEL4_CACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 198;
    
    
    pub const _SC_LEVEL4_CACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 199;
    
    
    pub const _SC_IPV6: crate::stdlib::C2RustUnnamed_1 = 235;
    
    
    pub const _SC_RAW_SOCKETS: crate::stdlib::C2RustUnnamed_1 = 236;
    
    
    pub const _SC_V7_ILP32_OFF32: crate::stdlib::C2RustUnnamed_1 = 237;
    
    
    pub const _SC_V7_ILP32_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 238;
    
    
    pub const _SC_V7_LP64_OFF64: crate::stdlib::C2RustUnnamed_1 = 239;
    
    
    pub const _SC_V7_LPBIG_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 240;
    
    
    pub const _SC_SS_REPL_MAX: crate::stdlib::C2RustUnnamed_1 = 241;
    
    
    pub const _SC_TRACE_EVENT_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 242;
    
    
    pub const _SC_TRACE_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 243;
    
    
    pub const _SC_TRACE_SYS_MAX: crate::stdlib::C2RustUnnamed_1 = 244;
    
    
    pub const _SC_TRACE_USER_EVENT_MAX: crate::stdlib::C2RustUnnamed_1 = 245;
    
    
    pub const _SC_XOPEN_STREAMS: crate::stdlib::C2RustUnnamed_1 = 246;
    
    
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: crate::stdlib::C2RustUnnamed_1 = 247;
    
    
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: crate::stdlib::C2RustUnnamed_1 = 248;
    
    
    pub const _SC_MINSIGSTKSZ: crate::stdlib::C2RustUnnamed_1 = 249;
    
    
    pub const _SC_SIGSTKSZ: crate::stdlib::C2RustUnnamed_1 = 250;
    pub type C2RustUnnamed_0_1 =  ::core::ffi::c_uint;
    
    
    pub const _ISupper: crate::stdlib::C2RustUnnamed_0_1 = 256;
    
    
    pub const _ISlower: crate::stdlib::C2RustUnnamed_0_1 = 512;
    
    
    pub const _ISalpha: crate::stdlib::C2RustUnnamed_0_1 = 1024;
    
    
    pub const _ISdigit: crate::stdlib::C2RustUnnamed_0_1 = 2048;
    
    
    pub const _ISxdigit: crate::stdlib::C2RustUnnamed_0_1 = 4096;
    
    
    pub const _ISspace: crate::stdlib::C2RustUnnamed_0_1 = 8192;
    
    
    pub const _ISprint: crate::stdlib::C2RustUnnamed_0_1 = 16384;
    
    
    pub const _ISgraph: crate::stdlib::C2RustUnnamed_0_1 = 32768;
    
    
    pub const _ISblank: crate::stdlib::C2RustUnnamed_0_1 = 1;
    
    
    pub const _IScntrl: crate::stdlib::C2RustUnnamed_0_1 = 2;
    
    
    pub const _ISpunct: crate::stdlib::C2RustUnnamed_0_1 = 4;
    
    
    pub const _ISalnum: crate::stdlib::C2RustUnnamed_0_1 = 8;
    pub const __O_LARGEFILE:  ::core::ffi::c_int =  0 as ::core::ffi::c_int;
    
    
    pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    
    pub const F_SETLK64: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    pub const __O_NOFOLLOW:  ::core::ffi::c_int =
         0o400000 as ::core::ffi::c_int;
    
    
    pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
    pub const M_PI:  ::core::ffi::c_double =  3.14159265358979323846f64;
    pub type C2RustUnnamed_1 =  ::core::ffi::c_uint;
    
    
    pub const PTHREAD_MUTEX_TIMED_NP: crate::stdlib::C2RustUnnamed_1 = 0;
    
    
    pub const PTHREAD_MUTEX_RECURSIVE_NP: crate::stdlib::C2RustUnnamed_1 = 1;
    
    
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: crate::stdlib::C2RustUnnamed_1 = 2;
    
    
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: crate::stdlib::C2RustUnnamed_1 = 3;
    
    
    pub const PTHREAD_MUTEX_NORMAL: crate::stdlib::C2RustUnnamed_1 = 0;
    
    
    pub const PTHREAD_MUTEX_RECURSIVE: crate::stdlib::C2RustUnnamed_1 = 1;
    
    
    pub const PTHREAD_MUTEX_ERRORCHECK: crate::stdlib::C2RustUnnamed_1 = 2;
    
    
    pub const PTHREAD_MUTEX_DEFAULT: crate::stdlib::C2RustUnnamed_1 = 0;
    
    
    pub const PTHREAD_MUTEX_FAST_NP: crate::stdlib::C2RustUnnamed_1 = 0;
    pub type pthread_t =  ::core::ffi::c_ulong;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union pthread_mutexattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union pthread_attr_t {
        pub __size: [::core::ffi::c_char; 56],
        pub __align: ::core::ffi::c_long,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union pthread_mutex_t {
        pub __data: crate::stdlib::__pthread_mutex_s,
        pub __size: [::core::ffi::c_char; 40],
        pub __align: ::core::ffi::c_long,
    }
    pub type intptr_t =  isize;
    
    
    pub type uintptr_t = usize;
    pub type int8_t =  crate::stdlib::__int8_t;
    
    
    pub type int16_t = crate::stdlib::__int16_t;
    pub type uint8_t =  crate::stdlib::__uint8_t;
    
    
    pub type uint16_t = crate::stdlib::__uint16_t;
    
    
    pub type uint32_t = crate::stdlib::__uint32_t;
    pub type va_list =  crate::__stdarg___gnuc_va_list_h::__gnuc_va_list;
    
    
    pub type off_t = crate::stdlib::__off64_t;
    
    
    pub type off64_t = crate::stdlib::__off64_t;
    
    
    pub type ssize_t = crate::stdlib::__ssize_t;
    pub const FILENAME_MAX:  ::core::ffi::c_int =  4096 as ::core::ffi::c_int;
    pub type __compar_fn_t =
         Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;
    pub type _IO_lock_t =  ();
    
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut crate::stdlib::_IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused2: [::core::ffi::c_char; 20],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_mutex_s {
        pub __lock:  ::core::ffi::c_int,
        pub __count:  ::core::ffi::c_uint,
        pub __owner:  ::core::ffi::c_int,
        pub __nusers:  ::core::ffi::c_uint,
        pub __kind:  ::core::ffi::c_int,
        pub __spins:  ::core::ffi::c_short,
        pub __elision:  ::core::ffi::c_short,
        pub __list:  crate::stdlib::__pthread_list_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev:  crate::stdlib::__dev_t,
        pub st_ino:  crate::stdlib::__ino_t,
        pub st_nlink:  crate::stdlib::__nlink_t,
        pub st_mode:  crate::stdlib::__mode_t,
        pub st_uid:  crate::stdlib::__uid_t,
        pub st_gid:  crate::stdlib::__gid_t,
        pub __pad0:  ::core::ffi::c_int,
        pub st_rdev:  crate::stdlib::__dev_t,
        pub st_size:  crate::stdlib::__off_t,
        pub st_blksize:  crate::stdlib::__blksize_t,
        pub st_blocks:  crate::stdlib::__blkcnt_t,
        pub st_atim:  ::libc::timespec,
        pub st_mtim:  ::libc::timespec,
        pub st_ctim:  ::libc::timespec,
        pub __glibc_reserved:  [crate::stdlib::__syscall_slong_t; 3],
    }
    pub type dev_t =  crate::stdlib::__dev_t;
    
    
    pub type gid_t = crate::stdlib::__gid_t;
    
    
    pub type mode_t = crate::stdlib::__mode_t;
    
    
    pub type uid_t = crate::stdlib::__uid_t;
    
    
    pub type pid_t = crate::stdlib::__pid_t;

    #[cfg(feature = "test")]
    pub type Tcl_WideInt =  ::core::ffi::c_longlong;
    
    #[cfg(feature = "test")]
    pub type Tcl_Size = crate::__stddef_ptrdiff_t_h::ptrdiff_t;
    
    #[cfg(feature = "test")]    
    pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj, *mut crate::stdlib::Tcl_Obj) -> ();
    
    #[cfg(feature = "test")]
    
    pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj) -> ();
    
    #[cfg(feature = "test")]    
    pub type Tcl_SetFromAnyProc =
        unsafe extern "C" fn(*mut crate::stdlib::Tcl_Interp, *mut crate::stdlib::Tcl_Obj) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")]
    
    pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj) -> ();
    
    #[cfg(feature = "test")]    
    pub type Tcl_ObjTypeLengthProc = unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj) -> crate::stdlib::Tcl_Size;
    
    #[cfg(feature = "test")]    
    pub type Tcl_ObjTypeIndexProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")]    
    pub type Tcl_ObjTypeSliceProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        crate::stdlib::Tcl_Size,
        *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")]    
    pub type Tcl_ObjTypeReverseProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")]    
    pub type Tcl_ObjTypeGetElements = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        *mut crate::stdlib::Tcl_Size,
        *mut *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")] 
    pub type Tcl_ObjTypeSetElement = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        *const *mut crate::stdlib::Tcl_Obj,
        *mut crate::stdlib::Tcl_Obj,
    ) -> *mut crate::stdlib::Tcl_Obj;
    
    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeReplaceProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        crate::stdlib::Tcl_Size,
        crate::stdlib::Tcl_Size,
        *const *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeInOperatorProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        *mut crate::stdlib::Tcl_Obj,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    
    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Tcl_ObjType {
        pub name: *const ::core::ffi::c_char,
        pub freeIntRepProc: Option<crate::stdlib::Tcl_FreeInternalRepProc>,
        pub dupIntRepProc: Option<crate::stdlib::Tcl_DupInternalRepProc>,
        pub updateStringProc: Option<crate::stdlib::Tcl_UpdateStringProc>,
        pub setFromAnyProc: Option<crate::stdlib::Tcl_SetFromAnyProc>,
        pub version: crate::__stddef_size_t_h::size_t,
        pub lengthProc: Option<crate::stdlib::Tcl_ObjTypeLengthProc>,
        pub indexProc: Option<crate::stdlib::Tcl_ObjTypeIndexProc>,
        pub sliceProc: Option<crate::stdlib::Tcl_ObjTypeSliceProc>,
        pub reverseProc: Option<crate::stdlib::Tcl_ObjTypeReverseProc>,
        pub getElementsProc: Option<crate::stdlib::Tcl_ObjTypeGetElements>,
        pub setElementProc: Option<crate::stdlib::Tcl_ObjTypeSetElement>,
        pub replaceProc: Option<crate::stdlib::Tcl_ObjTypeReplaceProc>,
        pub inOperProc: Option<crate::stdlib::Tcl_ObjTypeInOperatorProc>,
    }
    #[cfg(feature = "test")]    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union Tcl_ObjInternalRep {
        pub longValue: ::core::ffi::c_long,
        pub doubleValue: ::core::ffi::c_double,
        pub otherValuePtr: *mut ::core::ffi::c_void,
        pub wideValue: crate::stdlib::Tcl_WideInt,
        pub twoPtrValue: crate::stdlib::C2RustUnnamed_0,
        pub ptrAndLongRep: crate::stdlib::C2RustUnnamed,
    }
    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Tcl_Obj {
        pub refCount: crate::stdlib::Tcl_Size,
        pub bytes: *mut ::core::ffi::c_char,
        pub length: crate::stdlib::Tcl_Size,
        pub typePtr: *const crate::stdlib::Tcl_ObjType,
        pub internalRep: crate::stdlib::Tcl_ObjInternalRep,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_0 {
        pub ptr1: *mut ::core::ffi::c_void,
        pub ptr2: *mut ::core::ffi::c_void,
    }
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed {
        pub ptr: *mut ::core::ffi::c_void,
        pub value: ::core::ffi::c_ulong,
    }
    

    pub type __pthread_list_t =  crate::stdlib::__pthread_internal_list;
    
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct __pthread_internal_list {
        pub __prev: *mut crate::stdlib::__pthread_internal_list,
        pub __next: *mut crate::stdlib::__pthread_internal_list,
    }
    pub type time_t =  crate::stdlib::__time_t;
    pub type __int8_t =  i8;
    
    
    pub type __uint8_t = u8;
    
    
    pub type __int16_t = i16;
    
    
    pub type __uint16_t = u16;
    
    
    pub type __uint32_t = u32;
    
    
    pub type __dev_t = ::core::ffi::c_ulong;
    
    
    pub type __uid_t = ::core::ffi::c_uint;
    
    
    pub type __gid_t = ::core::ffi::c_uint;
    
    
    pub type __ino_t = ::core::ffi::c_ulong;
    
    
    pub type __mode_t = ::core::ffi::c_uint;
    
    
    pub type __nlink_t = ::core::ffi::c_ulong;
    
    
    pub type __off_t = ::core::ffi::c_long;
    
    
    pub type __off64_t = ::core::ffi::c_long;
    
    
    pub type __pid_t = ::core::ffi::c_int;
    
    
    pub type __time_t = ::core::ffi::c_long;
    
    
    pub type __suseconds_t = ::core::ffi::c_long;
    
    
    pub type __blksize_t = ::core::ffi::c_long;
    
    
    pub type __blkcnt_t = ::core::ffi::c_long;
    
    
    pub type __ssize_t = ::core::ffi::c_long;
    
    
    pub type __syscall_slong_t = ::core::ffi::c_long;
}#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod ctime;
    pub mod ext {
        pub mod fts3 {
            pub mod fts3;
            pub mod fts3_aux;
            pub mod fts3_expr;
            pub mod fts3_hash;
            pub mod fts3_icu;
            pub mod fts3_porter;
            pub mod fts3_snippet;
            pub mod fts3_tokenize_vtab;
            pub mod fts3_tokenizer;
            pub mod fts3_tokenizer1;
            pub mod fts3_unicode;
            pub mod fts3_unicode2;
            pub mod fts3_write;
        } // mod fts3
        pub mod icu {
            pub mod icu;
        } // mod icu
        pub mod misc {
            pub mod stmt;
        } // mod misc
        pub mod rbu {
            pub mod sqlite3rbu;
        } // mod rbu
        pub mod rtree {
            pub mod rtree;
        } // mod rtree
        pub mod session {
            pub mod sqlite3session;
        } // mod session
    } // mod ext
    pub mod fts5;
    pub mod opcodes;
    pub mod parse;
    pub mod src {
        pub mod alter;
        pub mod analyze;
        pub mod attach;
        pub mod auth;
        pub mod backup;
        pub mod bitvec;
        pub mod btmutex;
        pub mod btree;
        pub mod build;
        pub mod callback;
        pub mod carray;
        pub mod complete;
        pub mod date;
        pub mod dbpage;
        pub mod dbstat;
        pub mod delete;
        pub mod expr;
        pub mod fault;
        pub mod fkey;
        pub mod func;
        pub mod global;
        pub mod hash;
        pub mod insert;
        pub mod json;
        pub mod legacy;
        pub mod loadext;
        pub mod main;
        pub mod malloc;
        pub mod mem0;
        pub mod mem1;
        pub mod mem2;
        pub mod mem3;
        pub mod mem5;
        pub mod memdb;
        pub mod memjournal;
        pub mod mutex;
        pub mod mutex_noop;
        pub mod mutex_unix;
        pub mod mutex_w32;
        pub mod notify;
        pub mod os;
        pub mod os_kv;
        pub mod os_unix;
        pub mod os_win;
        pub mod pager;
        pub mod pcache;
        pub mod pcache1;
        pub mod pragma;
        pub mod prepare;
        pub mod printf;
        pub mod random;
        pub mod resolve;
        pub mod rowset;
        pub mod select;
        pub mod status;
        pub mod table;
        pub mod threads;
        pub mod tokenize;
        pub mod treeview;
        pub mod trigger;
        pub mod update;
        pub mod upsert;
        pub mod utf;
        pub mod util;
        pub mod vacuum;
        pub mod vdbe;
        pub mod vdbeapi;
        pub mod vdbeaux;
        pub mod vdbeblob;
        pub mod vdbemem;
        pub mod vdbesort;
        pub mod vdbetrace;
        pub mod vdbevtab;
        pub mod vtab;
        pub mod wal;
        pub mod walker;
        pub mod r#where;
        pub mod wherecode;
        pub mod whereexpr;
        pub mod window;
    } // mod src
} // mod src
