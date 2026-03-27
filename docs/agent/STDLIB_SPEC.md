# Axiom Standard Library Specification

## Overview

The Axiom standard library (std) provides essential functionality for building applications. It is designed to be comprehensive, efficient, and easy to use. The library follows the same principles as the language: simple APIs with powerful implementations.

---

## Module Structure

```
std
├── core           # Core types and traits
├── collections    # Collection types
├── io             # Input/output
├── fs             # File system operations
├── net            # Networking
├── sync           # Synchronization primitives
├── thread         # Threading
├── async          # Async runtime
├── time           # Time and dates
├── fmt            # Formatting
├── str            # String manipulation
├── math           # Mathematical functions
├── rand           # Random number generation
├── hash           # Hashing algorithms
├── serialize      # Serialization
├── process        # Process management
├── env            # Environment access
├── mem            # Memory operations
├── ptr            # Pointer operations
├── ffi            # Foreign function interface
└── test           # Testing framework
```

---

## std::core

### Option

The Option type represents an optional value: either Some(value) or None.

```axiom
pub enum Option<T> {
    Some(T)
    None
}

impl<T> Option<T> {
    // Constructors
    pub fn some(value: T) -> Self
    pub fn none() -> Self
    
    // Queries
    pub fn is_some(self: &Self) -> bool
    pub fn is_none(self: &Self) -> bool
    pub fn contains(self: &Self, x: &T) -> bool where T: PartialEq
    
    // Accessors
    pub fn unwrap(self: Self) -> T
    pub fn unwrap_or(self: Self, default: T) -> T
    pub fn unwrap_or_else<F: FnOnce() -> T>(self: Self, f: F) -> T
    pub fn unwrap_or_default(self: Self) -> T where T: Default
    pub fn expect(self: Self, msg: &str) -> T
    
    // Transformations
    pub fn map<U, F: FnOnce(T) -> U>(self: Self, f: F) -> Option<U>
    pub fn map_or<U, F: FnOnce(T) -> U>(self: Self, default: U, f: F) -> U
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(
        self: Self, default: D, f: F
    ) -> U
    
    // Chaining
    pub fn and<U>(self: Self, optb: Option<U>) -> Option<U>
    pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self: Self, f: F) -> Option<U>
    pub fn or(self: Self, optb: Option<T>) -> Option<T>
    pub fn or_else<F: FnOnce() -> Option<T>>(self: Self, f: F) -> Option<T>
    
    // Conversion
    pub fn ok_or<E>(self: Self, err: E) -> Result<T, E>
    pub fn ok_or_else<E, F: FnOnce() -> E>(self: Self, f: F) -> Result<T, E>
    
    // Iterator
    pub fn iter(self: &Self) -> Iter<T>
    pub fn into_iter(self: Self) -> IntoIter<T>
}

// Type alias for nullable types
pub type ?T = Option<T>
```

### Result

The Result type represents either success (Ok) or failure (Err).

```axiom
pub enum Result<T, E> {
    Ok(T)
    Err(E)
}

impl<T, E> Result<T, E> {
    // Constructors
    pub fn ok(value: T) -> Self
    pub fn err(error: E) -> Self
    
    // Queries
    pub fn is_ok(self: &Self) -> bool
    pub fn is_err(self: &Self) -> bool
    pub fn contains(self: &Self, x: &T) -> bool where T: PartialEq
    pub fn contains_err(self: &Self, e: &E) -> bool where E: PartialEq
    
    // Accessors
    pub fn unwrap(self: Self) -> T
    pub fn unwrap_err(self: Self) -> E
    pub fn unwrap_or(self: Self, default: T) -> T
    pub fn unwrap_or_else<F: FnOnce(E) -> T>(self: Self, f: F) -> T
    pub fn unwrap_or_default(self: Self) -> T where T: Default
    pub fn expect(self: Self, msg: &str) -> T
    pub fn expect_err(self: Self, msg: &str) -> E
    
    // Transformations
    pub fn map<U, F: FnOnce(T) -> U>(self: Self, f: F) -> Result<U, E>
    pub fn map_err<F, F: FnOnce(E) -> F>(self: Self, f: F) -> Result<T, F>
    pub fn map_or<U, F: FnOnce(T) -> U>(self: Self, default: U, f: F) -> U
    pub fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(
        self: Self, default: D, f: F
    ) -> U
    
    // Chaining
    pub fn and<U>(self: Self, res: Result<U, E>) -> Result<U, E>
    pub fn and_then<U, F: FnOnce(T) -> Result<U, E>>(self: Self, f: F) -> Result<U, E>
    pub fn or<F>(self: Self, res: Result<T, F>) -> Result<T, F>
    pub fn or_else<F, F: FnOnce(E) -> Result<T, F>>(self: Self, f: F) -> Result<T, F>
    
    // Conversion
    pub fn ok(self: Self) -> Option<T>
    pub fn err(self: Self) -> Option<E>
    
    // Propagation
    pub fn try<R: Try<Ok = T>>(self: Self) -> R
}

// Type alias for common Result patterns
pub type Result!T = Result<T, Error>
```

