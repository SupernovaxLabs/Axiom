# Axiom Programming Language

## Simple Frontend, Complex Backend

**Axiom** is a modern, low-level, high-performance programming language that combines the simplicity of **Python** and **Go** with the raw power of **C++**.

### Quick Example

```axiom
// Hello World in Axiom
fn main() {
    println("Hello, World!")
}

// Simple and readable
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Powerful like C++
struct Vec3 {
    x: f64
    y: f64
    z: f64
}

impl Vec3 {
    fn dot(self: &Self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    fn length(self: &Self) -> f64 {
        (self.x ** 2 + self.y ** 2 + self.z ** 2).sqrt()
    }
}
```

---


## Bootstrap Status (Rust-first)

The repository now includes a **Rust bootstrap workspace** that mirrors the planned Axiom compiler/runtime/std layout from the design docs.

- Bootstrap source files currently use `.rs` (not `.ax`)
- Workspace members: `compiler/`, `runtime/`, `std/`
- Includes module skeletons for lexer, parser, type checker, borrow checker, IR, optimizer, codegen, linker, driver, and utility layers
- Pre-created the broader documentation/test/tooling/examples tree from `docs/agent/PROJECT_STRUCTURE.md` with Rust bootstrap placeholders

Run the bootstrap compiler placeholder:

```bash
cargo run -p axiom-compiler
```

## Why Axiom?

| Feature | Description |
|---------|-------------|
| **Simple Syntax** | Clean, readable code like Python |
| **High Performance** | Compiled to native code like C++ |
| **Memory Safe** | Ownership system without garbage collection |
| **Modern Concurrency** | Async/await with lightweight tasks |
| **Zero-Cost Abstractions** | High-level code, low-level performance |
| **Fast Compilation** | Incremental builds, smart caching |

---

## Documentation Package

This package contains **14,500+ lines** of comprehensive documentation:

### Core Documentation

| File | Lines | Description |
|------|-------|-------------|
| `docs/agent/LANGUAGE_SPEC.md` | 1,578 | Complete language specification |
| `docs/agent/SYNTAX_REFERENCE.md` | 992 | Quick syntax reference |
| `docs/agent/STDLIB_SPEC.md` | 931 | Standard library API |
| `docs/agent/GETTING_STARTED.md` | 857 | Developer onboarding guide |
| `docs/agent/ROADMAP.md` | 1,033 | 3-4 year development plan |

### Technical Documentation

| File | Lines | Description |
|------|-------|-------------|
| `docs/agent/TECHNICAL_IMPLEMENTATION.md` | 3,288 | Compiler, lexer, parser implementation |
| `docs/agent/TECHNICAL_IMPLEMENTATION_PART2.md` | 1,798 | Type checker, borrow checker, IR |
| `docs/agent/TECHNICAL_IMPLEMENTATION_PART3.md` | 2,246 | Binary generation, linker, interpreter |
| `docs/agent/COMPILER_ARCHITECTURE.md` | 861 | Compiler architecture overview |

### Project Documentation

| File | Lines | Description |
|------|-------|-------------|
| `docs/agent/PROJECT_STRUCTURE.md` | 455 | Complete file organization |
| `docs/agent/AGENTS.md` | 489 | AI agent contribution guidelines |

---

## Key Design Decisions

### Simple Frontend

```
┌─────────────────────────────────────────┐
│           DEVELOPER WRITES               │
│                                          │
│   fn sum(arr: []i32) -> i32 {           │
│       var total = 0                      │
│       for x in arr {                     │
│           total += x                     │
│       }                                  │
│       return total                       │
│   }                                      │
│                                          │
│   Clean, readable, intuitive             │
└─────────────────────────────────────────┘
```

### Complex Backend

```
┌─────────────────────────────────────────┐
│           COMPILER PRODUCES              │
│                                          │
│   • SIMD vectorized loops                │
│   • Inlined function calls               │
│   • Optimized memory layout              │
│   • Dead code eliminated                 │
│   • Bounds checks removed                │
│   • Cache-friendly access patterns       │
│                                          │
│   Fast, efficient, optimized             │
└─────────────────────────────────────────┘
```

---

## Language Features

### Memory Management

```axiom
// Ownership system (no garbage collection)
let s1 = String::from("hello")
let s2 = s1           // Ownership transferred
// s1 is no longer valid

// Borrowing
fn calculate_length(s: &string) -> usize {
    s.len()
}

let text = String::from("hello")
let len = calculate_length(&text)  // Borrow
// text is still valid

// Mutable borrowing
fn append(s: &mut string, suffix: &str) {
    s.push_str(suffix)
}
```

### Concurrency

```axiom
// Async/await
async fn fetch(url: string) -> Result!Response {
    await http::get(url)
}

async fn main() {
    let results = await [
        fetch("https://api.example.com/1"),
        fetch("https://api.example.com/2"),
    ]
}

// Channels
let (tx, rx) = channel::bounded(10)

spawn {
    for i in 0..10 {
        tx.send(i)
    }
}

while let Some(value) = rx.recv() {
    println("Received: {}", value)
}
```

### Error Handling

