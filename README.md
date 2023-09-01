# modern-assembly
[![Build status](https://github.com/BoettcherDasOriginal/modern-assembly/workflows/rust-build/badge.svg)](#)
[![Release](https://img.shields.io/github/v/release/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/releases/latest)
[![Top language](https://img.shields.io/github/languages/top/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/search?l=C%23)
[![License](https://img.shields.io/github/license/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues/BoettcherDasOriginal/modern-assembly)](https://github.com/BoettcherDasOriginal/modern-assembly/issues)

Modern Assembly is an experimental native language.<br>
Its main goal is to mix some parts of high-level languages with the good old parts of low-level assembler.

<em>⚠ Modern Assembly is at an early development stage. Don't expect everything to work smoothly ⚠</em>
```
fn main:
  var msg "Hello, world!"
  print msg
            
  var x 1
  add x 2 2
  print x

  if msg == x:
     print "???"
  else:
     move x 1
```
## To Do's
- [x] Add Lexer
- [ ] Add Parser (that's a lot of work)