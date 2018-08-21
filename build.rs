extern crate capnpc;
extern crate failure;
extern crate sass_rs;

use failure::Error;
use sass_rs::{compile_file, Options, OutputStyle};
use std::{
    env,
    fs::{copy, read_to_string, write},
    path::{Path, PathBuf},
    process::Command,
};

const REPOSITORY: &str = "https://github.com/uikit/uikit.git";
const TAG: &str = "v3.0.0-rc.11";
const CSS_FILE: &str = "style.css";
const SCSS_FILE: &str = "style.scss";
const CAPNP_FILE: &str = "protocol.capnp";

fn main() -> Result<(), Error> {
    ::capnpc::CompilerCommand::new()
        .file(PathBuf::from("src").join(CAPNP_FILE))
        .run()
        .expect("compiling schema");
    prepare_style()?;
    Ok(())
}

fn prepare_style() -> Result<(), Error> {
    // Prepare the directory
    let out_dir = env::var("OUT_DIR")?;
    let mut target = PathBuf::from(out_dir);
    target.push("uikit");

    // Clone the repo if needed
    if !Path::new(&target).exists() {
        let status = Command::new("git")
            .arg("clone")
            .arg(format!("--branch={}", TAG))
            .arg("--recursive")
            .arg(REPOSITORY)
            .arg(&target)
            .status()?;
        if !status.success() {
            panic!("Failed to build Uikit");
        }
    }

    // Copy the scss file into the output directory
    target.pop();
    target.push(SCSS_FILE);
    copy(format!("src/frontend/{}", SCSS_FILE), &target)?;

    // Build the file
    let mut options = Options::default();
    options.output_style = OutputStyle::Compressed;
    match compile_file(&target, options) {
        Err(error) => panic!(error),
        Ok(content) => {
            // Copy the file into the static directory
            target.pop();
            target.push(CSS_FILE);
            write(&target, content)?;
            copy(&target, format!("static/css/{}", CSS_FILE))?;
        }
    }

    Ok(())
}
