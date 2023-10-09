---
title: Android Rust NDK
date: 2023-10-08 15:36:41
tags:

---

一、集成Rust&NDK开发

1. 创建 moudle [:encrypt] 将完成Rust&NDK

2. 当前项目 build.gradle.kts  中引入插件

   > [Rust Android Gradle Plugin](https://github.com/mozilla/rust-android-gradle#rust-android-gradle-plugin)

   1.  id("org.mozilla.rust-android-gradle.rust-android") version "0.9.3" apply false

   ```
   // Top-level build file where you can add configuration options common to all sub-projects/modules.
   plugins {
       id("com.android.application") version "8.1.0" apply false
       id("org.jetbrains.kotlin.android") version "1.8.10" apply false
       id("com.android.library") version "8.1.0" apply false
       id("org.mozilla.rust-android-gradle.rust-android") version "0.9.3" apply false
   }
   ```

3. 在 [:encrypt] 中构建rust结构

   1. 在当前moudle下执行命令 `cargo new rust`

      目录结构如下

      ```
      encrypt
       -libs
       -rust
       	--src
       	--.gitignore
       	--Cargo.lock
       	--Cargo.toml
       -src
       -.gitignore
       -build.gradle.kts
       -consumer-rules.pro
       -proguard-rules.pro
      ```

   2. 配置Cargo.toml

      ```
      [package]
      name = "brencrypt"
      version = "0.1.0"
      edition = "2021"

      # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

      [dependencies]
      jni = "0.18.0"

      [lib]
      crate_type = ["staticlib", "cdylib"]
      ```



4. 在 [:encrypt] 中集成 mozilla.rust-android-gradle.

   > 具体说明查看 [Rust Android Gradle Plugin](https://github.com/mozilla/rust-android-gradle#rust-android-gradle-plugin)

   1. id("org.mozilla.rust-android-gradle.rust-android") version "0.9.3"

      ```
      plugins {
          id("com.android.library")
          id("org.jetbrains.kotlin.android")
          id("org.mozilla.rust-android-gradle.rust-android") version "0.9.3"
      }
      ```

   2. CargoExtension配置

      ```
      extensions.configure(com.nishtahir.CargoExtension::class) {
          module = "./rust"  // Or whatever directory contains your Cargo.toml
          libname = "brencrypt" // Or whatever matches Cargo.toml's [package] name.
          targets = listOf("arm","x86","arm64") // See bellow for a longer list of options
          profile = "release" // The Cargo release profile to build.
          prebuiltToolchains = true
          // targetIncludes = ['libnotlibname.so']
      }
      ```

5. 在项目下执行 `gradlew.bat cargobuild`

6. so文件转移

> 目前 Mozilla 插件还不支持AGP8.0 有不少问题， 没办法自动迁移so

​		手动转移到  [:encrypt] 中jniLibs中

```
main
java
 -jniLibs
 	--arm64-v8a
  		---libbrencrypt.so
 	--armeabi-v7a
  		---libbrencrypt.so
```





