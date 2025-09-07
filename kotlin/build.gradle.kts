plugins {
    kotlin("jvm") version "2.2.0"
}

group = "com.takahashi.app"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test")) // Kotlin test library
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.0") // JUnit 5
}

tasks.test {
    useJUnitPlatform()
}
kotlin {
    jvmToolchain(21)
}