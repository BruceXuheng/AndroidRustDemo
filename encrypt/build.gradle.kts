plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.mozilla.rust-android-gradle.rust-android") version "0.9.3"
}

extensions.configure(com.nishtahir.CargoExtension::class) {
    module = "./rust"  // Or whatever directory contains your Cargo.toml
    libname = "brencrypt" // Or whatever matches Cargo.toml's [package] name.
    targets = listOf("arm64","arm") // See bellow for a longer list of options
    // profile = "release" // The Cargo release profile to build.
    prebuiltToolchains = true
    targetDirectory = "./src/main/jniLibs/"
    // targetIncludes = ['libnotlibname.so']
}


// 1. windows use gradlew.bat cargobuild  (my test this success)
// 2. config write
// tasks.whenTaskAdded {
//     if ((this.name == "javaPreCompileDebug" || this.name == "javaPreCompileRelease")) {
//         this.dependsOn+" cargoBuild"
//     }
// }



android {
    namespace = "com.example.android.encrypt"
    compileSdk = 33
    ndkVersion = "23.1.7779620"

    defaultConfig {
        minSdk = 24

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }

    tasks.whenTaskAdded {
        if ((this.name == "javaPreCompileDebug" || this.name == "javaPreCompileRelease")) {
            this.dependsOn.add("cargoBuild")

        }
    }

    tasks.preBuild.configure {
        dependsOn.add(tasks.withType(com.nishtahir.CargoBuildTask::class.java))
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.9.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.8.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}