plugins {
    id("java-library")
    id("maven-publish")
    id("fr.stardustenterprises.rust.importer")
    id("com.github.johnrengelman.shadow") version "5.0.0"
}

dependencies {
    implementation("fr.stardustenterprises", "yanl", "0.7.4")
    implementation("fr.stardustenterprises", "plat4k", "1.6.2")
    rust(project(":rust-library"))
}


rustImport {
    baseDir.set("/META-INF/natives")
    layout.set("hierarchical")
}

tasks {
    shadowJar {
        archiveClassifier.set("")
    }
}
