/*
 * Shim for generating bindgen bindings. To regenerate, run:
 *
 * bindgen ./bindings.h --ctypes-prefix="::libc" -o src/bindings.rs
 *
 * You will have to manually add the following lint-suppressors:
 *
 *     #![allow(non_upper_case_globals)]
 *     #![allow(non_camel_case_types)]
 *     #![allow(non_snake_case)]
 *     #![allow(dead_code)]
 */


#include "./vms-empire/empire.h"
#include "./vms-empire/extern.h"
