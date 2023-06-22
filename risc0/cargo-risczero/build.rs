use std::env;

#[cfg(feature = "experimental")]
mod runtime {
    use std::{env, fs, io, path::Path};

    use risc0_build::setup_guest_build_env;
    use zip::{write::FileOptions, CompressionMethod, ZipWriter};

    pub fn build_and_zip_runtime() {
        // Build the rust-runtime.a file and place it in a zip archive for inclusion in
        // the cargo-risczero binary.
        let out_dir_env = env::var_os("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir_env); // $ROOT/target/$profile/build/$crate/out
        let guest_build_env = setup_guest_build_env(out_dir);

        let rust_runtime = guest_build_env.build_rust_runtime();
        let f = fs::File::create(out_dir.join("cargo-risczero.zip")).unwrap();
        let mut zip = ZipWriter::new(f);
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);

        zip.start_file("rust-runtime.a", options).unwrap();
        let mut runtime_in = fs::File::open(rust_runtime).unwrap();
        io::copy(&mut runtime_in, &mut zip).unwrap();
        zip.finish().unwrap();
    }
}

fn main() {
    env_logger::init();

    if env::var("CARGO_CFG_TARGET_OS").unwrap().contains("zkvm") {
        // Guest shouldn't recursively depend on itself.
        return;
    }

    println!("V12!");

    #[cfg(feature = "experimental")]
    {
        runtime::build_and_zip_runtime();
    }
}
