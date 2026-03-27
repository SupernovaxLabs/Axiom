# Axiom Syntax Reference

## Quick Reference Card

This document provides a comprehensive syntax reference for the Axiom programming language. It is designed to be a quick lookup guide for developers already familiar with the language concepts.

---

## Program Structure

### Module Declaration

```axiom
module myapp.utils

import std.collections
import std.io::{File, Read}
import external_lib as ext

// Module contents...
```

### Entry Point

```axiom
fn main() {
    println("Hello, World!")
}

// Or with arguments
fn main(args: []string) {
    for arg in args {
        println(arg)
    }
}

// Or async
async fn main() {
    await do_async_work()
}
```

---

## Comments

```axiom
// Single-line comment

/* Multi-line
   comment */

/// Documentation comment
/// Supports multiple lines

/**
 * Detailed documentation
 * @param x The input value
 * @return The result
 */
fn documented(x: i32) -> i32 { x }
```

---

## Variables and Bindings

### Immutable Variables

```axiom
let x = 10                    // Type inferred
let y: i32 = 20               // Explicit type
let z: i64 = 30i64            // Type suffix
let (a, b) = (1, 2)           // Destructuring
```

### Mutable Variables

```axiom
var counter = 0
counter = counter + 1

var buffer: Vec<u8> = Vec::new()
buffer.push(255)
```

### Constants

```axiom
const MAX_SIZE: usize = 1024
const PI: f64 = 3.14159
const GREETING = "Hello"
```

### Static Variables

```axiom
static GLOBAL_COUNTER: AtomicI32 = AtomicI32::new(0)
static mut COUNTER: i32 = 0  // Unsafe to access
```

---

## Primitive Types

### Integers

```axiom
// Signed
let a: i8 = -128
let b: i16 = -32768
let c: i32 = -2147483648
let d: i64 = -9223372036854775808i64
let e: i128 = -170141183460469231731687303715884105728i128
let f: isize = -1is  // Pointer-sized

// Unsigned
let g: u8 = 255
let h: u16 = 65535
let i: u32 = 4294967295u32
let j: u64 = 18446744073709551615u64
let k: u128 = 340282366920938463463374607431768211455u128
let l: usize = 1us  // Pointer-sized
```

### Floating-Point

```axiom
let pi: f32 = 3.14159f32
let e: f64 = 2.718281828
let inf = f64::INFINITY
let nan = f64::NAN
```

### Other Primitives

```axiom
let flag: bool = true
let letter: char = 'A'
let emoji: char = '🎉'
let byte: byte = 0xFF
```

---

## Literals

### Integer Literals

```axiom
let decimal = 42
let hex = 0xFF
let octal = 0o755
let binary = 0b1010_1010
let separated = 1_000_000
```

### Float Literals

```axiom
let standard = 3.14
let scientific = 1.5e10
let typed = 2.5f32
```

### String Literals

```axiom
let standard = "Hello, World!\n"
let raw = `C:\path\to\file.txt`
let multiline = `
    Line 1
    Line 2
`
let bytes = b"byte string"
```

### Character Literals

```axiom
let letter = 'A'
let escape = '\n'
let unicode = '\u{1F389}'
```

---

## Composite Types

### Arrays

```axiom
let arr = [1, 2, 3]                    // Type: [i32; 3]
let explicit: [i32; 5] = [1, 2, 3, 4, 5]
let zeros = [0; 100]                   // 100 zeros
let nested = [[1, 2], [3, 4]]
```

### Slices

```axiom
let arr = [1, 2, 3, 4, 5]
let slice: &[i32] = &arr
let range = &arr[1..4]                 // [2, 3, 4]
let to_end = &arr[2..]                 // [3, 4, 5]
let from_start = &arr[..3]             // [1, 2, 3]
let full = &arr[..]                    // [1, 2, 3, 4, 5]
```

### Tuples

```axiom
let pair = (1, "hello")
let triple: (i32, f64, string) = (1, 2.0, "three")
let unit = ()
let access = pair.0                     // 1
let named = (x: 10, y: 20)
let x = named.x                         // 10
```

