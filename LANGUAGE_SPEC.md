# Axiom Language Specification v1.0

## Table of Contents

1. [Introduction](#introduction)
2. [Design Philosophy](#design-philosophy)
3. [Lexical Structure](#lexical-structure)
4. [Types System](#types-system)
5. [Variables and Bindings](#variables-and-bindings)
6. [Expressions and Operators](#expressions-and-operators)
7. [Statements and Control Flow](#statements-and-control-flow)
8. [Functions](#functions)
9. [Structs and Data Types](#structs-and-data-types)
10. [Interfaces and Polymorphism](#interfaces-and-polymorphism)
11. [Memory Management](#memory-management)
12. [Concurrency Model](#concurrency-model)
13. [Error Handling](#error-handling)
14. [Modules and Packages](#modules-and-packages)
15. [Generics](#generics)
16. [Metaprogramming](#metaprogramming)
17. [Unsafe Operations](#unsafe-operations)
18. [Foreign Function Interface](#foreign-function-interface)
19. [Compilation Model](#compilation-model)
20. [Standard Library Overview](#standard-library-overview)

---

## Introduction

### What is Axiom?

Axiom is a modern, low-level, high-performance programming language designed to combine the simplicity and developer experience of Python and Go with the raw power and control of C++. The language is built on a fundamental principle: **simple frontend, complex backend**. This means that developers write clean, readable code that feels natural and intuitive, while the compiler handles sophisticated optimizations, memory management, and code generation to produce highly efficient machine code.

The name "Axiom" reflects the language's foundational approach to programming—providing fundamental, self-evident constructs that developers can trust to work correctly and efficiently. Just as axioms form the basis of mathematical reasoning, Axiom's core constructs form the foundation for building reliable, high-performance software systems.

### Key Features

Axiom provides a unique combination of features that distinguish it from other programming languages. First and foremost, it offers Python-like readability with minimal syntax noise, allowing developers to express complex ideas concisely without sacrificing clarity. The language features type inference that reduces verbosity while maintaining full type safety at compile time. Unlike many modern languages that rely on garbage collection, Axiom provides deterministic memory management through ownership semantics, giving developers predictable performance characteristics.

The concurrency model in Axiom is built around lightweight tasks and message passing, inspired by languages like Erlang and Go, but with zero-cost abstractions that make concurrent programming both safe and efficient. The language also features a powerful compile-time metaprogramming system that enables developers to write generic, reusable code without runtime overhead.

### Target Audience

Axiom is designed for developers who need both performance and productivity. This includes systems programmers building operating systems, game engines, and embedded systems; backend engineers developing high-throughput services; and performance-critical application developers who refuse to compromise on either developer experience or runtime efficiency. The language is particularly well-suited for teams that want to write maintainable code without sacrificing the ability to optimize critical paths.

### Comparison with Other Languages

When compared to C++, Axiom offers significantly improved developer experience through cleaner syntax, better error messages, and safer defaults, while still providing access to low-level operations when needed. Compared to Rust, Axiom has a gentler learning curve with simpler ownership rules and more intuitive syntax. Compared to Go, Axiom provides better performance characteristics through its advanced compilation strategy and more expressive type system. Compared to Python, Axiom offers compiled performance and type safety while maintaining similar levels of readability.

---

## Design Philosophy

### Core Principles

The design of Axiom is guided by several core principles that inform every aspect of the language. These principles are not merely guidelines but fundamental decisions that shape how developers interact with the language and what kind of code they can write.

**Simplicity First** means that every feature added to the language must justify its complexity. Axiom prefers a small set of orthogonal features that combine well over a large set of specialized features. The language avoids syntactic sugar that obscures underlying semantics, preferring explicit constructs that make code behavior clear and predictable.

**Zero-Cost Abstractions** ensure that high-level programming constructs compile down to efficient machine code. Developers should not have to choose between writing clean code and achieving good performance. Every abstraction in Axiom is designed to be fully resolved at compile time, resulting in no runtime overhead compared to hand-written low-level code.

**Safety by Default** means that safe operations are the default, and unsafe operations require explicit opt-in. Memory safety, type safety, and thread safety are guaranteed for all code that does not use explicit unsafe blocks. This design allows developers to reason about their code with confidence while still providing escape hatches for performance-critical or low-level operations.

**Explicit Over Implicit** ensures that important decisions are visible in the code. While Axiom uses type inference to reduce verbosity, side effects, mutations, and resource management are always explicit. This design makes code easier to understand and reason about, particularly in large codebases worked on by multiple developers.

**Practical Performance** means that Axiom is designed to produce code that runs fast in real-world scenarios, not just in benchmarks. The language provides developers with the tools they need to understand and optimize performance, including detailed compile-time feedback and runtime profiling support.

### The Simple Frontend, Complex Backend Paradigm

Axiom's most distinctive characteristic is its separation of the frontend (what developers write) from the backend (what the compiler produces). The frontend is deliberately simple and consistent, with a grammar that can be learned in an afternoon. The backend, however, is sophisticated and complex, incorporating advanced optimization passes, intelligent memory layout decisions, and aggressive inlining.

This separation allows Axiom to evolve its optimization strategies without requiring developers to rewrite their code. As the compiler improves, existing Axiom programs automatically become faster. It also means that developers can focus on expressing their intent clearly rather than manually optimizing every critical path.

Consider a simple example of summing an array. In Axiom, you would write:

```axiom
fn sum(arr: []i32) -> i32 {
    var total: i32 = 0
    for x in arr {
        total += x
    }
    return total
}
```

This clean, readable code compiles to highly optimized machine code that uses SIMD instructions when available, unrolls loops when beneficial, and eliminates bounds checks when safety analysis proves them unnecessary. The developer writes simple code; the compiler produces complex, optimized output.

---

## Lexical Structure

### Source File Organization

Axiom source files use the `.ax` extension. Each source file is encoded in UTF-8 and consists of a sequence of tokens separated by whitespace and comments. Files are organized into modules, and the module hierarchy corresponds to the file system directory structure.

A source file typically begins with a module declaration, followed by import statements, and then declarations. The module declaration is optional at the root level; if omitted, the module name is derived from the file path.

```axiom
// module declaration (optional for main package)
module myapp.utils

// imports
import std.collections
import std.io

// declarations
fn helper() -> void {
    // implementation
}
```

### Whitespace and Formatting

Axiom treats spaces, tabs, and newlines as token separators. The language does not use whitespace for semantic purposes (unlike Python), allowing developers to format their code according to their preferences. However, the standard library includes a code formatter that enforces consistent style across projects.

### Comments

Axiom supports three types of comments: line comments, block comments, and documentation comments. Line comments begin with `//` and continue to the end of the line. Block comments begin with `/*` and end with `*/`, and can span multiple lines. Block comments can be nested, allowing developers to comment out code that already contains block comments.

Documentation comments begin with `///` for single-line documentation or `/**` for multi-line documentation. These comments are attached to the immediately following declaration and can be extracted by documentation tools.

```axiom
// This is a line comment

/* This is a block comment
   that spans multiple lines */

/// This is a documentation comment
/// that documents the following function
fn documented_function() -> void { }

/**
 * This is a multi-line documentation comment.
 * It provides detailed information about the function.
 * @param x The input value
 * @return The result of the computation
 */
fn complex_function(x: i32) -> i32 { x * 2 }
```

### Identifiers

Identifiers in Axiom must begin with a letter or underscore, followed by any combination of letters, digits, or underscores. The language supports Unicode identifiers, allowing developers to use non-ASCII characters in identifier names. However, the standard naming conventions recommend using ASCII identifiers for consistency.

```axiom
// Valid identifiers
let myVariable = 10
let _private = 20
let 名前 = "name"  // Unicode identifier

// Naming conventions
// Types: PascalCase
// Functions: snake_case
// Variables: snake_case
// Constants: SCREAMING_SNAKE_CASE
// Private members: _leading_underscore
```

### Keywords

Axiom reserves the following keywords for language constructs. These words cannot be used as identifiers unless escaped with a leading underscore.

```
as          async       await       break       const       continue
defer       else        enum        export      extern      fn
for         if          impl        import      in          let
match       mod         move        mut         pub         return
struct      self        Self        static      super       trait
true        false       type        union       unsafe      use
var         where       while       yield       loop
```

### Literals

Axiom supports various literal forms for different types of data.

**Integer literals** can be written in decimal, hexadecimal, octal, or binary notation. Underscores can be used as separators for readability.

```axiom
let decimal = 1_000_000
let hex = 0xFF00FF
let octal = 0o755
let binary = 0b1010_1010
```

**Floating-point literals** use standard decimal notation with optional exponent parts.

```axiom
let pi = 3.14159
let scientific = 1.5e10
let precise = 2.718_281_828
```

**String literals** are enclosed in double quotes and support escape sequences. Raw string literals use backticks and do not process escape sequences.

```axiom
let standard = "Hello, World!\n"
let raw = `C:\path\to\file.txt`
let multiline = `
    This string
    spans multiple lines
`
```

**Character literals** are enclosed in single quotes and represent a single Unicode code point.

```axiom
let letter = 'A'
let emoji = '🎉'
let newline = '\n'
```

**Boolean literals** are `true` and `false`.

```axiom
let yes = true
let no = false
```

**Null literal** is `null`, representing the absence of a value for nullable types.

```axiom
let maybe: ?i32 = null
```

### Operators and Punctuation

Axiom uses a rich set of operators and punctuation symbols:

```
+    -    *    /    %    **   //
==   !=   <    >    <=   >=   <=>
&&   ||   !    &    |    ^    ~
<<   >>   >>>  
=    +=   -=   *=   /=   %=   **=
&=   |=   ^=   <<=  >>=  >>>=
->   =>   ..   ...  ..=  ::
?    ??   ?.   !.
(    )    [    ]    {    }
,    ;    :    .    _    @
```

---

## Types System

### Type Categories

Axiom's type system is divided into several categories: primitive types, composite types, reference types, function types, and special types. The type system is statically typed with full type inference, meaning types are known at compile time but do not always need to be explicitly written.

### Primitive Types

Axiom provides a comprehensive set of primitive types that map directly to hardware representations.

**Integer Types** are available in signed and unsigned variants with explicit sizes:

| Type | Size | Range |
|------|------|-------|
| i8 | 8 bits | -128 to 127 |
| i16 | 16 bits | -32,768 to 32,767 |
| i32 | 32 bits | -2^31 to 2^31-1 |
| i64 | 64 bits | -2^63 to 2^63-1 |
| i128 | 128 bits | -2^127 to 2^127-1 |
| isize | pointer size | platform-dependent |
| u8 | 8 bits | 0 to 255 |
| u16 | 16 bits | 0 to 65,535 |
| u32 | 32 bits | 0 to 2^32-1 |
| u64 | 64 bits | 0 to 2^64-1 |
| u128 | 128 bits | 0 to 2^128-1 |
| usize | pointer size | platform-dependent |

**Floating-Point Types** follow IEEE 754 standards:

| Type | Size | Precision |
|------|------|-----------|
| f32 | 32 bits | ~7 decimal digits |
| f64 | 64 bits | ~15 decimal digits |

**Other Primitive Types**:

- `bool`: Boolean type with values `true` and `false`
- `char`: Unicode code point (4 bytes)
- `byte`: Alias for `u8`, used for raw data

### Type Aliases

Axiom allows creating type aliases for readability and documentation:

```axiom
type UserId = u64
type Point = (f64, f64)
type Callback = fn(i32) -> void
```

### Composite Types

**Arrays** are fixed-size collections of elements of the same type:

```axiom
let fixed: [i32; 5] = [1, 2, 3, 4, 5]
let inferred = [1, 2, 3]  // Type: [i32; 3]
let zeros: [i32; 100] = [0; 100]  // Array of 100 zeros
```

**Slices** are views into arrays with dynamic length:

```axiom
fn print_slice(s: []i32) {
    for x in s {
        println(x)
    }
}

let arr = [1, 2, 3, 4, 5]
print_slice(arr[1..4])  // Elements 2, 3, 4
```

**Tuples** group values of different types:

```axiom
let point: (f64, f64) = (1.0, 2.0)
let named = (x: 10, y: 20, name: "point")
let first = point.0
let x = named.x
```

**Structs** are user-defined types with named fields:

```axiom
struct Point {
    x: f64
    y: f64
}

let origin = Point { x: 0.0, y: 0.0 }
let moved = Point { x: 1.0, ..origin }  // Partial initialization
```

### Reference Types

**References** provide borrowed access to values:

```axiom
let x = 10
let ref_x: &i32 = &x       // Immutable reference
let mut_ref: &mut i32 = &mut x  // Mutable reference
```

**Pointers** provide raw memory access (unsafe):

```axiom
let raw: *i32 = &raw x  // Raw pointer
unsafe {
    *raw = 20  // Dereference requires unsafe block
}
```

### Optional Types

Optional types represent values that may or may not be present:

```axiom
let maybe: ?i32 = some(42)
let nothing: ?i32 = null

// Safe unwrapping
if let value = maybe {
    println(value)
}

// Default value
let value = maybe ?? 0

// Chaining
let result = maybe?.abs()?.to_string()
```

### Function Types

Functions have types that can be stored and passed around:

```axiom
type BinaryOp = fn(i32, i32) -> i32

fn add(a: i32, b: i32) -> i32 { a + b }
fn sub(a: i32, b: i32) -> i32 { a - b }

let op: BinaryOp = add
let result = op(10, 5)  // 15
```

### Type Inference

Axiom's type inference engine deduces types from context, reducing verbosity while maintaining full type safety:

```axiom
// Types are inferred
let x = 10           // i32
let y = 10.0         // f64
let z = x + 5        // i32
let arr = [1, 2, 3]  // [i32; 3]

// Generic function instantiation
let nums = [1, 2, 3, 4, 5]
let doubled = nums.map(|x| x * 2)  // Type inferred
```

---

## Variables and Bindings

### Variable Declarations

Axiom provides three ways to declare bindings, each with different semantics:

**Immutable bindings** use the `let` keyword and cannot be reassigned:

```axiom
let x = 10
let y: i32 = 20
// x = 30  // Error: cannot assign to immutable variable
```

**Mutable bindings** use the `var` keyword and can be reassigned:

```axiom
var counter = 0
counter = counter + 1  // OK
```

**Constants** are computed at compile time and use the `const` keyword:

```axiom
const MAX_SIZE: usize = 1024
const PI: f64 = 3.14159
const GREETING = "Hello, World!"
```

### Scope and Shadowing

Variables are scoped to the block in which they are declared. Axiom allows shadowing, where a new variable can have the same name as a variable in an outer scope:

```axiom
let x = 10
{
    let x = "hello"  // Shadows outer x
    println(x)       // "hello"
}
println(x)           // 10

// Shadowing in same scope
let y = 10
let y = y + 1        // New binding, old y still used for initialization
```

### Destructuring

Axiom supports destructuring declarations for tuples, structs, and arrays:

```axiom
// Tuple destructuring
let (a, b) = (1, 2)
let (first, rest) = tuple

// Struct destructuring
let Point { x, y } = point
let Point { x: px, y: py } = point  // Rename fields

// Array destructuring
let [first, second, ..] = [1, 2, 3, 4, 5]

// Ignoring values
let (important, _) = (10, "ignored")
```

### Ownership and Moves

Axiom uses an ownership system to manage memory without garbage collection. Each value has a single owner, and ownership can be transferred:

```axiom
let s1 = String::from("hello")
let s2 = s1  // s1 is moved to s2
// println(s1)  // Error: s1 was moved

// Clone for explicit copy
let s3 = s2.clone()  // Both s2 and s3 are valid
```

Types that implement the `Copy` trait are copied instead of moved:

```axiom
let x = 10
let y = x  // i32 is Copy, so x is still valid
println(x)  // OK
```

---

## Expressions and Operators

### Expression-Oriented Design

Axiom is expression-oriented, meaning most constructs produce values. This design enables concise and expressive code:

```axiom
// If is an expression
let max = if a > b { a } else { b }

// Blocks are expressions
let result = {
    let temp = compute()
    temp * 2
}

// Match is an expression
let description = match value {
    0 => "zero",
    1 => "one",
    _ => "many"
}
```

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| + | Addition | a + b |
| - | Subtraction | a - b |
| * | Multiplication | a * b |
| / | Division | a / b |
| % | Modulo | a % b |
| ** | Power | a ** b |
| // | Integer division | a // b |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| == | Equal | a == b |
| != | Not equal | a != b |
| < | Less than | a < b |
| > | Greater than | a > b |
| <= | Less or equal | a <= b |
| >= | Greater or equal | a >= b |
| <=> | Three-way comparison | a <=> b |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| && | Logical AND | a && b |
| \|\| | Logical OR | a \|\| b |
| ! | Logical NOT | !a |

### Bitwise Operators

| Operator | Description | Example |
|----------|-------------|---------|
| & | Bitwise AND | a & b |
| \| | Bitwise OR | a \| b |
| ^ | Bitwise XOR | a ^ b |
| ~ | Bitwise NOT | ~a |
| << | Left shift | a << n |
| >> | Right shift (signed) | a >> n |
| >>> | Right shift (unsigned) | a >>> n |

### Assignment Operators

```axiom
var x = 10
x += 5   // x = x + 5
x -= 3   // x = x - 3
x *= 2   // x = x * 2
x /= 4   // x = x / 4
x %= 3   // x = x % 3
x **= 2  // x = x ** 2
x <<= 1  // x = x << 1
x >>= 1  // x = x >> 1
x &= 0xFF  // x = x & 0xFF
x |= 0x0F  // x = x | 0x0F
x ^= 0x55  // x = x ^ 0x55
```

### Operator Precedence

From highest to lowest:

1. Postfix: `.`, `?`, `!`, `[]`, `()`
2. Prefix: `!`, `-`, `~`, `*`, `&`, `move`
3. Power: `**`
4. Multiplicative: `*`, `/`, `%`, `//`
5. Additive: `+`, `-`
6. Shift: `<<`, `>>`, `>>>`
7. Bitwise AND: `&`
8. Bitwise XOR: `^`
9. Bitwise OR: `|`
10. Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`, `<=>`
11. Logical AND: `&&`
12. Logical OR: `||`
13. Range: `..`, `..=`
14. Assignment: `=`, `+=`, `-=`, etc.

---

## Statements and Control Flow

### Expression Statements

Expressions can be used as statements by adding a semicolon:

```axiom
compute();  // Expression statement
x + 1;      // Result discarded
```

### Block Statements

Blocks group multiple statements and can contain declarations:

```axiom
{
    let temp = 10
    var result = temp * 2
    result += 5
    result  // Final expression is the block's value
}
```

### If Statements

Axiom's `if` can be used as a statement or expression:

```axiom
// Statement form
if condition {
    do_something()
} else if other_condition {
    do_other()
} else {
    do_default()
}

// Expression form
let value = if x > 0 {
    "positive"
} else if x < 0 {
    "negative"
} else {
    "zero"
}
```

### Pattern Matching

The `match` expression provides powerful pattern matching:

```axiom
match value {
    0 => println("zero"),
    1 | 2 | 3 => println("small"),
    4..=10 => println("medium"),
    n if n > 100 => println("large: {}", n),
    _ => println("other")
}

// Struct matching
match point {
    Point { x: 0, y: 0 } => "origin",
    Point { x: 0, y } => "on y-axis",
    Point { x, y: 0 } => "on x-axis",
    Point { x, y } => "at ({}, {})"
}

// Enum matching
match option {
    Some(value) => process(value),
    None => handle_none()
}
```

### Loops

**Infinite loops** use the `loop` keyword:

```axiom
var i = 0
loop {
    println(i)
    i += 1
    if i >= 10 {
        break
    }
}
```

**While loops** continue while a condition is true:

```axiom
var i = 0
while i < 10 {
    println(i)
    i += 1
}
```

**For loops** iterate over ranges and collections:

```axiom
// Range iteration
for i in 0..10 {
    println(i)
}

// Inclusive range
for i in 1..=10 {
    println(i)
}

// Collection iteration
for item in collection {
    println(item)
}

// With index
for (index, item) in collection.iter().enumerate() {
    println("{}: {}", index, item)
}
```

**Loop control** with `break` and `continue`:

```axiom
for i in 0..100 {
    if i % 2 == 0 {
        continue  // Skip even numbers
    }
    if i > 50 {
        break  // Exit loop
    }
    println(i)
}

// Labeled loops
'outer: for i in 0..10 {
    for j in 0..10 {
        if i + j > 15 {
            break 'outer  // Break outer loop
        }
    }
}
```

### Defer Statements

The `defer` keyword schedules cleanup operations:

```axiom
fn process_file(path: string) -> Result!void {
    let file = try File::open(path)
    defer file.close()  // Guaranteed to run on function exit
    
    // Process file
    try file.write(data)
    // file.close() called automatically
}
```

---

## Functions

### Function Declarations

Functions are declared with the `fn` keyword:

```axiom
fn greet(name: string) -> void {
    println("Hello, {}!", name)
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // Implicit return for single expression
}

fn complex(x: i32, y: i32) -> i32 {
    let temp = x * y
    temp + x  // Last expression is returned
}
```

### Parameters

**Default parameters** allow omitting arguments:

```axiom
fn greet(name: string, greeting: string = "Hello") -> void {
    println("{}, {}!", greeting, name)
}

greet("World")           // "Hello, World!"
greet("World", "Hi")     // "Hi, World!"
```

**Variadic functions** accept variable arguments:

```axiom
fn sum(values: ...i32) -> i32 {
    var total = 0
    for v in values {
        total += v
    }
    total
}

let result = sum(1, 2, 3, 4, 5)
```

**Named arguments** improve readability:

```axiom
fn create_window(width: u32, height: u32, title: string) -> Window {
    // ...
}

let win = create_window(
    width: 800,
    height: 600,
    title: "My Window"
)
```

### Closures

Closures capture their environment:

```axiom
let x = 10
let add_x = |y| x + y  // Captures x
println(add_x(5))      // 15

// Mutable capture
var counter = 0
let increment = || {
    counter += 1
    counter
}
println(increment())  // 1
println(increment())  // 2

// Move closure
let data = vec![1, 2, 3]
let process = move || {
    // data is moved into the closure
    data.iter().sum()
}
```

### Function Overloading

Axiom supports function overloading based on parameter types:

```axiom
fn process(value: i32) -> string {
    "integer"
}

fn process(value: string) -> string {
    "string"
}

fn process(value: bool) -> string {
    "boolean"
}
```

### Generic Functions

Functions can be generic over types:

```axiom
fn identity<T>(value: T) -> T {
    value
}

fn first<T>(arr: []T) -> ?T {
    if arr.len() > 0 {
        some(arr[0])
    } else {
        null
    }
}

// With constraints
fn compare<T: Ord>(a: T, b: T) -> Ordering {
    a.cmp(b)
}
```

---

## Structs and Data Types

### Struct Definitions

Structs group related data:

```axiom
struct Point {
    x: f64
    y: f64
}

struct Person {
    name: string
    age: u32
    email: string
}

// Tuple struct
struct Color(u8, u8, u8)

// Unit struct
struct Marker
```

### Methods

Methods are functions associated with types:

```axiom
struct Rectangle {
    width: f64
    height: f64
}

impl Rectangle {
    // Constructor
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    
    // Instance method
    fn area(self: &Self) -> f64 {
        self.width * self.height
    }
    
    // Mutable method
    fn scale(self: &mut Self, factor: f64) {
        self.width *= factor
        self.height *= factor
    }
    
    // Associated function (static method)
    fn square(size: f64) -> Self {
        Self::new(size, size)
    }
}
```

### Enums

Enums define types with multiple variants:

```axiom
enum Option<T> {
    Some(T)
    None
}

enum Result<T, E> {
    Ok(T)
    Err(E)
}

enum Message {
    Quit
    Move { x: i32, y: i32 }
    Write(string)
    ChangeColor(u8, u8, u8)
}
```

### Pattern Matching with Enums

```axiom
fn process_result<T, E>(result: Result<T, E>) -> T!E {
    match result {
        Ok(value) => value,
        Err(e) => throw e
    }
}

fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println("Quit"),
        Message::Move { x, y } => println("Move to ({}, {})", x, y),
        Message::Write(text) => println("Write: {}", text),
        Message::ChangeColor(r, g, b) => println("Color: ({}, {}, {})", r, g, b)
    }
}
```

---

## Interfaces and Polymorphism

### Trait Definitions

Traits define shared behavior:

```axiom
trait Drawable {
    fn draw(self: &Self) -> void
    fn bounding_box(self: &Self) -> Rectangle
}

trait Compare {
    fn cmp(self: &Self, other: &Self) -> Ordering
    
    fn lt(self: &Self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Less
    }
    
    fn gt(self: &Self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Greater
    }
}
```

### Implementing Traits

```axiom
struct Circle {
    center: Point
    radius: f64
}

impl Drawable for Circle {
    fn draw(self: &Self) -> void {
        // Draw circle implementation
    }
    
    fn bounding_box(self: &Self) -> Rectangle {
        Rectangle::new(
            self.center.x - self.radius,
            self.center.y - self.radius,
            self.radius * 2,
            self.radius * 2
        )
    }
}
```

### Trait Objects

Dynamic dispatch through trait objects:

```axiom
fn render(shapes: []&dyn Drawable) {
    for shape in shapes {
        shape.draw()
    }
}

let circle = Circle::new(Point::new(0.0, 0.0), 5.0)
let rect = Rectangle::new(0.0, 0.0, 10.0, 20.0)

let shapes: [&dyn Drawable; 2] = [&circle, &rect]
render(shapes)
```

---

## Memory Management

### Ownership System

Axiom uses an ownership-based memory management system without garbage collection:

```axiom
// Ownership rules:
// 1. Each value has exactly one owner
// 2. When the owner goes out of scope, the value is dropped
// 3. Ownership can be transferred (moved) or borrowed

let s1 = String::from("hello")
let s2 = s1  // Ownership transferred
// s1 is no longer valid

let s3 = String::from("world")
let s4 = s3.clone()  // Explicit copy, both valid
```

### Borrowing

References allow borrowing without taking ownership:

```axiom
fn calculate_length(s: &string) -> usize {
    s.len()
}  // s is a reference, nothing is dropped here

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

The compiler enforces these borrowing rules:

1. Any number of immutable references OR exactly one mutable reference
2. References must always be valid (no dangling references)
3. Mutable references cannot alias with any other references

```axiom
let mut s = String::from("hello")

// OK: multiple immutable borrows
let r1 = &s
let r2 = &s
println("{} {}", r1, r2)

// OK: mutable borrow after immutable borrows end
let r3 = &mut s
r3.push_str(" world")

// Error: cannot have mutable and immutable borrows simultaneously
// let r4 = &s
// let r5 = &mut s
```

### Lifetimes

Lifetime annotations ensure references are valid:

```axiom
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Lifetime elision - compiler infers in common cases
fn first_word(s: &str) -> &str {
    // ...
}
```

---

## Concurrency Model

### Tasks

Axiom uses lightweight tasks for concurrent execution:

```axiom
async fn fetch_data(url: string) -> Result!Data {
    let response = await http::get(url)
    try response.json()
}

async fn main() {
    let task1 = spawn fetch_data("https://api.example.com/data1")
    let task2 = spawn fetch_data("https://api.example.com/data2")
    
    let (result1, result2) = await (task1, task2)
    // Process results
}
```

### Channels

Message passing between tasks:

```axiom
use std::sync::channel

fn producer(tx: Sender<i32>) {
    for i in 0..10 {
        tx.send(i)
    }
}

fn consumer(rx: Receiver<i32>) {
    while let Some(value) = rx.recv() {
        println("Received: {}", value)
    }
}

fn main() {
    let (tx, rx) = channel::bounded(10)
    
    spawn producer(tx)
    consumer(rx)
}
```

### Synchronization Primitives

```axiom
use std::sync::{Mutex, RwLock, Atomic}

// Mutex for exclusive access
let counter = Mutex::new(0)
{
    let mut guard = counter.lock()
    *guard += 1
}

// RwLock for read-heavy workloads
let cache = RwLock::new(HashMap::new())
{
    let read_guard = cache.read()
    let value = read_guard.get("key")
}
{
    let mut write_guard = cache.write()
    write_guard.insert("key", "value")
}

// Atomic operations
let atomic_counter = Atomic::new(0i32)
atomic_counter.fetch_add(1)
```

---

## Error Handling

### Result Type

Axiom uses `Result` for recoverable errors:

```axiom
enum Result<T, E> {
    Ok(T)
    Err(E)
}

fn divide(a: i32, b: i32) -> Result<i32, string> {
    if b == 0 {
        return Err("division by zero")
    }
    Ok(a / b)
}

// Pattern matching
match divide(10, 2) {
    Ok(result) => println("Result: {}", result),
    Err(e) => println!("Error: {}", e)
}
```

### Error Propagation

The `try` keyword propagates errors:

```axiom
fn read_config(path: string) -> Result!Config {
    let content = try File::read_to_string(path)
    let config = try parse_config(content)
    Ok(config)
}

// Equivalent to:
fn read_config_verbose(path: string) -> Result!Config {
    let content = match File::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(e)
    }
    let config = match parse_config(content) {
        Ok(c) => c,
        Err(e) => return Err(e)
    }
    Ok(config)
}
```

### Custom Errors

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
            column: 1
        })
    }
}
```

---

## Modules and Packages

### Module System

```axiom
// math.ax
module math

pub const PI: f64 = 3.14159

pub fn square(x: f64) -> f64 {
    x * x
}

// Internal function
fn internal_helper() { }
```

```axiom
// main.ax
import math

fn main() {
    let area = math::PI * math::square(5.0)
    println("Area: {}", area)
}
```

### Visibility

```axiom
pub fn public_function() { }       // Public
fn private_function() { }          // Private (default)
pub(crate) fn crate_visible() { }  // Visible within crate
pub(super) fn parent_visible() { } // Visible in parent module
```

### Package Management

```toml
# Axiom.toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

---

## Generics

### Generic Types

```axiom
struct Container<T> {
    value: T
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
    
    fn get(self: &Self) -> &T {
        &self.value
    }
}
```

### Generic Constraints

```axiom
fn largest<T: Compare>(items: []T) -> ?T {
    if items.is_empty() {
        return null
    }
    
    var largest = items[0]
    for item in items[1..] {
        if item > largest {
            largest = item
        }
    }
    some(largest)
}

// Multiple constraints
fn process<T: Debug + Clone + Serialize>(value: T) {
    // ...
}

// Where clauses
fn complex_function<T, U>(t: T, u: U) -> string
where
    T: Display + Clone,
    U: Debug + Serialize
{
    // ...
}
```

---

## Metaprogramming

### Compile-Time Execution

```axiom
const fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

const FACTORIAL_10: u64 = factorial(10)  // Computed at compile time
```

### Macros

```axiom
// Declarative macros
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new()
            $(v.push($x))*
            v
        }
    }
}

let nums = vec![1, 2, 3, 4, 5]

// Procedural macros
#[derive(Debug, Clone, Serialize)]
struct Point {
    x: f64
    y: f64
}
```

---

## Unsafe Operations

### Unsafe Blocks

When low-level operations are necessary:

```axiom
unsafe fn dangerous_operation() {
    // Unsafe operations here
}

fn safe_wrapper() {
    unsafe {
        dangerous_operation()
    }
}
```

### Raw Pointers

```axiom
let mut x = 10
let raw_ptr: *mut i32 = &raw mut x

unsafe {
    *raw_ptr = 20
    println("Value: {}", *raw_ptr)
}
```

---

## Foreign Function Interface

### Calling C Functions

```axiom
extern "C" {
    fn printf(format: *const i8, ...) -> i32
    fn malloc(size: usize) -> *mut void
    fn free(ptr: *mut void)
}

fn call_c_function() {
    unsafe {
        printf(b"Hello from C!\n\0".as_ptr())
    }
}
```

### Exporting Functions

```axiom
#[export_name = "axiom_function"]
pub extern "C" fn exported_function(x: i32) -> i32 {
    x * 2
}
```

---

## Compilation Model

### Compilation Phases

1. **Lexing**: Source code → Tokens
2. **Parsing**: Tokens → Abstract Syntax Tree (AST)
3. **Semantic Analysis**: Type checking, borrow checking
4. **Lowering**: AST → Intermediate Representation (IR)
5. **Optimization**: IR transformations
6. **Code Generation**: IR → Machine code
7. **Linking**: Final executable

### Optimization Levels

```bash
# Debug build (fast compilation, no optimizations)
axiom build

# Release build (slow compilation, full optimizations)
axiom build --release

# Specific optimization level
axiom build -O2
```

---

## Standard Library Overview

### Core Types

- `Option<T>`: Optional values
- `Result<T, E>`: Error handling
- `Vec<T>`: Dynamic arrays
- `String`: UTF-8 strings
- `HashMap<K, V>`: Hash maps
- `HashSet<T>`: Hash sets

### I/O

- `std::io::File`: File operations
- `std::io::Stdin`, `std::io::Stdout`: Standard streams
- `std::net`: Networking

### Concurrency

- `std::sync::Mutex`, `std::sync::RwLock`: Synchronization
- `std::sync::channel`: Message passing
- `std::thread`: Thread management

---

*End of Language Specification v1.0*
