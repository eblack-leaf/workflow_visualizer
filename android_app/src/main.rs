fn main() {
    #[cfg(target_os = "android")]
    android_builder::android::main();
}
