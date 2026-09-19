#!/usr/bin/env bash

find src/ -type f \( -name '*.cpp' -o -name '*.hpp' \) -exec clang-tidy -p build {} +
