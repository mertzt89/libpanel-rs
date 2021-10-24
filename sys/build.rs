#[cfg(not(feature = "dox"))]
use std::process;

mod build_version;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if std::env::var("SYSTEM_DEPS_LIBPANEL_1_BUILD_INTERNAL").is_err() {
        std::env::set_var("SYSTEM_DEPS_LIBPANEL_1_BUILD_INTERNAL", "auto");
    }
    let res = system_deps::Config::new()
        .add_build_internal("libpanel_1", |_lib, version| {
            use std::{env, path::PathBuf};

            let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
            let profile = env::var("PROFILE").unwrap();
            let build_dir = out_dir.join("meson-build");
            if !build_dir.join("build.ninja").as_path().exists() {
                process::Command::new("meson")
                    .arg("setup")
                    .arg(&build_dir)
                    .arg(&manifest_dir.join(".."))
                    .arg(&format!("-Dbuildtype={}", profile))
                    .status()
                    .and_then(handle_exit_status)
                    .expect("Running meson failed");
            }
            process::Command::new("meson")
                .arg("compile")
                .arg("-C")
                .arg(&build_dir)
                .arg("panel-1")
                .status()
                .and_then(handle_exit_status)
                .expect("Running meson failed");
            let pc_dir = build_dir.join("meson-uninstalled");
            system_deps::Library::from_internal_pkg_config(
                &pc_dir,
                "libpanel-1-uninstalled",
                version,
            )
        })
        .probe();
    if let Err(s) = res {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}

fn handle_exit_status(status: std::process::ExitStatus) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, format!("{:?}", status.code())))
    }
}
