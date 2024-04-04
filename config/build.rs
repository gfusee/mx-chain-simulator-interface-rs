use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir_env = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env).to_path_buf();

    let path = Path::new("assets/config");

    copy_dir_recursive(
        &path,
        &out_dir.join(path)
    ).unwrap();

    let dest_path = Path::new(&out_dir).join("generated_code.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Write the generated Rust code to the file.
    let generated_code = format!(
        r#"
        pub const CONFIG_PATH: &str = "{}/assets/config";
        "#,
        out_dir.to_str().unwrap()
    );

    f.write_all(generated_code.as_bytes()).unwrap()
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