ARCH=x86_64
RARCH=$ARCH
mkdir -p apk/libs/lib/$ARCH
SONAME=$(grep android.app.lib_name AndroidManifest.xml | grep -oP 'android:value="\K(.*)"')
OUT="apk/libs/lib/$ARCH/lib"${SONAME%\"}".so"

cp target/$RARCH-linux-android/release/librust_vk_main.so $OUT

readelf $OUT -d > sym.txt
readelf $OUT -s >> sym.txt
VALIDATION_LAYER=$ANDROID_NDK_HOME/sources/third_party/vulkan/src/build-android/jniLibs/$ARCH/libVkLayer_khronos_validation.so
cp $VALIDATION_LAYER apk/libs/lib/$ARCH
aapt package -f -M AndroidManifest.xml -I $ANDROID_SDK_ROOT/platforms/android-30/android.jar -A assets -F apk/app0.apk apk/libs
zipalign -f -v 4 apk/app0.apk apk/app.apk
signapk.sh apk/app.apk
apkanalyzer files list apk/app.apk
#adb uninstall com.native_activity
#adb install apk/app.apk
#adb logcat -c
#adb shell monkey -p com.native_activity 1
##gnome-terminal -e "adb logcat"
##adb logcat -s native-activity:D vulkan:D ActivityManager:I libc:I DEBUG:I AndroidRuntime:I VALIDATION:I threaded_app:I
#gnome-terminal -e "adb logcat -s native-activity:D threaded_app:D vulkan:D ActivityManager:I libc:I DEBUG:I AndroidRuntime:I VALIDATION:I"