### Core Traits

```axiom
/// Types that can be cloned.
pub trait Clone {
    fn clone(self: &Self) -> Self
    fn clone_from(self: &mut Self, source: &Self) {
        *self = source.clone()
    }
}

/// Types that can be copied bitwise.
pub trait Copy: Clone { }

/// Types with a default value.
pub trait Default {
    fn default() -> Self
}

/// Types that can be compared for equality.
pub trait PartialEq {
    fn eq(self: &Self, other: &Self) -> bool
    fn ne(self: &Self, other: &Self) -> bool { !self.eq(other) }
}

/// Types that have a total order.
pub trait Ord: PartialEq {
    fn cmp(self: &Self, other: &Self) -> Ordering
    
    fn lt(self: &Self, other: &Self) -> bool { self.cmp(other) == Ordering::Less }
    fn le(self: &Self, other: &Self) -> bool { self.cmp(other) != Ordering::Greater }
    fn gt(self: &Self, other: &Self) -> bool { self.cmp(other) == Ordering::Greater }
    fn ge(self: &Self, other: &Self) -> bool { self.cmp(other) != Ordering::Less }
}

pub enum Ordering {
    Less
    Equal
    Greater
}

/// Types that can be formatted.
pub trait Debug {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

/// Types that can be displayed.
pub trait Display {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

/// Types that can be hashed.
pub trait Hash {
    fn hash<H: Hasher>(self: &Self, state: &mut H)
}
```

---

## std::collections

### Vec

A contiguous growable array type.

