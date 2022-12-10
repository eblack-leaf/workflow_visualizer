fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(r_app_lib::web());
    r_app_lib::native();
}
