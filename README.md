# modern-assembly
[![Build status](https://github.com/BoettcherDasOriginal/modern-assembly/workflows/rust-build/badge.svg)](#)
[![Release](https://img.shields.io/github/v/release/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/releases/latest)
[![GitHub milestone](https://img.shields.io/github/milestones/progress-percent/BoettcherDasOriginal/modern-assembly/1)](https://github.com/BoettcherDasOriginal/modern-assembly/milestone/1)
[![Top language](https://img.shields.io/github/languages/top/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/search?l=C%23)
[![License](https://img.shields.io/github/license/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/issues)

## Overview

Modern Assembly is an experimental programming language engineered with Rust and LLVM. It aims to bring the power of high-level languages and the efficiency of low-level assembly under one umbrella.

<em>⚠ Modern Assembly is at an early development stage. Don't expect everything to work smoothly ⚠</em>

## Features

- High-Level Syntax: Enjoy readability and ease-of-use akin to high-level languages.
- Low-Level Control: Get down to the metal, controlling every aspect of the code.
- Optimized Performance: Built with Rust and LLVM for blazing fast execution.

## Quick Start

Here is a small example that prints "Hello, World!" and performs a basic calculation:

```
fn main:
  let msg "Hello, world!"
  print msg
            
  var x 1
  add x 2 2
  print x

  if msg == x:
     print "???"
  else:
     move x 1
  end
end
```

## Language Server Protocol (LSP) Extension

We are also working on a Language Server Protocol (LSP) extension for VS Code. You can track its progress [here](https://github.com/nwrenger/modern-assembly-analyzer).

## Documentation

Further documentation will be found in the Wiki.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](https://github.com/BoettcherDasOriginal/modern-assembly/blob/main/LICENSE) file for details.