```axiom
pub struct Vec<T> {
    ptr: *mut T
    len: usize
    cap: usize
    _marker: PhantomData<T>
}

impl<T> Vec<T> {
    // Constructors
    pub fn new() -> Self
    pub fn with_capacity(capacity: usize) -> Self
    pub fn from_raw_parts(ptr: *mut T, len: usize, cap: usize) -> Self
    
    // Capacity
    pub fn capacity(self: &Self) -> usize
    pub fn len(self: &Self) -> usize
    pub fn is_empty(self: &Self) -> bool
    pub fn reserve(self: &mut Self, additional: usize)
    pub fn shrink_to_fit(self: &mut Self)
    
    // Accessors
    pub fn get(self: &Self, index: usize) -> ?&T
    pub fn get_mut(self: &mut Self, index: usize) -> ?&mut T
    pub fn first(self: &Self) -> ?&T
    pub fn last(self: &Self) -> ?&T
    
    // Indexing (panics on out of bounds)
    pub fn index(self: &Self, index: usize) -> &T
    pub fn index_mut(self: &mut Self, index: usize) -> &mut T
    
    // Modification
    pub fn push(self: &mut Self, value: T)
    pub fn pop(self: &mut Self) -> ?T
    pub fn insert(self: &mut Self, index: usize, value: T)
    pub fn remove(self: &mut Self, index: usize) -> T
    pub fn swap_remove(self: &mut Self, index: usize) -> T
    pub fn clear(self: &mut Self)
    pub fn append(self: &mut Self, other: &mut Vec<T>)
    pub fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iter: I)
    pub fn truncate(self: &mut Self, len: usize)
    
    // Search
    pub fn contains(self: &Self, x: &T) -> bool where T: PartialEq
    pub fn binary_search(self: &Self, x: &T) -> Result<usize, usize> where T: Ord
    pub fn binary_search_by<F: Fn(&T) -> Ordering>(self: &Self, f: F) -> Result<usize, usize>
    
    // Sorting
    pub fn sort(self: &mut Self) where T: Ord
    pub fn sort_by<F: Fn(&T, &T) -> Ordering>(self: &mut Self, compare: F)
    pub fn sort_unstable(self: &mut Self) where T: Ord
    pub fn reverse(self: &mut Self)
    
    // Conversion
    pub fn as_slice(self: &Self) -> &[T]
    pub fn as_mut_slice(self: &mut Self) -> &mut [T]
    pub fn into_boxed_slice(self: Self) -> Box<[T]>
    pub fn into_iter(self: Self) -> IntoIter<T>
    
    // Iterator adapters
    pub fn iter(self: &Self) -> Iter<T>
    pub fn iter_mut(self: &mut Self) -> IterMut<T>
    pub fn drain<R: RangeBounds>(self: &mut Self, range: R) -> Drain<T>
}

impl<T: Clone> Vec<T> {
    pub fn from_slice(slice: &[T]) -> Self
    pub fn resize(self: &mut Self, new_len: usize, value: T)
    pub fn extend_from_slice(self: &mut Self, slice: &[T])
}

// Macro for creating vectors
macro_rules! vec {
    ($elem:expr; $n:expr) => { Vec::from([$elem; $n]) }
    ($($x:expr),+ $(,)?) => { Vec::from([$($x),+]) }
}
```

### String

A UTF-8 encoded, growable string.

```axiom
pub struct String {
    vec: Vec<u8>
}

impl String {
    // Constructors
    pub fn new() -> Self
    pub fn with_capacity(capacity: usize) -> Self
    pub fn from_utf8(vec: Vec<u8>) -> Result!Self
    pub fn from_utf8_lossy(v: &[u8]) -> Self
    pub fn from_raw_parts(buf: *mut u8, len: usize, cap: usize) -> Self
    
    // Capacity
    pub fn capacity(self: &Self) -> usize
    pub fn len(self: &Self) -> usize
    pub fn is_empty(self: &Self) -> bool
    pub fn reserve(self: &mut Self, additional: usize)
    pub fn shrink_to_fit(self: &mut Self)
    
    // Accessors
    pub fn as_str(self: &Self) -> &str
    pub fn as_bytes(self: &Self) -> &[u8]
    pub fn as_mut_str(self: &mut Self) -> &mut str
    pub fn chars(self: &Self) -> Chars
    pub fn char_indices(self: &Self) -> CharIndices
    
    // Modification
    pub fn push(self: &mut Self, ch: char)
    pub fn push_str(self: &mut Self, s: &str)
    pub fn pop(self: &mut Self) -> ?char
    pub fn remove(self: &mut Self, idx: usize) -> char
    pub fn insert(self: &mut Self, idx: usize, ch: char)
    pub fn insert_str(self: &mut Self, idx: usize, s: &str)
    pub fn clear(self: &mut Self)
    pub fn truncate(self: &mut Self, new_len: usize)
    
    // Search
    pub fn contains(self: &Self, pat: impl Pattern) -> bool
    pub fn starts_with(self: &Self, pat: impl Pattern) -> bool
    pub fn ends_with(self: &Self, pat: impl Pattern) -> bool
    pub fn find(self: &Self, pat: impl Pattern) -> ?usize
    pub fn rfind(self: &Self, pat: impl Pattern) -> ?usize
    
    // Substring
    pub fn split_at(self: &Self, mid: usize) -> (&str, &str)
    pub fn split_once(self: &Self, pat: impl Pattern) -> ?(&str, &str)
    
    // Transformation
    pub fn replace(self: &Self, from: &str, to: &str) -> String
    pub fn to_lowercase(self: &Self) -> String
    pub fn to_uppercase(self: &Self) -> String
    pub fn trim(self: &Self) -> &str
    pub fn trim_start(self: &Self) -> &str
    pub fn trim_end(self: &Self) -> &str
    
    // Conversion
    pub fn into_bytes(self: Self) -> Vec<u8>
    pub fn into_boxed_str(self: Self) -> Box<str>
}

// String slice type
pub type str = [char]

impl str {
    pub fn len(self: &Self) -> usize
    pub fn is_empty(self: &Self) -> bool
    pub fn chars(self: &Self) -> Chars
    pub fn bytes(self: &Self) -> Bytes
    // ... many more methods
}
```

