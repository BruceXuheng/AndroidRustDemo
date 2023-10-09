pluginManagement {
    repositories {
        maven(url = "https://plugins.gradle.org/m2/")
        maven(url = "https://maven.aliyun.com/nexus/content/groups/public/")
        maven(url = "https://maven.aliyun.com/nexus/content/repositories/google")
        maven(url = "https://maven.aliyun.com/nexus/content/repositories/gradle-plugin")

        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        maven(url = "https://plugins.gradle.org/m2/")
        maven(url = "https://maven.aliyun.com/nexus/content/groups/public/")
        maven(url = "https://maven.aliyun.com/nexus/content/repositories/google")
        maven(url = "https://maven.aliyun.com/nexus/content/repositories/gradle-plugin")
        google()
        mavenCentral()
    }
}

rootProject.name = "AndroidRustDemo"
include(":app")
include(":encrypt")