### Structs

```axiom
// Declaration
struct Point {
    x: f64
    y: f64
}

// Construction
let origin = Point { x: 0.0, y: 0.0 }
let moved = Point { x: 1.0, ..origin }  // Partial update
let inferred = Point { x: 1.0, y: 2.0 }

// Access
let x = origin.x
let Point { x, y } = origin            // Destructuring

// Tuple struct
struct Color(u8, u8, u8)
let red = Color(255, 0, 0)
let r = red.0

// Unit struct
struct Marker
```

### Enums

```axiom
// Simple enum
enum Direction {
    Up
    Down
    Left
    Right
}

let dir = Direction::Up

// Enum with data
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

let msg = Message::Move { x: 10, y: 20 }
```

---

## Functions

### Basic Functions

```axiom
fn greet() {
    println("Hello!")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn no_return() -> void {
    println("No return value")
}
```

### Parameters

```axiom
// Default parameters
fn greet(name: string, greeting: string = "Hello") {
    println("{}, {}!", greeting, name)
}

// Variadic
fn sum(values: ...i32) -> i32 {
    var total = 0
    for v in values { total += v }
    total
}

// Named arguments
fn create_window(width: u32, height: u32, title: string) -> Window { /* ... */ }
let win = create_window(width: 800, height: 600, title: "App")
```

### Generic Functions

```axiom
fn identity<T>(value: T) -> T {
    value
}

fn first<T>(arr: []T) -> ?T {
    if arr.len() > 0 { some(arr[0]) } else { null }
}

fn compare<T: Ord>(a: T, b: T) -> Ordering {
    a.cmp(b)
}

fn complex<T, U>(t: T, u: U) -> void
where
    T: Clone + Debug,
    U: Serialize
{
    // ...
}
```

### Closures

```axiom
let add = |a, b| a + b
let add_typed = |a: i32, b: i32| -> i32 { a + b }

let x = 10
let captures = || x + 1           // Captures x by reference
let moves = move || x + 1         // Moves x into closure

// As function parameter
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}
```

### Async Functions

```axiom
async fn fetch(url: string) -> Result!Response {
    await http::get(url)
}

async fn parallel() -> void {
    let results = await [
        fetch("url1"),
        fetch("url2"),
    ]
}
```

---

## Control Flow

### If Expressions

```axiom
// Statement
if x > 0 {
    println("positive")
} else if x < 0 {
    println("negative")
} else {
    println("zero")
}

// Expression
let abs = if x >= 0 { x } else { -x }

// With let
let result = if condition {
    compute_a()
} else {
    compute_b()
}
```

### Match Expressions

```axiom
match value {
    0 => "zero",
    1 | 2 | 3 => "small",
    4..=10 => "medium",
    n if n > 100 => "large",
    _ => "other"
}

// With bindings
match option {
    Some(x) => println("got {}", x),
    None => println("nothing")
}

// Struct patterns
match point {
    Point { x: 0, y: 0 } => "origin",
    Point { x: 0, y } => "y-axis",
    Point { x, y: 0 } => "x-axis",
    Point { x, y } => "point"
}

// Enum patterns
match message {
    Message::Quit => handle_quit(),
    Message::Move { x, y } => handle_move(x, y),
    Message::Write(text) => handle_write(text),
    Message::ChangeColor(r, g, b) => handle_color(r, g, b),
}
```

### Loops

```axiom
// Infinite loop
loop {
    if done { break }
    if should_skip { continue }
    do_work()
}

// While loop
while condition {
    do_work()
}

// For loop
for i in 0..10 {
    println(i)
}

for item in collection {
    println(item)
}

for (idx, item) in collection.iter().enumerate() {
    println("{}: {}", idx, item)
}

// Labeled loops
'outer: for i in 0..10 {
    for j in 0..10 {
        if condition { break 'outer }
    }
}

// Loop with value
let result = loop {
    let value = compute()
    if ready { break value }
}
```

### Range Expressions

