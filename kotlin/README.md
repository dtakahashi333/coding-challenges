Creating a Kotlin unit test in IntelliJ is pretty straightforward! Here’s a step-by-step guide to get you started:

---

## How to Create a Kotlin Unit Test in IntelliJ

### 1. **Make sure you have a test framework dependency**

* If you use Gradle, your `build.gradle.kts` should include:

```kotlin
dependencies {
    testImplementation(kotlin("test"))           // Kotlin test library
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.0") // JUnit 5
}
```

* Sync your Gradle project (`View → Tool Windows → Gradle → Refresh`).

---

### 2. **Create the test source folder (if it doesn’t exist)**

By convention, test Kotlin files go in:

```
src/test/kotlin/
```

If the folder isn’t marked as a test source root:

* Right-click on `src/test/kotlin`
* Choose **Mark Directory As → Test Sources Root**

---

### 3. **Create a new test class**

* Right-click the package or folder inside `src/test/kotlin`
* Select **New → Kotlin Class/File**
* Name it something like `MyClassTest`

---

### 4. **Write a simple test function**

Example: Testing a function `sum(a, b)` in `MyClass.kt`

```kotlin
import kotlin.test.Test
import kotlin.test.assertEquals

class MyClassTest {

    @Test
    fun testSum() {
        val result = sum(2, 3)
        assertEquals(5, result)
    }
}
```

---

### 5. **Run your test**

* Right-click inside the test file or on the test function
* Click **Run 'MyClassTest'**

You’ll see the test results in IntelliJ’s Run window.

---

## Bonus Tips:

* IntelliJ can **generate test stubs** for you:

    * Right-click on a function or class → **Generate → Test...**
* You can run all tests from the Gradle tool window or using `./gradlew test` in the terminal.

---

If you want, I can help you create a full example project with tests configured! Just ask.
