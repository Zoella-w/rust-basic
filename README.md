# Rust Basic Course

Some notes and coding during my rust learning. The link of the course is as follows:

https://www.bilibili.com/video/BV15y421h7j7?spm_id_from=333.788.videopod.episodes&vd_source=69ac93649ea21c4726fe85f272b6d968

# Directory Structure

- Chapter1 Basic Knowledge
- Chapter2 Variables and Simple Types
  - ch2.1 Variables
  - ch2.2 Const and Static
  - ch2.3 Simple Types
  - ch2.4 Tuple and Array
- Chapter3 Ownership
  - ch3.1 Ownership
  - ch3.2 String and Str
  - ch3.3 Enum
  - ch3.4 Struct
  - ch3.5 Ownership and Struct
  - ch3.6 Box
- Chapter4 Process Control and Function
  - ch4.1 If and Match
  - ch4.2 Loop and Iter
  - ch4.3 Functions
  - ch4.4 Move nad Borrow
  - ch4.5 Return Values
- Chapter5 Error
  - ch5.1 Error Basic
  - ch5.2 Question and Unwrap
  - ch5.3 Custom Error
- Chapter6 Borrow and Lifetime
  - ch6.1 Borrow Checker
  - ch6.2 Lifetime of Function
  - ch6.3 Lifetime of Struct
- Chapter7 Generic
  - ch7.1 Generic Struct
  - ch7.2 Generic Function
- Chapter8 Trait
  - ch8.1 Trait Basic
  - ch8.2 Trait Object and Box
  - ch8.3 Trait and Generic
  - ch8.4 Overload Operator
  - ch8.5 Polymorphic and Inherit
  - ch8.6 Common Trait
- Chapter9 Iterator
  - ch9.1 Iterator Loop
  - ch9.2 Intointerator, Iterator and Iter
  - ch9.3 Receive Iterator
  - ch9.4 Custom Iter
- Chapter10 Closure
  - ch10.1 Closure Basic
  - ch10.2 Closure Function
  - ch10.3 Closure Trait
  - ch10.4 Closure Example

# Getting Started

## Install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## rustup (rust)

Update:

```
rustup update
```

Uninstall:

```
rustup self uninstall
```

Add components:

```
rustup component add rustfmt
```

Check version:

```
rustup --version
```

Install and switch version:

```
rustup install stable/nightly
rustup default stable/nightly
```

## rustc (compiler)

Check version:

```
rustc --version
```

Compile and generate binary files:

```
rustc -o output_filename filename.rs
```

Compile and generate library files:

```
rustc --crate-type lib filename.rs
```

## cargo (package management tool)

Create project:

```
cargo new project_name
```

Create library project:

```
cargo new --lib project_name
```

Build project:

```
cargo build
```

Build optimized project:

```
cargo build --release
```

Check error:

```
cargo check
```

Run:

```
cargo run
```

Test:

```
cargo test
```

## Development environment

### Visual Studio Code

Plug-ins:

- rust-analyzer
- Error Lens
