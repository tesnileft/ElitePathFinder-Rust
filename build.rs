use std::process::Command;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        std::env::set_var("GSETTINGS_SCHEMA_DIR", "data");
    }

    println!("cargo:rerun-if-changed=src/resources");
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "elite_pathfinder.gresource",
    );
    println!("cargo:rerun-if-changed=data");
    let status = Command::new("glib-compile-schemas")
        .arg("data")
        .status()
        .expect("Failed to run glib-compile-schemas");

    assert!(status.success());
}
