plugins {
    id("fr.stardustenterprises.rust.wrapper") version "3.2.5" apply false
}

subprojects {
    group = "org.terasology.engine"
    version = "3.0.0"

    repositories {
        mavenLocal()
        mavenCentral()
    }
}