### HashMap

A hash map with customizable hashing.

```axiom
pub struct HashMap<K, V, H: Hasher = DefaultHasher> {
    buckets: Vec<Option<Entry<K, V>>>
    len: usize
    hasher: H
}

struct Entry<K, V> {
    hash: u64
    key: K
    value: V
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self
    pub fn with_capacity(capacity: usize) -> Self
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    // Capacity
    pub fn capacity(self: &Self) -> usize
    pub fn len(self: &Self) -> usize
    pub fn is_empty(self: &Self) -> bool
    
    // Accessors
    pub fn get(self: &Self, key: &K) -> ?&V
    pub fn get_mut(self: &mut Self, key: &K) -> ?&mut V
    pub fn get_key_value(self: &Self, key: &K) -> ?(&K, &V)
    pub fn contains_key(self: &Self, key: &K) -> bool
    
    // Modification
    pub fn insert(self: &mut Self, key: K, value: V) -> ?V
    pub fn remove(self: &mut Self, key: &K) -> ?V
    pub fn remove_entry(self: &mut Self, key: &K) -> ?(K, V)
    pub fn clear(self: &mut Self)
    
    // Entry API
    pub fn entry(self: &mut Self, key: K) -> Entry<K, V>
    pub fn or_insert(self: &mut Self, key: K, default: V) -> &mut V
    pub fn or_insert_with<F: FnOnce() -> V>(self: &mut Self, key: K, f: F) -> &mut V
    
    // Iteration
    pub fn iter(self: &Self) -> Iter<K, V>
    pub fn iter_mut(self: &mut Self) -> IterMut<K, V>
    pub fn keys(self: &Self) -> Keys<K, V>
    pub fn values(self: &Self) -> Values<K, V>
    pub fn values_mut(self: &mut Self) -> ValuesMut<K, V>
    pub fn into_iter(self: Self) -> IntoIter<K, V>
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>)
    Vacant(VacantEntry<'a, K, V>)
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn or_insert(self: Self, default: V) -> &'a mut V
    pub fn or_insert_with<F: FnOnce() -> V>(self: Self, f: F) -> &'a mut V
    pub fn and_modify<F: FnOnce(&mut V)>(self: Self, f: F) -> Self
}

// Macro for creating maps
macro_rules! map {
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut m = HashMap::new()
            $(m.insert($key, $value));+
            m
        }
    }
}
```

---

## std::io

### File

File operations.

```axiom
pub struct File {
    handle: RawHandle
}

impl File {
    pub fn open(path: impl AsRef<Path>) -> Result!File
    pub fn create(path: impl AsRef<Path>) -> Result!File
    pub fn options() -> OpenOptions
    
    pub fn read(&mut self, buf: &mut [u8]) -> Result!usize
    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result!usize
    pub fn read_to_string(&mut self, buf: &mut String) -> Result!usize
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result!void
    
    pub fn write(&mut self, buf: &[u8]) -> Result!usize
    pub fn write_all(&mut self, buf: &[u8]) -> Result!void
    pub fn flush(&mut self) -> Result!void
    
    pub fn seek(&mut self, pos: SeekFrom) -> Result!u64
    pub fn set_len(&mut self, size: u64) -> Result!void
    pub fn metadata(&self) -> Result!Metadata
    
    pub fn try_clone(&self) -> Result!File
}

pub struct OpenOptions {
    read: bool
    write: bool
    append: bool
    truncate: bool
    create: bool
    create_new: bool
}

impl OpenOptions {
    pub fn new() -> Self
    pub fn read(self: &mut Self, read: bool) -> &mut Self
    pub fn write(self: &mut Self, write: bool) -> &mut Self
    pub fn append(self: &mut Self, append: bool) -> &mut Self
    pub fn truncate(self: &mut Self, truncate: bool) -> &mut Self
    pub fn create(self: &mut Self, create: bool) -> &mut Self
    pub fn create_new(self: &mut Self, create_new: bool) -> &mut Self
    pub fn open(self: &Self, path: impl AsRef<Path>) -> Result!File
}
```

