# coding-challenges


You can generate a complete Swift project from the command line using **Swift Package Manager (SwiftPM)**, which is built into Swift.

Here’s exactly how to do it:

---

## ✅ Generate a New Swift Project

```bash
swift package init --type library
```

### Options:

* `--type library`: Creates a Swift **library** project (with `Sources/` and `Tests/`)
* `--type executable`: Creates a Swift **command-line tool** (with a `main.swift`)

---

## 📁 This creates the following structure:

For `--type library`:

```
MySwiftProject/
├── Package.swift
├── Sources/
│   └── MySwiftProject/
│       └── MySwiftProject.swift
├── Tests/
│   └── MySwiftProjectTests/
│       ├── MySwiftProjectTests.swift
│       └── XCTestManifests.swift
```

---

## ✅ Customize the Folder Name

If you want to name the project, pass a folder:

```bash
swift package init --type library --name DSA
```

or create a directory manually and run init inside it:

```bash
mkdir DSA
cd DSA
swift package init --type library
```

---

## 🚀 Run and Test It

```bash
cd DSA
swift build       # Compiles the package
swift test        # Runs the unit tests
```

---

## 🛠️ Customize Further

* You can add subfolders like `BinaryTree` under `Sources/DSA/`
* Update `Package.swift` if you restructure beyond SwiftPM's defaults

---

## 💡 Tip: Common Flags

| Command              | Purpose                          |
| -------------------- | -------------------------------- |
| `swift package init` | Initializes a Swift package      |
| `--type executable`  | For a command-line app           |
| `--type library`     | For reusable code module         |
| `--name <Name>`      | Sets the package/module name     |
| `swift build`        | Builds the code                  |
| `swift test`         | Runs all tests                   |
| `swift run`          | Runs main executable (if exists) |

---

Let me know if you want to scaffold a project with custom submodules like `DSA/BinaryTree` pre-setup!



Great — that's a clean and modular Swift project structure. To achieve:

```
MySwiftProject/
├── Package.swift
├── Sources/
│   └── DSA/
│       └── BinaryTree/
│           └── BinaryTree.swift
├── Tests/
│   └── DSA/
│       └── BinaryTreeTests/
│           └── BinaryTreeTests.swift
```

you’ll need to manually set up the folder structure and update the `Package.swift` file to recognize the custom target and test target.

---

## ✅ Step-by-Step: Create Custom Module & Test Structure

### 1. **Initialize the Swift package**

```bash
swift package init --name MySwiftProject
cd MySwiftProject
```

### 2. **Create your desired folder structure**

```bash
mkdir -p Sources/DSA/BinaryTree
mkdir -p Tests/DSA/BinaryTreeTests
```

Move the default files out of the way (or delete them if you want a clean start):

```bash
rm -rf Sources/MySwiftProject
rm -rf Tests/MySwiftProjectTests
```

### 3. **Create your files**

Create the source file:

```bash
touch Sources/DSA/BinaryTree/BinaryTree.swift
```

Example content:

```swift
// BinaryTree.swift
public struct BinaryTree {
    public init() {}

    public func isEmpty() -> Bool {
        return true
    }
}
```

Create the test file:

```bash
touch Tests/DSA/BinaryTreeTests/BinaryTreeTests.swift
```

Example content:

```swift
// BinaryTreeTests.swift
import XCTest
@testable import DSA

final class BinaryTreeTests: XCTestCase {
    func testIsEmpty() {
        let tree = BinaryTree()
        XCTAssertTrue(tree.isEmpty())
    }
}
```

### 4. **Edit `Package.swift`**

Update your `Package.swift` to define a new target (`DSA`) and a test target.

```swift
// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MySwiftProject",
    products: [
        .library(
            name: "DSA",
            targets: ["DSA"]),
    ],
    dependencies: [
        // No external dependencies for now
    ],
    targets: [
        .target(
            name: "DSA",
            path: "Sources/DSA"
        ),
        .testTarget(
            name: "DSATests",
            dependencies: ["DSA"],
            path: "Tests/DSA"
        ),
    ]
)
```

> 🔹 You can name the test target anything, but it's common to suffix it with `Tests`.

---

### 5. **Build & Test**

From the root of your project:

```bash
swift build
swift test
```

You should see your `BinaryTreeTests` run and pass ✅

---

## ✅ Summary

You now have:

* A modular structure (`DSA` is its own module)
* Organized subfolders inside both `Sources/` and `Tests/`
* Unit tests properly set up with Swift Package Manager

---

Would you like a shell script that generates this structure automatically?
