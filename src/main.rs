use inline_c::assert_c;
use std::env;
use std::fs;
use libloading;
use core::ffi::c_void;


unsafe fn test_stdout() {
    let compiled_lib = assert_c! {
        #inline_c_rs CFLAGS: "-shared"
        #include <stdio.h>
        #include <stdlib.h>

        void uaf() {
            printf("Hello World\n");
            int* buf = malloc(32);
            free(buf);
            *buf = 0xdeadbeef;
            
        }
    };

    println!("{:?}", compiled_lib.output_path());
    let lib = libloading::Library::new(compiled_lib.output_path()).expect("Failed to load library");
    let symbol: libloading::Symbol<unsafe extern fn() -> c_void> = lib.get(b"uaf").expect("Failed to find uaf");
    //println!("{:#x}", symbol.into_raw().into_raw() as u64);
    
    symbol();

    
}



fn main() {

    unsafe { test_stdout() };    
    
}
