# Axiom Getting Started Guide

## Welcome to Axiom

Axiom is a modern programming language that combines the simplicity of Python and Go with the performance and power of C++. This guide will help you get started with Axiom, from installation to writing your first programs.

---

## Installation

### Prerequisites

Before installing Axiom, ensure you have the following prerequisites installed on your system:

**For all platforms:**
- A C compiler (gcc, clang, or MSVC)
- LLVM 17 or later
- CMake 3.20 or later
- Git

**Platform-specific requirements:**

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install build-essential cmake llvm-dev clang curl git
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install gcc cmake llvm-devel clang curl git
```

**macOS:**
```bash
xcode-select --install
brew install cmake llvm git
```

**Windows:**
- Visual Studio 2022 with C++ development tools
- LLVM from https://releases.llvm.org/
- Git from https://git-scm.com/

### Installing Axiom

**From Pre-built Binaries:**

The easiest way to install Axiom is from pre-built binaries:

```bash
# Linux/macOS
curl -fsSL https://axiom-lang.org/install.sh | sh

# Or manually
wget https://github.com/axiom-lang/axiom/releases/latest/download/axiom-linux-x64.tar.gz
tar -xzf axiom-linux-x64.tar.gz
sudo mv axiom /usr/local/bin/
```

**From Source:**

For the latest development version, build from source:

```bash
# Clone the repository
git clone https://github.com/axiom-lang/axiom.git
cd axiom

# Build the compiler
mkdir build && cd build
cmake ..
make -j$(nproc)

# Install
sudo make install
```

**Verify Installation:**

```bash
axiom --version
# axiom 1.0.0 (LLVM 17.0.0)
```

---

## Your First Axiom Program

### Hello, World!

Create a file named `hello.ax`:

```axiom
// hello.ax
fn main() {
    println("Hello, World!")
}
```

Run it:

```bash
axiom run hello.ax
# Hello, World!
```

Or compile it to an executable:

```bash
axiom build hello.ax -o hello
./hello
# Hello, World!
```

### Understanding the Code

Let's break down this simple program:

- `fn main()` - This declares the main function, which is the entry point of every Axiom program.
- `{ }` - Curly braces define a block of code.
- `println(...)` - This is a macro that prints a line to standard output.
- `"Hello, World!"` - A string literal.

---

## Language Basics

### Variables

Axiom provides three ways to declare variables:

```axiom
// Immutable variable (cannot be reassigned)
let x = 10
let name: string = "Axiom"

// Mutable variable (can be reassigned)
var counter = 0
counter = counter + 1

// Compile-time constant
const MAX_SIZE = 1024
```

### Types

Axiom has a rich type system with type inference:

```axiom
// Numeric types
let integer: i32 = 42
let big_int: i64 = 9223372036854775807
let decimal: f64 = 3.14159
let unsigned: u32 = 100

// Boolean
let flag: bool = true

// String
let message: string = "Hello"

// Arrays
let numbers = [1, 2, 3, 4, 5]
let explicit: [i32; 5] = [1, 2, 3, 4, 5]

// Tuples
let pair = (10, "hello")
let access = pair.0  // 10
```

### Functions

Functions are declared with the `fn` keyword:

```axiom
// Basic function
fn greet(name: string) {
    println("Hello, {}!", name)
}

// Function with return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // Implicit return (no semicolon for expression)
}

// Generic function
fn first<T>(arr: []T) -> ?T {
    if arr.len() > 0 {
        some(arr[0])
    } else {
        null
    }
}

// Function with default parameters
fn greet_with(name: string, greeting: string = "Hello") {
    println("{}, {}!", greeting, name)
}
```

### Control Flow

**If expressions:**

```axiom
let x = 10

// Statement form
if x > 0 {
    println("positive")
} else if x < 0 {
    println("negative")
} else {
    println("zero")
}

// Expression form (returns a value)
let abs = if x >= 0 { x } else { -x }
```

**Loops:**

```axiom
// Infinite loop
var i = 0
loop {
    println(i)
    i += 1
    if i >= 10 {
        break
    }
}

// While loop
var j = 0
while j < 10 {
    println(j)
    j += 1
}

// For loop
for k in 0..10 {
    println(k)  // Prints 0 through 9
}

// Iterating over collections
let fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    println(fruit)
}

// With index
for (idx, fruit) in fruits.iter().enumerate() {
    println("{}: {}", idx, fruit)
}
```

**Match expressions:**

```axiom
let value = 42

let description = match value {
    0 => "zero",
    1 | 2 | 3 => "small",
    4..=10 => "medium",
    n if n > 100 => "large",
    _ => "other"
}
```

### Structs

Define custom data types with structs:

```axiom
struct Point {
    x: f64
    y: f64
}

impl Point {
    // Constructor
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    // Method
    fn distance_from_origin(self: &Self) -> f64 {
        (self.x ** 2 + self.y ** 2).sqrt()
    }
}

