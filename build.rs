extern crate serde_codegen;

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    match fs::create_dir(Path::new(&out_dir).join("sc")) {
        Ok(()) => (),
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => (),
        Err(e) => panic!("{}", e)
    }

    for &(src, dst) in [("src/schema.rs.in", "schema.rs")].into_iter() {
        let src = Path::new(src);
        let dst = Path::new(&out_dir).join(dst);
        serde_codegen::expand(&src, &dst).unwrap()
    }
}