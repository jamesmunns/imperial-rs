extern crate gcc;

use gcc::Build;

use std::env;
use std::path::PathBuf;

// todo, globbing?
const FILES: &[&str] = &[
    "attack.c",
    "compmove.c",
    "data.c",
    "display.c",
    "edit.c",
    "empire.c",
    "game.c",
    // "main.c",
    "map.c",
    "math.c",
    "object.c",
    "term.c",
    "usermove.c",
    "util.c",
];

fn main() {
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut config = Build::new();

    config.out_dir(outdir.clone());
    config.include("./vms-empire");
    for file in FILES {
        config.file(&format!("./vms-empire/{}", file));
    }
    config.flag("-Wall");
    config.flag("-Wno-format-security");

    config.compile("libempire.a");

    println!("cargo:rustc-link-search={}", outdir.display());
    println!("cargo:rustc-link-lib=static=empire");
    // println!("cargo:rustc-link-search={}", "/usr/lib");
    println!("cargo:rustc-link-lib=dylib=ncurses");


}