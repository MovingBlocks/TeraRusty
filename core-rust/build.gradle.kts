import org.apache.tools.ant.taskdefs.condition.Os
import java.net.URI

plugins {
    id("base")
    id("maven-publish")
}

//configurations {
//    rust
//}
//
//dependencies {
//    rust(fileTree("natives") { include ("*.rs")})
//}

val natives = mutableMapOf<String, String>()
val baseDir = "${projectDir.toString()}"


ext {
    if (Os.isFamily(Os.FAMILY_MAC)) {
//        natives = listOf("macosx_amd64_clang")
    } else if (Os.isFamily(Os.FAMILY_UNIX)) {
        natives["target"] = "x86_64-unknown-linux-gnu"
        natives["module"] = "linux-amd64"
    } else if (Os.isFamily(Os.FAMILY_WINDOWS)) {
        natives["target"] = "x86_64-pc-windows-msvc"
        natives["module"] = "windows-amd64"
    } else {
        throw GradleException("Unsupported platform")
    }
}

version = "0.0.1"
group = "org.terasology.rust"

// We use both Maven Central and our own Artifactory instance, which contains module builds, extra libs, and so on
repositories {
    mavenCentral()

    // Terasology Artifactory instance for libs not readily available elsewhere plus our own libs
    maven {
        val repoViaEnv = System.getenv()["RESOLUTION_REPO"]
        if (rootProject.hasProperty("alternativeResolutionRepo")) {
            // If the user supplies an alternative repo via gradle.properties then use that
            name = "from alternativeResolutionRepo property"
            url = URI(properties["alternativeResolutionRepo"].toString())
        } else if (repoViaEnv != null && repoViaEnv != "") {
            name = "from \$RESOLUTION_REPO"
            url = URI(repoViaEnv)
        } else {
            // Our default is the main virtual repo containing everything except repos for testing Artifactory itself
            name = "Terasology Artifactory"
            url = URI("http://artifactory.terasology.org/artifactory/virtual-repo-live")
            isAllowInsecureProtocol = true
        }
    }
}

tasks.register<Exec>(name = "native_${natives["target"]}") {
    description = "cargo ${natives["target"]} "
    executable = "cargo"
    workingDir("$baseDir/natives")
    args = listOf("build", "--target=${natives["target"]}")
    doFirst {
        mkdir("$baseDir/build/natives")
    }
    doLast {
        copy {
            from("$baseDir/natives/target/${natives["target"]}/debug")
            include("*.so")
            include("*.dll")
            rename("(.+).so", "\$1-${natives["module"]}.so")
            rename("(.+).dll", "lib\$1-${natives["module"]}.dll")
            into("$baseDir/build/natives")
        }
    }
}


tasks.register("buildNatives") {
    description = "Builds Natives"
    dependsOn(fileTree("$baseDir/natives")) // Input directory containing the source files
    dependsOn("native_${natives["target"]}")
}

// TODO: outputs are not defined well enough yet for Gradle to skip this if already done (maybe more the natives task?)
val zipNatives by tasks.creating(Zip::class) {
    description = "Creates a zip archive that contains all TeraBullet native files"
    dependsOn("buildNatives")

    from("$baseDir/build/natives") {
        include("*linux*")
        into("linux")
    }

    from("$baseDir/build/natives") {
        include("*osx*")
        into("macosx")
    }

    from("$baseDir/build/natives") {
        include("*windows*")
        into("windows")
    }

    destinationDirectory.set(file(layout.buildDirectory))
    archiveBaseName.set("core-rust")
}

artifacts.add("default", zipNatives)
//tasks.register<Zip>("zipNatives") {
//    description = "Creates a zip archive that contains all TeraBullet native files"
//    dependsOn("buildNatives")
//
//    from("$baseDir/build/natives") {
//        include("*linux*")
//        into("linux")
//    }
//
//    from("$baseDir/build/natives") {
//        include("*osx*")
//        into("macosx")
//    }
//
//    from("$baseDir/build/natives") {
//        include("*windows*")
//        into("windows")
//    }
//
//    destinationDirectory.set(file(layout.buildDirectory))
//    archiveBaseName.set("core-rust")
//    artifacts.add("default", archiveFile)
//}

tasks.getByName("publish") {
    dependsOn("zipNatives")
}

tasks.build {
    dependsOn("buildNatives")
    dependsOn("zipNatives")
}

// Define the artifacts we want to publish (the .pom will also be included since the Maven plugin is active)

publishing {
    publications {
        create<MavenPublication>("TeraRustyCoreRust") {
            artifact(zipNatives)
            repositories {
                maven {
                    name = "TerasologyOrg"
                    isAllowInsecureProtocol = true
                    if (project.hasProperty("publishRepo")) {
                        val publishRepo = project.property("publishRepo")
                        // This first option is good for local testing, you can set a full explicit target repo in gradle.properties
                        url = URI("http://artifactory.terasology.org/artifactory/$publishRepo")
                        logger.info("Changing PUBLISH repoKey set via Gradle property to $publishRepo")
                    } else {
                        // Support override from the environment to use a different target publish org
                        var deducedPublishRepo: String? = System.getenv()["PUBLISH_ORG"]
                        if (deducedPublishRepo.isNullOrEmpty()) {
                            // If not then default
                            deducedPublishRepo = "libs"
                        }

                        // Base final publish repo on whether we're building a snapshot or a release

                        if (project.version.toString().endsWith("SNAPSHOT")) {
                            deducedPublishRepo += "-snapshot-local"
                        } else {
                            deducedPublishRepo += "-release-local"
                        }

                        logger.info("The final deduced publish repo is {}", deducedPublishRepo)
                        url = URI("http://artifactory.terasology.org/artifactory/$deducedPublishRepo")
                    }
                    if (project.hasProperty("mavenUser") && project.hasProperty("mavenPass")) {
                        credentials {
                            val mavenUser =
                                project.property("mavenUser") as? String
                                    ?: throw RuntimeException("Not a valid maven user")
                            val mavenPass =
                                project.property("mavenPass") as? String
                                    ?: throw RuntimeException("Not a valid maven pass")
                            username = mavenUser
                            password = mavenPass

                            authentication {
                                create<BasicAuthentication>("basic")
                            }
                        }
                    }
                }
            }
        }
    }
}