let origin = Point::new(0.0, 0.0)
let point = Point { x: 3.0, y: 4.0 }
println("Distance: {}", point.distance_from_origin())  // 5.0
```

### Enums

Enums represent data with multiple variants:

```axiom
enum Option<T> {
    Some(T)
    None
}

enum Result<T, E> {
    Ok(T)
    Err(E)
}

fn divide(a: f64, b: f64) -> Result<f64, string> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println("Result: {}", result),
    Err(e) => println("Error: {}", e)
}
```

### Error Handling

Axiom uses the `try` keyword for error propagation:

```axiom
fn read_config(path: string) -> Result!Config {
    let content = try File::read_to_string(path)
    let config = try parse_config(content)
    Ok(config)
}

// Or use the ? operator
fn read_config_alt(path: string) -> Result!Config {
    let content = File::read_to_string(path)?
    let config = parse_config(content)?
    config
}
```

### Optionals

Handle potentially missing values:

```axiom
let maybe: ?i32 = some(42)
let nothing: ?i32 = null

// Safe unwrapping
if let value = maybe {
    println("Got: {}", value)
}

// Default value
let value = maybe ?? 0

// Chaining
let result = maybe?.abs()?.to_string()
```

---

## Project Structure

### Creating a New Project

Use the `axm` package manager to create a new project:

```bash
# Create a new project
axm new my_project

# Or initialize in an existing directory
mkdir my_project && cd my_project
axm init
```

This creates the following structure:

```
my_project/
├── Axiom.toml      # Project configuration
├── src/
│   └── main.ax     # Main source file
└── tests/
    └── test.ax     # Test file
```

### Axiom.toml Configuration

The `Axiom.toml` file contains project configuration:

```toml
[package]
name = "my_project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2024"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[features]
default = ["std"]
async = ["tokio"]
```

### Building and Running

```bash
# Build the project
axm build

# Build in release mode (with optimizations)
axm build --release

# Run the project
axm run

# Run tests
axm test

# Generate documentation
axm doc
```

---

## Memory Management

### Ownership

Axiom uses an ownership system instead of garbage collection:

```axiom
// Each value has one owner
let s1 = String::from("hello")
let s2 = s1  // Ownership moves from s1 to s2
// s1 is no longer valid

// Clone for explicit copy
let s3 = String::from("world")
let s4 = s3.clone()  // Both s3 and s4 are valid
```

### Borrowing

Borrow values with references:

```axiom
fn calculate_length(s: &string) -> usize {
    s.len()
}

let my_string = String::from("hello")
let len = calculate_length(&my_string)  // Borrow my_string
// my_string is still valid

// Mutable borrowing
fn append(s: &mut string, suffix: &str) {
    s.push_str(suffix)
}

let mut greeting = String::from("Hello")
append(&mut greeting, " World")
```

### Borrowing Rules

- Any number of immutable references OR exactly one mutable reference
- References must always be valid
- Mutable references cannot alias with any other references

---

## Concurrency

### Tasks

Axiom uses lightweight tasks for concurrent programming:

```axiom
async fn fetch_data(url: string) -> Result!string {
    let response = await http::get(url)
    try response.text()
}

async fn main() {
    // Spawn concurrent tasks
    let task1 = spawn fetch_data("https://api.example.com/1")
    let task2 = spawn fetch_data("https://api.example.com/2")
    
    // Wait for both to complete
    let (result1, result2) = await (task1, task2)
    
    println("Result 1: {}", result1)
    println("Result 2: {}", result2)
}
```

### Channels

Communicate between tasks using channels:

```axiom
use std::sync::channel

fn main() {
    let (tx, rx) = channel::bounded(10)
    
    // Producer task
    spawn {
        for i in 0..10 {
            tx.send(i)
        }
    }
    
    // Consumer
    while let Some(value) = rx.recv() {
        println("Received: {}", value)
    }
}
```

### Synchronization

Use mutex and other primitives:

```axiom
use std::sync::Mutex

fn main() {
    let counter = Mutex::new(0)
    
    // Spawn multiple tasks
    let mut handles = Vec::new()
    for _ in 0..10 {
        handles.push(spawn {
            let mut guard = counter.lock()
            *guard += 1
        })
    }
    
    // Wait for all tasks
    for handle in handles {
        handle.join()
    }
    
    println!("Counter: {}", *counter.lock())  // 10
}
```

---

## Modules

### Module Organization

Organize code into modules:

```axiom
// src/math/geometry.ax
module math.geometry

pub struct Point {
    x: f64
    y: f64
}

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x - p1.x) ** 2 + (p2.y - p1.y) ** 2).sqrt()
}

// Internal function (not public)
fn internal_helper() { }
```

### Imports

```axiom
// Import entire module
import math.geometry

// Selective import
import math.geometry::{Point, distance}

// Aliasing
import very.long.module.name as short

