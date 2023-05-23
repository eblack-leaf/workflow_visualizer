cd ~/Desktop/dev/workflow_visualizer/application/
export ANDROID_NDK_HOME="/home/omi-voshuli/Desktop/dev/android_sdk/ndk/25.2.9519653"
export ANDROID_HOME="/home/omi-voshuli/Desktop/dev/android_sdk"
cargo ndk -t arm64-v8a -o /home/omi-voshuli/Desktop/dev/workflow_visualizer/application/app/src/main/jniLibs/ build --package application --target-dir android_rebuild_avoidance_target
./gradlew build
./gradlew installDebug
adb shell am start -n co.workflow.visualizer/.MainActivity
