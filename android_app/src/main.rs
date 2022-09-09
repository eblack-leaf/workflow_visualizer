#[cfg(target_os = "android")]
fn main() {
    android_builder::android::main();
}