// Usage
let p1 = geometry::Point { x: 0.0, y: 0.0 }
let p2 = Point { x: 3.0, y: 4.0 }
let dist = distance(&p1, &p2)
```

---

## Testing

### Writing Tests

```axiom
// tests/test.ax
import my_project::calculator

#[test]
fn test_addition() {
    assert_eq!(calculator::add(2, 3), 5)
}

#[test]
fn test_subtraction() {
    assert_eq!(calculator::subtract(5, 3), 2)
}

#[test]
fn test_division() {
    match calculator::divide(10.0, 2.0) {
        Ok(result) => assert_eq!(result, 5.0),
        Err(e) => panic!("Unexpected error: {}", e)
    }
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_division_by_zero() {
    calculator::divide(10.0, 0.0).unwrap()
}
```

### Running Tests

```bash
# Run all tests
axm test

# Run specific test
axm test test_addition

# Run with verbose output
axm test --verbose
```

### Benchmarks

```axiom
#[bench]
fn bench_fibonacci(b: &mut Bencher) {
    b.iter(|| {
        fibonacci(30)
    })
}
```

---

## Tooling

### Code Formatting

Axiom includes a code formatter:

```bash
# Format all files
axfmt .

# Check formatting without modifying
axfmt --check .

# Format specific file
axfmt src/main.ax
```

### Linting

Use the linter to find common issues:

```bash
# Run linter
axclippy

# Run with specific lints
axclippy -W style -W correctness
```

### Language Server

Axiom provides a language server for IDE integration:

- **VS Code**: Install the "Axiom" extension
- **JetBrains IDEs**: Install the Axiom plugin
- **Vim/Neovim**: Use with nvim-lspconfig
- **Emacs**: Use with lsp-mode

Features:
- Code completion
- Go to definition
- Find references
- Rename symbol
- Inline error messages
- Code actions

---

## Next Steps

Now that you've learned the basics, here are some resources to continue your journey:

### Documentation

- [Language Specification](LANGUAGE_SPEC.md) - Complete language reference
- [Syntax Reference](SYNTAX_REFERENCE.md) - Quick syntax lookup
- [Standard Library](STDLIB_SPEC.md) - API documentation
- [Compiler Architecture](COMPILER_ARCHITECTURE.md) - How the compiler works

### Examples

The Axiom repository includes example projects:

```bash
# Clone examples
git clone https://github.com/axiom-lang/axiom-examples.git
cd axiom-examples

# Run an example
cd http-server
axm run
```

### Community

- **Discord**: https://discord.gg/axiom-lang
- **Forum**: https://forum.axiom-lang.org
- **GitHub**: https://github.com/axiom-lang/axiom
- **Twitter**: @AxiomLang

### Contributing

We welcome contributions! See [AGENTS.md](AGENTS.md) for guidelines on contributing to the project.

---

## Common Patterns

### Builder Pattern

```axiom
struct Request {
    url: string
    method: string
    headers: HashMap<string, string>
    body: ?string
}

impl Request {
    fn new(url: string) -> Self {
        Self {
            url,
            method: "GET",
            headers: HashMap::new(),
            body: null,
        }
    }
    
    fn method(self: Self, method: string) -> Self {
        Self { method, ..self }
    }
    
    fn header(self: Self, key: string, value: string) -> Self {
        let mut headers = self.headers.clone()
        headers.insert(key, value)
        Self { headers, ..self }
    }
    
    fn body(self: Self, body: string) -> Self {
        Self { body: some(body), ..self }
    }
}

let request = Request::new("https://api.example.com")
    .method("POST")
    .header("Content-Type", "application/json")
    .body("{\"name\": \"test\"}")
```

### Error Handling with Custom Errors

```axiom
struct ParseError {
    message: string
    line: usize
    column: usize
}

impl Error for ParseError {
    fn message(self: &Self) -> string {
        format!("Parse error at {}:{}: {}", self.line, self.column, self.message)
    }
}

fn parse_int(s: string) -> Result<i32, ParseError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(ParseError {
            message: "Invalid integer",
            line: 1,
            column: 1,
        })
    }
}
```

---

## Troubleshooting

### Common Issues

**"axiom: command not found"**
- Ensure Axiom is in your PATH
- Try `source ~/.bashrc` or restart your terminal

**"LLVM not found"**
- Install LLVM development packages
- Set `LLVM_SYS_170_PREFIX` environment variable

**Compilation errors with "cannot find module"**
- Run `axm update` to fetch dependencies
- Check import paths are correct

**Slow compilation in debug mode**
- Use `axm build --release` for release builds
- Consider using the Cranelift backend for faster debug builds

### Getting Help

If you encounter issues:

1. Check the [FAQ](https://axiom-lang.org/faq)
2. Search existing issues on GitHub
3. Ask on Discord or the forum
4. File a bug report if it's a new issue

---

*Welcome to Axiom! We're excited to see what you build.*