```axiom
// Result type
fn divide(a: f64, b: f64) -> Result!f64 {
    if b == 0.0 {
        throw Error::division_by_zero()
    }
    Ok(a / b)
}

// Error propagation
fn process() -> Result!void {
    let config = try load_config()
    let data = try read_file(config.path)
    try process_data(data)
}
```

### Pattern Matching

```axiom
match value {
    0 => "zero",
    1 | 2 | 3 => "small",
    4..=10 => "medium",
    n if n > 100 => "large",
    _ => "other"
}

// Destructuring
let Point { x, y } = point
let (first, rest) = tuple
let [head, ..tail] = array
```

---

## Compiler Pipeline

```
Source Code (.ax)
       │
       ▼
┌──────────────┐
│    Lexer     │ ──► Tokens
└──────────────┘
       │
       ▼
┌──────────────┐
│    Parser    │ ──► AST
└──────────────┘
       │
       ▼
┌──────────────┐
│  Type Check  │ ──► Typed AST
│  Borrow Check│
└──────────────┘
       │
       ▼
┌──────────────┐
│   IR Gen     │ ──► AIR (SSA)
└──────────────┘
       │
       ▼
┌──────────────┐
│  Optimize    │ ──► Optimized IR
└──────────────┘
       │
       ▼
┌──────────────┐
│ Code Gen     │ ──► Object Files
└──────────────┘
       │
       ▼
┌──────────────┐
│    Linker    │ ──► Executable
└──────────────┘
```

---

## Project Structure

```
axiom/
├── compiler/           # Compiler implementation
│   ├── lexer/         # Lexical analysis
│   ├── parser/        # Parsing
│   ├── typeck/        # Type checking
│   ├── borrowck/      # Borrow checking
│   ├── air/           # Intermediate representation
│   ├── opt/           # Optimization passes
│   ├── codegen/       # Code generation
│   └── linker/        # Linking
│
├── runtime/           # Runtime library
├── std/               # Standard library
├── interpreter/       # REPL interpreter
├── tools/             # Development tools
│   ├── axm/          # Package manager
│   ├── axfmt/        # Formatter
│   ├── axclippy/     # Linter
│   └── axls/         # Language server
│
└── docs/              # Documentation
```

---

## Development Roadmap

| Phase | Duration | Focus |
|-------|----------|-------|
| 0 | Months 1-3 | Foundation & Planning |
| 1 | Months 4-6 | Lexer & Parser |
| 2 | Months 7-9 | Semantic Analysis |
| 3 | Months 10-12 | Code Generation |
| 4 | Months 13-18 | Standard Library |
| 5 | Months 19-21 | Package Manager |
| 6 | Months 22-24 | Tooling & IDE |
| 7 | Months 25-30 | Advanced Features |
| 8 | Months 31-36 | Optimization & 1.0 |

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Compilation (10K LOC) | < 1s debug, < 30s release |
| Generated code | Within 5% of C |
| Memory usage | Competitive with Rust |
| Binary size | Similar to Go |
| Startup time | < 1ms |

---

## Getting Started

### Installation

**Windows** — run this in PowerShell (no Rust or build tools required):

```powershell
iwr -useb https://raw.githubusercontent.com/SupernovaxLabs/Axiom/main/install.ps1 | iex
```

This downloads the latest pre-built `axiom-cli.exe` and `axiom-interpreter.exe` from [GitHub Releases](https://github.com/SupernovaxLabs/Axiom/releases/latest) and installs them to `%LOCALAPPDATA%\Axiom\bin`, then adds that directory to your user `PATH` automatically.

**Linux / macOS** — run this in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/SupernovaxLabs/Axiom/main/install.sh | bash
```

After installation, open a **new terminal** and verify:

```bash
axiom version
```

#### Install a specific version

```powershell
# Windows — install a specific release tag
iwr -useb https://raw.githubusercontent.com/SupernovaxLabs/Axiom/main/install.ps1 | iex
# or with a specific version:
& ([scriptblock]::Create((iwr -useb https://raw.githubusercontent.com/SupernovaxLabs/Axiom/main/install.ps1))) -Version build-5
```

```bash
# Linux/macOS — install a specific release tag
AXIOM_VERSION=build-5 curl -fsSL https://raw.githubusercontent.com/SupernovaxLabs/Axiom/main/install.sh | bash
```

---

### Running Axiom Scripts

Once installed, use the `axiom` command to run `.ax` files:

```bash
axiom run hello.ax
```

**Hello World** (`hello.ax`):

```axiom
println("Hello, Axiom!")
```

**Fibonacci** (`fib.ax`):

```axiom
fn fib(n) {
    if n <= 1 { return n; }
    return fib(n - 1) + fib(n - 2);
}

fib(10)
```

**Arrays and loops** (`arrays.ax`):

```axiom
var numbers = [1, 2, 3, 4, 5]
var sum = 0
var i = 0
while i < len(numbers) {
    sum += numbers[i]
    i += 1
}
sum
```

More examples are in the [`examples/ax/`](examples/ax/) directory.

---

## Community

- **GitHub**: https://github.com/SupernovaxLabs/Axiom
- **Releases**: https://github.com/SupernovaxLabs/Axiom/releases

---

## License

MIT OR Apache-2.0

---

*Built with ❤️ for developers who refuse to choose between productivity and performance.*
