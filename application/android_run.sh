cd ~/Desktop/dev/workflow_visualizer/application/
export ANDROID_NDK_HOME="/home/omi-voshuli/Desktop/dev/android_sdk/ndk/25.2.9519653"
export ANDROID_HOME="/home/omi-voshuli/Desktop/dev/android_sdk"
cargo ndk -t arm64-v8a -o ./android_src/app/src/main/jniLibs/ build --package application
# shellcheck disable=SC2164
cd android_src
./gradlew build
#./gradlew installDebug
#adb shell am start -n co.workflow.visualizer/.MainActivity
#adb shell pidof co.workflow.visualizer
#adb shell logcat -v color --pid= > logcat1.txt