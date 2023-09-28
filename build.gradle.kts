
plugins {
    idea apply true
}

// Using this instead of allprojects allows this project to be embedded yet not affect parent projects
group = "org.terasology"
subprojects {
    group = "org.terasology.rust"
}