use std::path::Path;
mod wasm_server;
pub use wasm_server::WasmServer;
pub struct WasmCompiler {
    pub package: String,
    pub package_options: String,
    pub destination: String,
}

impl WasmCompiler {
    pub fn new<T: Into<String>>(package: T, package_options: T, destination: T) -> Self {
        Self {
            package: package.into(),
            package_options: package_options.into(),
            destination: destination.into(),
        }
    }
    pub fn compile(&self) -> Result<(), bool> {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let package = self.package.as_str();
        let profile = self.package_options.as_str();
        let project_root = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf();
        let target = Path::new("wasm_rebuild_avoidance_target");
        let mut args = Vec::<&str>::new();
        args.push("build");
        if profile == "release" {
            args.push("--release");
        }
        args.push("--target");
        args.push("wasm32-unknown-unknown");
        args.push("--package");
        args.push(&package);
        args.push("--target-dir");
        args.push(target.as_os_str().to_str().unwrap());
        args.push("--bin");
        args.push(&package);
        let status = std::process::Command::new(&cargo)
            .current_dir(&project_root)
            .args(&args)
            .status()
            .unwrap();
        if !status.success() {
            return Err(true);
        }
        let absolute_target = project_root.join(&target);
        let source = absolute_target
            .join("wasm32-unknown-unknown")
            .join(&profile)
            .join(format!("{}.wasm", &package));
        let destination = project_root.join(self.destination.as_str());
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
        Ok(())
    }
}