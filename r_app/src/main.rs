fn main() {
    #[cfg(target_os = "android")]
    android::main();
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(web::main());
    desktop::main();
}
