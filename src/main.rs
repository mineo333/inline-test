use inline_c::assert_cxx;
use std::env;
use std::fs;
use libloading;
use core::ffi::c_void;


unsafe fn test_harness() {
    let compiled_lib = assert_cxx!{
        #inline_c_rs CFLAGS: "-shared"
        #include <stdint.h>
        #include <stdlib.h>
        #include <string>

        extern "C" int heap_uaf_read() {
            int *array = new int[100];
            delete[] array;
            fprintf(stdout, "%d\n", array[5]);
            return 0;
        }

        extern "C" int heap_uaf_write() {
            int *array = new int[100];
            delete[] array;
            array[5] = 1;
            return 0;
        }

        extern "C" int heap_oob_read() {
            int *array = new int[100];
            fprintf(stdout, "%d\n", array[100]);
            delete[] array;
            return 0;
        }

        extern "C" int heap_oob_write() {
            int *array = new int[100];
            array[100] = 1;
            delete[] array;
             return 0;
        }
        extern "C" int malloc_heap_uaf_read() {
            int *array = static_cast<int *>(malloc(100 * sizeof(int)));
            free(array);
            fprintf(stdout, "%d\n", array[5]);
            return 0;
        }

        extern "C" int malloc_heap_uaf_write() {
            int *array = static_cast<int *>(malloc(100 * sizeof(int)));
            free(array);
            array[5] = 1;
            return 0;
        }

        extern "C" int malloc_heap_oob_read() {
            int *array = static_cast<int *>(malloc(100 * sizeof(int)));
            fprintf(stdout, "%d\n", array[100]);
            free(array);
            return 0;
        }

        extern "C" int malloc_heap_oob_write() {
            int *array = static_cast<int *>(malloc(100 * sizeof(int)));
            array[100] = 1;
            free(array);
            return 0;
        }

        extern "C" int LLVMFuzzerTestOneInput() {
            // abort();
            return 0;
        }
    };

    println!("{:?}", compiled_lib.output_path());

    loop {

    }
    let lib = libloading::Library::new(compiled_lib.output_path()).expect("Failed to load library");
    let symbol: libloading::Symbol<unsafe extern fn() -> c_void> = lib.get(b"uaf").expect("Failed to find uaf");
    //println!("{:#x}", symbol.into_raw().into_raw() as u64);
    
    symbol();

    
}



fn main() {

    unsafe { test_harness() };    
    
}