### Standard Streams

```axiom
pub fn stdin() -> Stdin
pub fn stdout() -> Stdout
pub fn stderr() -> Stderr

pub struct Stdin { /* ... */ }
pub struct Stdout { /* ... */ }
pub struct Stderr { /* ... */ }

impl Stdin {
    pub fn read_line(&mut self, buf: &mut String) -> Result!usize
    pub fn read_to_string(&mut self, buf: &mut String) -> Result!usize
    pub fn lock(&self) -> StdinLock
}

impl Stdout {
    pub fn write(&mut self, buf: &[u8]) -> Result!usize
    pub fn flush(&mut self) -> Result!void
    pub fn lock(&self) -> StdoutLock
}

// Print macros
macro_rules! print {
    ($($arg:tt)*) => {
        std::io::stdout().write_fmt(format_args!($($arg)*))
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        print!("{}\n", format_args!($($arg)*))
    }
}
```

### BufReader / BufWriter

```axiom
pub struct BufReader<R: Read> {
    inner: R
    buf: Vec<u8>
}

impl<R: Read> BufReader<R> {
    pub fn new(inner: R) -> Self
    pub fn with_capacity(capacity: usize, inner: R) -> Self
    pub fn read_line(&mut self, buf: &mut String) -> Result!usize
    pub fn lines(self: Self) -> Lines<R>
}

pub struct BufWriter<W: Write> {
    inner: W
    buf: Vec<u8>
}

impl<W: Write> BufWriter<W> {
    pub fn new(inner: W) -> Self
    pub fn with_capacity(capacity: usize, inner: W) -> Self
    pub fn flush(&mut self) -> Result!void
}
```

---

## std::sync

### Mutex

Mutual exclusion primitive.

```axiom
pub struct Mutex<T> {
    inner: UnsafeCell<T>
    lock: AtomicBool
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self
    pub fn lock(self: &Self) -> MutexGuard<T>
    pub fn try_lock(self: &Self) -> TryLockResult<MutexGuard<T>>
    pub fn into_inner(self: Self) -> T
    pub fn get_mut(self: &mut Self) -> &mut T
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T
    fn deref(self: &Self) -> &T
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(self: &mut Self) -> &mut T
}
```

### RwLock

Reader-writer lock.

```axiom
pub struct RwLock<T> {
    inner: UnsafeCell<T>
    readers: AtomicUsize
    writer: AtomicBool
}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self
    pub fn read(self: &Self) -> RwLockReadGuard<T>
    pub fn write(self: &Self) -> RwLockWriteGuard<T>
    pub fn try_read(self: &Self) -> TryLockResult<RwLockReadGuard<T>>
    pub fn try_write(self: &Self) -> TryLockResult<RwLockWriteGuard<T>>
}

pub struct RwLockReadGuard<'a, T> { /* ... */ }
pub struct RwLockWriteGuard<'a, T> { /* ... */ }
```

### Channel

Multi-producer, multi-consumer channels.

```axiom
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
pub fn bounded<T>(capacity: usize) -> (Sender<T>, Receiver<T>)

pub struct Sender<T> {
    inner: Arc<ChannelInner<T>>
}

impl<T> Sender<T> {
    pub fn send(self: &Self, value: T) -> Result!void
    pub fn try_send(self: &Self, value: T) -> TrySendResult
}

pub struct Receiver<T> {
    inner: Arc<ChannelInner<T>>
}

impl<T> Receiver<T> {
    pub fn recv(self: &Self) -> Result!T
    pub fn try_recv(self: &Self) -> TryRecvResult
    pub fn iter(self: &Self) -> Iter<T>
}

impl<T> Iterator for Receiver<T> {
    type Item = T
    fn next(&mut self) -> ?T
}
```