```axiom
let exclusive = 0..10      // 0, 1, 2, ..., 9
let inclusive = 0..=10     // 0, 1, 2, ..., 10
let open_start = ..5       // ..., 4
let open_end = 5..         // 5, 6, ...
let full = ..              // All elements
```

---

## Operators

### Arithmetic

```axiom
let sum = a + b
let diff = a - b
let prod = a * b
let quot = a / b
let rem = a % b
let pow = a ** b
let int_div = a // b
```

### Comparison

```axiom
let eq = a == b
let ne = a != b
let lt = a < b
let gt = a > b
let le = a <= b
let ge = a >= b
let cmp = a <=> b    // Ordering::Less, Equal, or Greater
```

### Logical

```axiom
let and = a && b
let or = a || b
let not = !a
```

### Bitwise

```axiom
let band = a & b
let bor = a | b
let xor = a ^ b
let not = ~a
let shl = a << n
let shr = a >> n     // Arithmetic (sign-extending)
let ushr = a >>> n   // Logical (zero-filling)
```

### Assignment

```axiom
x = 10
x += 5
x -= 3
x *= 2
x /= 4
x %= 3
x **= 2
x &= mask
x |= flags
x ^= toggle
x <<= 1
x >>= 1
```

### Other

```axiom
// Range
let range = 0..10

// Dereference
let value = *reference

// Reference
let ref = &value
let mut_ref = &mut value

// Error propagation
let result = try fallible_operation()

// Null coalescing
let value = maybe ?? default

// Safe navigation
let name = person?.name?.first

// Type cast
let specific = general as SpecificType
```

---

## References and Pointers

### References

```axiom
let x = 10
let ref_x: &i32 = &x              // Immutable reference
let mut_ref: &mut i32 = &mut x    // Mutable reference

// Reference rules:
// - Many immutable refs OR one mutable ref
// - References must always be valid
```

### Raw Pointers

```axiom
let raw: *const i32 = &raw const x   // Immutable raw pointer
let raw_mut: *mut i32 = &raw mut x   // Mutable raw pointer

unsafe {
    let value = *raw
    *raw_mut = 20
}
```

---

## Option and Result

### Option

```axiom
let some_value: ?i32 = some(42)
let none_value: ?i32 = null

// Checking
if some_value.is_some() { /* ... */ }
if some_value.is_none() { /* ... */ }

// Unwrapping
let value = some_value.unwrap()           // Panics if None
let value = some_value.unwrap_or(0)       // Default value
let value = some_value.unwrap_or_else(|| compute_default())

// Chaining
let result = some_value?.abs()?.to_string()

// Pattern matching
match some_value {
    Some(v) => println(v),
    None => println("none")
}

// Conversion
let result: Result<i32, string> = some_value.ok_or("error")
```

### Result

```axiom
let ok_value: Result!i32 = Ok(42)
let err_value: Result!i32 = Err("failed")

// Propagation
fn read_file(path: string) -> Result!string {
    let content = try File::read_to_string(path)
    try validate(content)
    Ok(content)
}

// Handling
let value = result.unwrap()
let value = result.unwrap_or(default)
let value = result.unwrap_or_else(|e| handle_error(e))

// Pattern matching
match result {
    Ok(value) => handle_success(value),
    Err(e) => handle_error(e),
}
```

---

## Traits (Interfaces)

### Declaration

```axiom
trait Drawable {
    fn draw(self: &Self) -> void
    fn bounding_box(self: &Self) -> Rectangle
    
    // Default implementation
    fn contains(self: &Self, point: Point) -> bool {
        self.bounding_box().contains(point)
    }
}

trait Compare<T> {
    fn cmp(self: &Self, other: &T) -> Ordering
}

// Associated types
trait Iterator {
    type Item
    fn next(&mut self) -> ?Self::Item
}
```

### Implementation

```axiom
impl Drawable for Circle {
    fn draw(self: &Self) -> void {
        // Draw circle
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

// Generic implementation
impl<T: Clone> Clone for Vec<T> {
    fn clone(self: &Self) -> Self {
        // Clone implementation
    }
}
```

### Trait Bounds

