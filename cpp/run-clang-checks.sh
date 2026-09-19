#!/usr/bin/env bash

set -euo pipefail

# Run clang-format in include, src and tests directories.
find include src tests -type f \( -name '*.cpp' -o -name '*.hpp' \) -exec clang-format -i {} +

# Run clang-tidy in include, src and tests directories.
find include src tests -type f \( -name '*.cpp' -o -name '*.hpp' \) -exec clang-tidy -p build {} +
