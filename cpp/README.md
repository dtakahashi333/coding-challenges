Sure! Below is a **clean, minimal example of Meson unit tests using C++**, with **two variants**:

1. **Plain C++ (`assert`)** – simplest, no external framework
2. **GoogleTest** – most common real-world setup

You can pick whichever matches your needs.

---

# ✅ Variant 1: Plain C++ unit test (no framework)

## 📁 Project layout

```
meson-cpp-demo/
├── meson.build
├── src/
│   ├── meson.build
│   ├── math.cpp
│   └── math.h
└── tests/
    ├── meson.build
    └── test_math.cpp
```

---

## Source code

### `src/math.h`

```cpp
#pragma once

int add(int a, int b);
```

### `src/math.cpp`

```cpp
#include "math.h"

int add(int a, int b) {
    return a + b;
}
```

---

## Unit test

### `tests/test_math.cpp`

```cpp
#include <cassert>
#include "math.h"

int main() {
    assert(add(2, 3) == 5);
    assert(add(-1, 1) == 0);
    return 0;
}
```

---

## Meson files

### Top-level `meson.build`

```meson
project('meson-cpp-demo', 'cpp',
  default_options : ['cpp_std=c++17']
)

subdir('src')
subdir('tests')
```

---

### `src/meson.build`

```meson
math_lib = library(
  'math',
  'math.cpp',
  include_directories : include_directories('.'),
)

math_dep = declare_dependency(
  include_directories : include_directories('.'),
  link_with : math_lib,
)
```

---

### `tests/meson.build`

```meson
test_exe = executable(
  'test_math',
  'test_math.cpp',
  dependencies : math_dep,
)

test('math tests', test_exe)
```

---

## Build & run

```bash
meson setup build
meson compile -C build
meson test -C build
```

---

# ✅ Variant 2: C++ unit tests with GoogleTest (recommended)

---

## 📁 Layout (same structure)

```
meson-cpp-gtest/
├── meson.build
├── src/
├── tests/
```

---

## `tests/test_math.cpp`

```cpp
#include <gtest/gtest.h>
#include "math.h"

TEST(MathTest, AddPositive) {
    EXPECT_EQ(add(2, 3), 5);
}

TEST(MathTest, AddNegative) {
    EXPECT_EQ(add(-1, 1), 0);
}
```

---

## `tests/meson.build` (GoogleTest)

```meson
gtest_dep = dependency('gtest', main : true, required : true)

test_exe = executable(
  'test_math',
  'test_math.cpp',
  dependencies : [math_dep, gtest_dep],
)

test('math gtest', test_exe)
```

Meson will:

* Find system-installed GoogleTest, or
* Fail with a clear error if missing

---

## Running

```bash
meson test -C build
```

Output includes:

* Test names
* Pass/fail info

---

# 🔍 When to use which?

| Approach   | When to use                       |
| ---------- | --------------------------------- |
| `assert`   | Small projects, learning Meson    |
| GoogleTest | Real projects, CI, test reporting |

---

# 💡 Meson testing tips

* `meson test --verbose` → see test output
* `test(..., timeout: 30)` → limit runtime
* `test(..., env: ['VAR=value'])` → set env vars
* `meson test --gdb` → debug failures

---

## ✅ Summary

* Meson treats tests as **executables**
* C++ tests integrate cleanly
* GoogleTest works out-of-the-box
* Setup is minimal and clean

If you want, I can also show:

* **Catch2 with Meson**
* **CTest vs Meson tests**
* **Coverage reports (gcov / lcov)**
* **Cross-platform CI setups**

Just ask 👍


Ah! That error happens when Meson tries to find **GoogleTest (`gtest`) via pkg-config**, but **pkg-config cannot locate it**. On macOS (or some Linux setups), GoogleTest isn’t always installed system-wide, so `pkg-config` fails.

Let’s go step by step.

---

## 1️⃣ Why it happens

Your `tests/meson.build` probably has something like:

```meson
gtest_dep = dependency('gtest', required: true)
```

* By default, Meson tries multiple lookup methods: `pkg-config`, `cmake`, system paths.
* If you don’t have GoogleTest installed **system-wide**, `pkg-config` cannot find it → error.

---

## 2️⃣ Options to fix it

### Option A: Install GoogleTest system-wide (Homebrew on macOS)

```bash
brew install googletest
```

Then tell Meson to find it via **CMake**:

```meson
gtest_dep = dependency('gtest', method: 'cmake', required: true)
```

* `method: 'cmake'` avoids pkg-config and uses CMake’s export files.

---

### Option B: Use Meson’s `subproject()` (recommended)

This is the **most portable solution**, avoids system dependencies.

1. Create a folder in your project:

```
subprojects/googletest/...
```

2. Add GoogleTest as a **Meson subproject** (downloaded from GitHub or via `meson wrap`):

```bash
meson wrap install googletest
```

3. In your `tests/meson.build`:

```meson
gtest_dep = dependency('gtest', fallback: ['googletest', 'gtest_dep'])
```

* Meson will **build GoogleTest automatically** if it’s not found system-wide.

---

### Option C: Build GoogleTest manually

1. Clone GoogleTest:

```bash
git clone https://github.com/google/googletest.git subprojects/googletest
```

2. Use a `wrap` file or a manual subproject Meson build.

---

### Option D: Use `assert` instead of GoogleTest

For small LeetCode-style projects:

```cpp
#include <cassert>
#include "leet_code/easy/xxx.hpp"

int main() {
    assert(solution(42) == 42);
}
```

* No dependency required
* Works with your current Meson setup
* Recommended if you just want **fast unit testing**

---

## 3️⃣ Quick fix for macOS + Homebrew

```meson
gtest_dep = dependency('gtest', method: 'cmake', required: true)
brew install googletest
meson setup build
```

---

### TL;DR

| Situation                      | Fix                                                     |
| ------------------------------ | ------------------------------------------------------- |
| pkg-config fails to find gtest | Install googletest via Homebrew + use `method: 'cmake'` |
| Want portable / CI-friendly    | Use Meson subproject (`wrap`)                           |
| Small project                  | Skip gtest and use `assert`                             |

---

If you want, I can show a **full working Meson project layout for your LeetCode folder structure** with **GoogleTest integrated**, ready to build and test.

Do you want me to do that?
