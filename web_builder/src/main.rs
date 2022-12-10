use std::path::Path;
use std::process::Command;

fn main() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let package = "r_app";
    let profile = "debug";
    let project_root = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf();
    let target = Path::new("wasm_rebuild_avoidance_target");
    let mut args = Vec::<&str>::new();
    args.push("build");
    if profile == "release" {
        args.push("--release")
    }
    args.push("--target");
    args.push("wasm32-unknown-unknown");
    args.push("--package");
    args.push(&package);
    args.push("--target-dir");
    args.push(target.as_os_str().to_str().unwrap());
    let status = Command::new(&cargo)
        .current_dir(&project_root)
        .args(&args)
        .status()
        .unwrap();
    if !status.success() {
        return;
    }
    let absolute_target = project_root.join(&target);
    let source = absolute_target
        .join("wasm32-unknown-unknown")
        .join(&profile)
        .join(format!("{}.wasm", &package));
    let destination = project_root.join(format!("{}_build", &package));
    std::fs::create_dir_all(&destination).unwrap();
    let mut bindgen = wasm_bindgen_cli_support::Bindgen::new();
    bindgen
        .web(true)
        .unwrap()
        .omit_default_module_path(false)
        .input_path(&source)
        .generate(&destination)
        .unwrap();
    let template = include_str!("index.template.html");
    let processed = template.replace("{{name}}", &package);
    std::fs::write(destination.join("index.html"), processed).unwrap();
}
