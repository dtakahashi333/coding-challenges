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
