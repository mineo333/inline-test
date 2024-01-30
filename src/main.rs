use libafl_inline_c::assert_cxx;
use std::env;
use std::fs;
use libloading;
use core::ffi::c_void;


macro_rules! compile_and_load{
    ($fuzz_code:expr) => {
        let compiled_lib = $fuzz_code;
        println!("{:?}", compiled_lib.output_path());
        
    }
}

unsafe fn test_harness() {
    compile_and_load!(assert_cxx!{
        #inline_c_rs SHARED
        #inline_c_rs TARGET: "x86_64-pc-windows-gnu"
        #include <stdint.h>
        #include <stdlib.h>
        #include <string>

        extern "C" int heap_uaf_read() {
            int *array = static_cast<int *>(malloc(100 * sizeof(int)));
            array[100] = 1;
            free(array);
            return 0;
        }
    });

   // println!("{:?}", compiled_lib.output_path());


    
}



fn main() {

    unsafe { test_harness() };    
    
}