```axiom
// On generic types
fn process<T: Debug + Clone>(value: T) { }

// Where clauses
fn complex<T, U>(t: T, u: U) -> void
where
    T: Debug + Clone,
    U: Serialize + Deserialize,
{
    // ...
}

// Trait objects
let drawable: &dyn Drawable = &circle
let drawables: Vec<&dyn Drawable> = vec![&circle, &rect]
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
}

impl<T: Clone> Container<T> {
    fn clone_value(self: &Self) -> T {
        self.value.clone()
    }
}
```

### Generic Functions

```axiom
fn identity<T>(value: T) -> T {
    value
}

fn first<T: Clone>(arr: []T) -> ?T {
    if arr.len() > 0 { some(arr[0].clone()) } else { null }
}
```

### Const Generics

```axiom
struct Array<T, const N: usize> {
    data: [T; N]
}

fn make_array<T, const N: usize>(value: T) -> Array<T, N>
where
    T: Copy,
{
    Array { data: [value; N] }
}

let arr: Array<i32, 10> = make_array(0)
```

---

## Modules and Imports

### Module Declaration

```axiom
// File: src/utils/math.ax
module utils.math

pub const PI: f64 = 3.14159

pub fn square(x: f64) -> f64 {
    x * x
}

// Internal (not public)
fn internal_helper() { }
```

### Imports

```axiom
// Import all
import std.collections

// Selective import
import std.io::{File, Read, Write}

// Aliasing
import very.long.module.name as short

// Re-export
pub import internal::helper
```

### Visibility

```axiom
pub fn public_function() { }           // Public
fn private_function() { }              // Private (default)
pub(crate) fn crate_visible() { }      // Crate-level
pub(super) fn parent_visible() { }     // Parent module
pub(in path) fn path_visible() { }     // Specific path
```

---

## Error Handling

### Try Operator

```axiom
fn read_config(path: string) -> Result!Config {
    let content = try File::read_to_string(path)
    let config = try parse(content)
    Ok(config)
}
```

### Throw Expression

```axiom
fn check_positive(x: i32) -> Result!void {
    if x < 0 {
        throw Error::negative_value()
    }
    Ok(())
}
```

### Defer

```axiom
fn process_file(path: string) -> Result!void {
    let file = try File::open(path)
    defer file.close()  // Guaranteed cleanup
    
    try process(&file)
    Ok(())
}
```

---

## Macros

### Declarative Macros

```axiom
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new()
            $(v.push($x))*
            v
        }
    }
}

let nums = vec![1, 2, 3]
```

### Procedural Macros

```axiom
#[derive(Debug, Clone)]
struct Point {
    x: f64
    y: f64
}

#[attribute_macro]
fn logged(item: TokenStream) -> TokenStream {
    // Transform the item
}

#[inline(always)]
fn hot_function() { }
```

---

## Unsafe

```axiom
unsafe fn dangerous() {
    // Unsafe operations
}

fn safe_wrapper() {
    unsafe {
        dangerous()
    }
}

// Unsafe block can do:
// - Dereference raw pointers
// - Call unsafe functions
// - Access mutable statics
// - Implement unsafe traits
```

---

## FFI

### Calling C

```axiom
extern "C" {
    fn printf(format: *const i8, ...) -> i32
    fn malloc(size: usize) -> *mut void
    fn free(ptr: *mut void)
}

fn call_c() {
    unsafe {
        printf(b"Hello\0".as_ptr())
    }
}
```

### Exporting

```axiom
#[export_name = "axiom_function"]
pub extern "C" fn exported(x: i32) -> i32 {
    x * 2
}
```

---

## Attributes

```axiom
// Derive
#[derive(Debug, Clone, Serialize)]

// Inline hints
#[inline]
#[inline(always)]
#[inline(never)]

// Deprecation
#[deprecated(since = "1.0", note = "use new_function instead")]

// Conditional compilation
#[cfg(target_os = "linux")]
#[cfg(feature = "async")]

// Documentation
#[doc = "Detailed description"]
#[doc(alias = "alternate_name")]
```

---

*End of Syntax Reference*
