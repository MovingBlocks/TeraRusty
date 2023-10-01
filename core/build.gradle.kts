import java.net.URI

plugins {
    id("base")
    id("java-library")
    id("maven-publish")
}

val baseDir = projectDir.toString()

dependencies {
    api("org.joml:joml:1.10.0")
    api("org.slf4j:slf4j-api:1.7.21")
    api("net.sf.trove4j:trove4j:3.0.3")
    api("org.terasology.joml-ext:joml-geometry:0.1.0")
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

val doctask by tasks.creating(Javadoc::class) {
    isFailOnError = false
}

val javaDocJar by tasks.creating(Jar::class) {
    archiveClassifier.set("javadoc")
    dependsOn(doctask)
    description = "Create a JAR with the JavaDoc for the java sources"
    from(doctask.destinationDir)
}

val sourceJar by tasks.creating(Jar::class) {
    archiveClassifier.set("sources")
    description = "Create a JAR with all sources"
    from(sourceSets.main.get().allSource)
    from(sourceSets.test.get().allSource)
}

tasks.getByName("publish") {
    dependsOn(sourceJar, javaDocJar)
}

publishing {
    publications {
        create<MavenPublication>("TeraRustyCore") {
            // Without this we get a .pom with no dependencies
            from(components["java"])
            artifacts {
                artifact(sourceJar)
                artifact(javaDocJar)
            }
            pom.withXml {
                asNode().apply {
                    appendNode("name", "core")
                    appendNode("description", "A Java Native Terasology Core Wrapper")
                    appendNode("url", "http://www.example.com/project")
                    appendNode("licenses").appendNode("license").apply {
                        appendNode("name", "The Apache License, Version 2.0")
                        appendNode("url", "http://www.apache.org/licenses/LICENSE-2.0.txt")
                    }
                    appendNode("developers").appendNode("developer").apply {
                        appendNode("id", "michaelpollind")
                        appendNode("name", "Michael Pollind")
                        appendNode("email", "mpollind@gmail.com")
                    }
                    appendNode("scm").apply {
                        appendNode("connection", "https://github.com/MovingBlocks/JNBullet")
                        appendNode("developerConnection", "git@github.com:MovingBlocks/JNBullet.git")
                        appendNode("url", "https://github.com/MovingBlocks/JNBullet")
                    }
                }
            }
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