---

## std::fmt

### Formatting Traits

```axiom
pub trait Display {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

pub trait Debug {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

pub trait LowerHex {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

pub trait UpperHex {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

pub trait Binary {
    fn fmt(self: &Self, f: &mut Formatter) -> Result!void
}

pub struct Formatter<'a> {
    buf: &'a mut String
    options: FormatOptions
}

impl Formatter<'_> {
    pub fn write_str(self: &mut Self, s: &str) -> Result!void
    pub fn write_char(self: &mut Self, c: char) -> Result!void
    pub fn write_fmt(self: &mut Self, args: Arguments) -> Result!void
    
    pub fn pad(self: &mut Self, s: &str) -> Result!void
    pub fn pad_integral(self: &mut Self, is_nonnegative: bool, prefix: &str, buf: &str) -> Result!void
    
    pub fn width(self: &Self) -> ?usize
    pub fn precision(self: &Self) -> ?usize
    pub fn alternate(self: &Self) -> bool
    pub fn sign_plus(self: &Self) -> bool
    pub fn sign_aware_zero_pad(self: &Self) -> bool
}
```

### format! Macro

```axiom
macro_rules! format {
    ($fmt:expr) => { /* ... */ }
    ($fmt:expr, $($arg:tt)*) => { /* ... */ }
}

// Usage
let s = format!("Hello, {}!", name)
let s = format!("{0} {1} {0}", "a", "b")  // "a b a"
let s = format!("{name}", name = "test")
let s = format!("{:?}", value)  // Debug format
let s = format!("{:x}", 255)    // Hex format
let s = format!("{:.2}", 3.14159)  // 3.14
let s = format!("{:10}", "hi")  // Right-align in 10 chars
let s = format!("{:<10}", "hi") // Left-align
let s = format!("{:^10}", "hi") // Center-align
let s = format!("{:010}", 42)   // Zero-padded: 0000000042
```

---

## std::math

### Mathematical Functions

```axiom
pub const PI: f64 = 3.14159265358979323846
pub const E: f64 = 2.71828182845904523536
pub const TAU: f64 = 6.28318530717958647693
pub const SQRT2: f64 = 1.41421356237309504880

// Basic operations
pub fn abs<T: Signed>(x: T) -> T
pub fn signum<T: Signed>(x: T) -> T
pub fn min<T: Ord>(a: T, b: T) -> T
pub fn max<T: Ord>(a: T, b: T) -> T
pub fn clamp<T: Ord>(value: T, min: T, max: T) -> T

// Floating-point operations
pub fn floor(x: f64) -> f64
pub fn ceil(x: f64) -> f64
pub fn round(x: f64) -> f64
pub fn trunc(x: f64) -> f64
pub fn fract(x: f64) -> f64

// Power functions
pub fn pow(base: f64, exp: f64) -> f64
pub fn sqrt(x: f64) -> f64
pub fn cbrt(x: f64) -> f64
pub fn exp(x: f64) -> f64
pub fn exp2(x: f64) -> f64
pub fn ln(x: f64) -> f64
pub fn log2(x: f64) -> f64
pub fn log10(x: f64) -> f64

// Trigonometric functions
pub fn sin(x: f64) -> f64
pub fn cos(x: f64) -> f64
pub fn tan(x: f64) -> f64
pub fn asin(x: f64) -> f64
pub fn acos(x: f64) -> f64
pub fn atan(x: f64) -> f64
pub fn atan2(y: f64, x: f64) -> f64
pub fn sinh(x: f64) -> f64
pub fn cosh(x: f64) -> f64
pub fn tanh(x: f64) -> f64

// Utility
pub fn hypot(x: f64, y: f64) -> f64
pub fn recip(x: f64) -> f64
pub fn rem_euclid(x: f64, y: f64) -> f64
```

---

## std::iter

### Iterator Trait

