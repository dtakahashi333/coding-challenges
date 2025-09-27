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


You're seeing this Git error because you're trying to run `git pull`, but Git doesn't know **how you want to reconcile** changes between your local branch and the remote — **merge, rebase, or fast-forward**.

This happens **only when your local and remote branches have diverged** (i.e., both have new commits that the other doesn't).

---

## 💥 Error Summary:

```
You have divergent branches and need to specify how to reconcile them.
```

This is **not an error in your code**, but Git asking:

> "When pulling, should I merge, rebase, or only fast-forward?"

---

## ✅ The Quick Fix (Choose One Strategy)

Pick the pull strategy you prefer and configure Git **once**, either globally or per-repo.

### ✅ Option 1: Merge (default and safe)

```bash
git config pull.rebase false
```

Or globally:

```bash
git config --global pull.rebase false
```

🔁 This means: "When pulling, merge remote changes into mine."

---

### ✅ Option 2: Rebase (cleaner history)

```bash
git config pull.rebase true
```

Or globally:

```bash
git config --global pull.rebase true
```

🔀 This means: "Replay my changes on top of the latest remote branch."

---

### ✅ Option 3: Fast-forward only (no merge commits allowed)

```bash
git config pull.ff only
```

Or globally:

```bash
git config --global pull.ff only
```

🚫 This will **fail** if your local branch and remote have diverged.

---

## 🧪 Example Use Case

Let’s say you want to merge by default:

```bash
git config --global pull.rebase false
```

Then you can just run:

```bash
git pull
```

And Git will perform a **merge** automatically when needed.

---

## 📍 One-Time Fix (No config change)

You can also resolve the issue just once by adding a flag to `git pull`:

* Merge:

  ```bash
  git pull --no-rebase
  ```

* Rebase:

  ```bash
  git pull --rebase
  ```

* Fast-forward only:

  ```bash
  git pull --ff-only
  ```

---

## 🔍 Recommendation for Most Developers

If you're unsure, use this:

```bash
git config --global pull.rebase false
```

Because:

* It's the default in many setups
* Easier to understand
* Less risk of conflicts

---

Let me know which strategy you'd like to use, and I can help you set it up for your workflow or team.

**Conventional Commits follow a structured format:**
```text
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Common Commit Types**
| Type       | Purpose                                                 |
| ---------- | ------------------------------------------------------- |
| `feat`     | A new feature                                           |
| `fix`      | A bug fix                                               |
| `docs`     | Documentation only changes                              |
| `style`    | Code style changes (whitespace, formatting, etc.)       |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf`     | Performance improvement                                 |
| `test`     | Adding or modifying tests                               |
| `chore`    | Build process, tools, or other non-functional changes   |
| `build`    | Build system changes (e.g., Docker, CI configs)         |
| `ci`       | CI/CD pipeline changes                                  |

**Examples**
```sh
git commit -m "feat(login): add forgot password link"
git commit -m "fix(api): handle null value in user data"
git commit -m "docs(readme): update installation instructions"
git commit -m "refactor(auth): remove redundant checks"
git commit -m "chore: update dependencies"
git commit -m "test(api): add tests for token expiration"
```

**TL;DR: Template**
```text
<type>(<optional scope>): <description>

[optional longer explanation]

[BREAKING CHANGE: <explanation>]
```

Perfect — if you're interested in using **`commitlint`** and **`commitizen`**, you're aiming for a high-quality, consistent Git commit workflow using **Conventional Commits**. Here's how they work together and how to set them up.

---

## 🚀 What Are They?

### ✅ `commitlint`

* A **linter** that checks your commit messages against rules (like "must start with `feat:` or `fix:`").
* Usually runs via a **Git hook** (with [Husky](https://github.com/typicode/husky)).

### ✅ `commitizen`

* An **interactive CLI** that helps you **write commit messages** in the conventional format (so you don’t have to memorize it).

Together, they:

* 🛡️ **Prevent bad commits**
* 🧠 **Guide devs with prompts**
* 🔄 **Enable semantic releases and changelogs**

---

## ⚙️ How to Set Up `commitlint` + `commitizen`

### ✅ Step 1: Initialize a Node.js project (if not done yet)

```bash
npm init -y
```

---

### ✅ Step 2: Install required packages

```bash
npm install --save-dev commitizen commitlint @commitlint/config-conventional husky
```

---

### ✅ Step 3: Configure Commitlint

Create a file named `commitlint.config.js`:

```js
// commitlint.config.js
module.exports = {
  extends: ['@commitlint/config-conventional']
};
```

---

### ✅ Step 4: Set up Husky (to enforce commit rules)

```bash
npx husky install
```

Add a hook to check commits:

```bash
npx husky add .husky/commit-msg 'npx --no -- commitlint --edit "$1"'
```

Also, add to `package.json` to auto-setup husky on install:

```json
"scripts": {
  "prepare": "husky install"
}
```

---

### ✅ Step 5: Configure Commitizen

Let Commitizen know to use the Conventional Commit adapter:

In your `package.json`:

```json
"config": {
  "commitizen": {
    "path": "cz-conventional-changelog"
  }
}
```

Install the adapter:

```bash
npm install --save-dev cz-conventional-changelog
```

---

### ✅ Step 6: Commit with Commitizen

Use this command instead of `git commit`:

```bash
npx cz
```

Or add a shortcut:

```json
"scripts": {
  "commit": "cz"
}
```

Now you can run:

```bash
npm run commit
```

It will prompt:

```
? Select the type of change that you're committing: (Use arrow keys)
  feat:     A new feature
  fix:      A bug fix
  ...
```

---

## 📋 Example Workflow

1. Run `npm run commit`
2. Pick: `feat`
3. Scope: `login`
4. Message: `add forgot password feature`
5. Git commit message becomes:

```bash
feat(login): add forgot password feature
```

6. Husky + Commitlint ensure it passes validation.

---

## 📦 Optional Enhancements

* **`standard-version`** – Auto-generate changelogs and version bumps
* **`semantic-release`** – Full auto-publish to npm/GitHub based on commit history

---

## 🧪 Test It

Try a bad commit message:

```bash
git commit -m "updated stuff"
```

→ Husky will block it:

```
⛔️ subject may not be empty
⛔️ type may not be empty
```

---

## ✅ Summary

| Tool         | Purpose                                     |
| ------------ | ------------------------------------------- |
| `commitlint` | Lints commit messages                       |
| `commitizen` | Helps generate conventional commits         |
| `husky`      | Hooks into Git to run tools like commitlint |

---

Would you like a **template repo** or a **fully working GitHub Actions + Commitizen + Husky setup**? I can generate it for you.
