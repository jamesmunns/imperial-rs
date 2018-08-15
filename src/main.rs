extern crate libc;

// todo: only in no std mode
#[allow(unused_imports)]
use std as core;

mod bindings;

use bindings::*;

fn main() {

    let wflg: i32 = 70; /* set defaults */
    let sflg: i32 = 5;
    let dflg: i32 = 2000;
    #[allow(non_snake_case)]
    let Sflg: i32 = 10;

    let filename = ::std::ffi::CString::new("empsave.dat").unwrap();

    unsafe {
        // leaking filename
        savefile = filename.into_raw();
    }

    unsafe {
        SMOOTH = sflg;
        WATER_RATIO = wflg;
        delay_time = dflg;
        save_interval = Sflg;

        /* compute min distance between cities */
        let mut land = MAP_SIZE * ((100i32 - WATER_RATIO) as u32)/ 100; /* available land */
        land /= NUM_CITY; /* land per city */
        MIN_CITY_DIST = isqrt (land as i32); /* distance between cities */

        empire (); /* call main routine */
    }
    return;
}