```axiom
pub trait Iterator {
    type Item
    
    fn next(&mut self) -> ?Self::Item
    
    // Provided methods
    fn size_hint(self: &Self) -> (usize, ?usize) { (0, null) }
    
    // Consumers
    fn count(self: Self) -> usize
    fn last(self: Self) -> ?Self::Item
    fn nth(&mut self, n: usize) -> ?Self::Item
    fn fold<B, F>(self: Self, init: B, f: F) -> B
    fn for_each<F>(self: Self, f: F)
    fn collect<B: FromIterator<Self::Item>>(self: Self) -> B
    fn reduce<F>(self: Self, f: F) -> ?Self::Item
    
    // Predicates
    fn all<F>(self: Self, f: F) -> bool
    fn any<F>(self: Self, f: F) -> bool
    fn find<F>(&mut self, f: F) -> ?Self::Item
    fn find_map<F, B>(&mut self, f: F) -> ?B
    fn position<F>(&mut self, f: F) -> ?usize
    
    // Transformations
    fn map<B, F>(self: Self, f: F) -> Map<Self, F>
    fn filter<F>(self: Self, f: F) -> Filter<Self, F>
    fn filter_map<B, F>(self: Self, f: F) -> FilterMap<Self, F>
    fn flat_map<U, F>(self: Self, f: F) -> FlatMap<Self, F>
    fn flatten(self: Self) -> Flatten<Self>
    fn inspect<F>(self: Self, f: F) -> Inspect<Self, F>
    fn enumerate(self: Self) -> Enumerate<Self>
    fn peekable(self: Self) -> Peekable<Self>
    fn skip(self: Self, n: usize) -> Skip<Self>
    fn take(self: Self, n: usize) -> Take<Self>
    fn skip_while<F>(self: Self, f: F) -> SkipWhile<Self, F>
    fn take_while<F>(self: Self, f: F) -> TakeWhile<Self, F>
    fn step_by(self: Self, step: usize) -> StepBy<Self>
    fn chain<I: Iterator<Item = Self::Item>>(self: Self, other: I) -> Chain<Self, I>
    fn zip<I: Iterator>(self: Self, other: I) -> Zip<Self, I>
    fn rev(self: Self) -> Rev<Self> where Self: DoubleEndedIterator
    fn cloned<'a, T: Clone>(self: Self) -> Cloned<Self>
    fn copied<'a, T: Copy>(self: Self) -> Copied<Self>
    
    // Sorting (when collectable)
    fn sorted(self: Self) -> Vec<Self::Item> where Self::Item: Ord
    fn sorted_by<F>(self: Self, compare: F) -> Vec<Self::Item>
}
```

### IntoIterator

```axiom
pub trait IntoIterator {
    type Item
    type IntoIter: Iterator<Item = Self::Item>
    
    fn into_iter(self: Self) -> Self::IntoIter
}

impl<T> IntoIterator for Vec<T> {
    type Item = T
    type IntoIter = IntoIter<T>
    
    fn into_iter(self: Self) -> IntoIter<T>
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T
    type IntoIter = Iter<'a, T>
    
    fn into_iter(self: Self) -> Iter<'a, T>
}
```

---

## std::test

### Testing Framework

```axiom
/// Mark a function as a test.
pub attribute test

/// Mark a function as a benchmark.
pub attribute bench

/// Assert a condition is true.
macro_rules! assert {
    ($cond:expr) => {
        if !$cond {
            panic!("assertion failed: {}", stringify!($cond))
        }
    }
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            panic!($($arg)*)
        }
    }
}

/// Assert two values are equal.
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (l, r) if l == r => {}
            (l, r) => panic!("assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`", l, r)
        }
    }
}

/// Assert two values are not equal.
macro_rules! assert_ne {
    ($left:expr, $right:expr) => { /* ... */ }
}

// Test example
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4)
}

#[test]
#[should_panic(expected = "overflow")]
fn test_overflow() {
    let _ = i32::MAX + 1
}

// Benchmark example
#[bench]
fn bench_vec_push(b: &mut Bencher) {
    b.iter(|| {
        let mut v = Vec::new()
        for i in 0..1000 {
            v.push(i)
        }
        v
    })
}
```

---

*End of Standard Library Specification*
