# AndroidVulkanTriangle

Global Config

File : $HOME/.cargo/config.toml
~~~
[target.i686-linux-android]
linker = "android-sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android30-clang++"
rustflags = ["-C", "link-args=-landroid -llog -shared"]

[target.x86_64-linux-android]
linker = "android-sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android30-clang++"
rustflags = ["-C", "link-args=-landroid -llog -shared"]

[target.aarch64-linux-android]
linker = "android-sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang++"
rustflags = ["-C", "link-args=-landroid -llog -shared"]

[term]
verbose = true        # whether cargo provides verbose output
color = true
~~~


File : $PATH/signapk.sh
~~~
mkdir -p $HOME/.keys
rm $HOME/.keys/debug.keystore
keytool -genkey -v -keystore $HOME/.keys/debug.keystore -storepass keydebug -alias debugkey -keypass keydebug -dname "CN=Android Debug,O=Android,C=US" -keyalg RSA -keysize 2048 -validity 100
apksigner sign --ks $HOME/.keys/debug.keystore --ks-pass pass:keydebug $1
~~~
