use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use mx_chain_simulator_interface_config_rs::CONFIG_PATH;

const CHAIN_SIMULATOR_NAME: &str = "chainsimulator";

#[allow(dead_code)]
const CHAIN_SIMULATOR_DARWIN_AMD64_NAME: &str = "chainsimulator_darwin_amd64";
#[allow(dead_code)]
const CHAIN_SIMULATOR_LINUX_AMD64_NAME: &str = "chainsimulator_linux_amd64";
#[allow(dead_code)]
const LIBWASMER_LINUX_AMD64_NAME: &str = "libwasmer_linux_amd64.so";
#[allow(dead_code)]
const LIBWASMER_DARWIN_AMD64_NAME: &str = "libwasmer_darwin_amd64.dylib";

fn main() {
    let out_dir_env = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env).to_path_buf();

    download_assets(&out_dir);
    copy_dir_recursive(
        &Path::new(CONFIG_PATH),
        &out_dir.join("assets/config")
    ).unwrap();

    let dest_path = Path::new(&out_dir).join("generated_code.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Write the generated Rust code to the file.
    let generated_code = format!(
        r#"
        pub const ASSETS_PATH: &str = "{}/assets";
        "#,
        out_dir.to_str().unwrap()
    );

    f.write_all(generated_code.as_bytes()).unwrap()
}

fn download_assets(out_dir: &Path) {
    let version= std::env::var("CARGO_PKG_VERSION").unwrap();

    let client = reqwest::blocking::Client::new();

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        let (chain_simulator_bin_name, libwasmer_name) = (
            CHAIN_SIMULATOR_DARWIN_AMD64_NAME,
            LIBWASMER_LINUX_AMD64_NAME
        );
    #[cfg(all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64")))]
        let (chain_simulator_bin_name, libwasmer_name) = (
            CHAIN_SIMULATOR_DARWIN_AMD64_NAME,
            LIBWASMER_DARWIN_AMD64_NAME
        );
    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64"))
    )))]
        compile_error!("Unsupported arch!");

    let files_to_download = vec![
        (format!("assets/{chain_simulator_bin_name}"), format!("assets/{CHAIN_SIMULATOR_NAME}")),
        (format!("assets/{libwasmer_name}"), format!("assets/{libwasmer_name}"))
    ];


    for (source_path, dest_path) in files_to_download {
        let content = client
            .get(
                format!("https://github.com/gfusee/mx-chain-simulator-interface-rs/raw/{version}/{source_path}")
            )
            .send()
            .unwrap()
            .bytes()
            .unwrap();

        write_bytes_to_file(
            Path::new(&out_dir.join(dest_path)),
            &content
        )
            .unwrap()
    }


}

fn write_bytes_to_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    // Ensure the directory structure exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Open the file, creating it if necessary
    let mut file = File::create(path)?;

    // Write bytes to the file
    file.write_all(bytes)?;

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_file() {
            fs::copy(&src_path, &dst_path)?;
        } else if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        }
    }

    Ok(())
}