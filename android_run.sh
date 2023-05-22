cd ~/Desktop/dev/workflow_visualizer/application/
export ANDROID_NDK_HOME="/home/omi-voshuli/Desktop/dev/android_sdk/ndk/25.2.9519653"
export ANDROID_HOME="/home/omi-voshuli/Desktop/dev/android_sdk"
#cargo apk run --package application
cargo ndk -t arm64-v8a -o /home/omi-voshuli/Desktop/dev/workflow_visualizer/application/app/src/main/jniLibs/ build --package application
./gradlew build
./gradlew installDebug
adb shell am start -n co.realfit.agdkwinitwgpu/.MainActivity
