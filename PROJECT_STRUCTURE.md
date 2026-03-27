# Axiom Programming Language - Project Structure

## Complete File Organization

```
axiom/
├── Axiom.toml                      # Project configuration
├── LICENSE                         # MIT/Apache-2.0 dual license
├── README.md                       # Project overview
├── CONTRIBUTING.md                 # Contribution guidelines
│
├── compiler/                       # Compiler implementation
│   ├── Axiom.toml
│   └── src/
│       ├── main.ax                 # Compiler entry point
│       │
│       ├── lexer/                  # Lexical analysis
│       │   ├── mod.ax
│       │   ├── token.ax            # Token definitions
│       │   ├── lexer.ax            # Lexer implementation
│       │   └── tests.ax            # Lexer tests
│       │
│       ├── parser/                 # Parsing
│       │   ├── mod.ax
│       │   ├── ast.ax              # AST node definitions
│       │   ├── parser.ax           # Parser implementation
│       │   ├── expr.ax             # Expression parsing
│       │   ├── stmt.ax             # Statement parsing
│       │   ├── pattern.ax          # Pattern parsing
│       │   └── tests.ax
│       │
│       ├── typeck/                 # Type checking
│       │   ├── mod.ax
│       │   ├── types.ax            # Type representation
│       │   ├── typeck.ax           # Type checker
│       │   ├── inference.ax        # Type inference
│       │   ├── unify.ax            # Unification algorithm
│       │   ├── coerce.ax           # Type coercion
│       │   └── tests.ax
│       │
│       ├── borrowck/               # Borrow checking
│       │   ├── mod.ax
│       │   ├── borrowck.ax         # Borrow checker
│       │   ├── loans.ax            # Loan tracking
│       │   ├── moves.ax            # Move tracking
│       │   ├── lifetimes.ax        # Lifetime analysis
│       │   ├── cfg.ax              # Control flow graph
│       │   └── tests.ax
│       │
│       ├── air/                    # Intermediate representation
│       │   ├── mod.ax
│       │   ├── types.ax            # AIR types
│       │   ├── instructions.ax     # Instructions
│       │   ├── builder.ax          # IR builder
│       │   ├── lower.ax            # AST to AIR lowering
│       │   └── verify.ax           # IR verification
│       │
│       ├── opt/                    # Optimization passes
│       │   ├── mod.ax
│       │   ├── pass.ax             # Pass trait
│       │   ├── const_fold.ax       # Constant folding
│       │   ├── dce.ax              # Dead code elimination
│       │   ├── inline.ax           # Inlining
│       │   ├── loop_opt.ax         # Loop optimizations
│       │   ├── mem2reg.ax          # Memory to register
│       │   ├── sroa.ax             # Scalar replacement
│       │   ├── vectorize.ax        # SIMD vectorization
│       │   └── pipeline.ax         # Optimization pipeline
│       │
│       ├── codegen/                # Code generation
│       │   ├── mod.ax
│       │   ├── llvm/               # LLVM backend
│       │   │   ├── mod.ax
│       │   │   ├── backend.ax
│       │   │   ├── types.ax
│       │   │   ├── instructions.ax
│       │   │   └── intrinsics.ax
│       │   │
│       │   ├── cranelift/          # Cranelift backend
│       │   │   ├── mod.ax
│       │   │   └── backend.ax
│       │   │
│       │   └── object.ax           # Object file writer
│       │
│       ├── linker/                 # Linker
│       │   ├── mod.ax
│       │   ├── linker.ax           # Linker implementation
│       │   ├── symbol.ax           # Symbol table
│       │   ├── layout.ax           # Section layout
│       │   ├── reloc.ax            # Relocations
│       │   └── writer.ax           # Output writer
│       │
│       ├── driver/                 # Compiler driver
│       │   ├── mod.ax
│       │   ├── session.ax          # Compilation session
│       │   ├── config.ax           # Configuration
│       │   └── diagnostics.ax      # Error reporting
│       │
│       └── utils/                  # Utilities
│           ├── mod.ax
│           ├── span.ax             # Source locations
│           ├── symbol.ax           # Interned strings
│           └── arena.ax            # Memory arena
│
├── runtime/                        # Runtime library
│   ├── Axiom.toml
│   └── src/
│       ├── lib.ax
│       ├── alloc/                  # Memory allocation
│       │   ├── mod.ax
│       │   ├── global.ax           # Global allocator
│       │   ├── heap.ax             # Heap allocator
│       │   └── arena.ax            # Arena allocator
│       │
│       ├── panic/                  # Panic handling
│       │   ├── mod.ax
│       │   └── unwind.ax           # Stack unwinding
│       │
│       ├── rt/                     # Runtime support
│       │   ├── mod.ax
│       │   ├── start.ax            # Startup code
│       │   ├── eh.ax               # Exception handling
│       │   └── tls.ax              # Thread-local storage
│       │
│       └── async/                  # Async runtime
│           ├── mod.ax
│           ├── executor.ax         # Task executor
│           ├── task.ax             # Task representation
│           └── waker.ax            # Waker implementation
│
├── std/                            # Standard library
│   ├── Axiom.toml
│   └── src/
│       ├── lib.ax
│       │
│       ├── core/                   # Core types
│       │   ├── mod.ax
│       │   ├── option.ax           # Option type
│       │   ├── result.ax           # Result type
│       │   ├── clone.ax            # Clone trait
│       │   ├── cmp.ax              # Comparison traits
│       │   ├── default.ax          # Default trait
│       │   ├── hash.ax             # Hash trait
│       │   └── iter.ax             # Iterator traits
│       │
│       ├── collections/            # Collections
│       │   ├── mod.ax
│       │   ├── vec.ax              # Vector
│       │   ├── string.ax           # String
│       │   ├── hashmap.ax          # Hash map
│       │   ├── hashset.ax          # Hash set
│       │   ├── btree.ax            # B-tree map
│       │   ├── linked_list.ax      # Linked list
│       │   └── deque.ax            # Double-ended queue
│       │
│       ├── io/                     # I/O
│       │   ├── mod.ax
│       │   ├── read.ax             # Read trait
│       │   ├── write.ax            # Write trait
│       │   ├── stdin.ax            # Standard input
│       │   ├── stdout.ax           # Standard output
│       │   ├── stderr.ax           # Standard error
│       │   ├── bufreader.ax        # Buffered reader
│       │   ├── bufwriter.ax        # Buffered writer
│       │   └── copy.ax             # Copy utilities
│       │
│       ├── fs/                     # File system
│       │   ├── mod.ax
│       │   ├── file.ax             # File operations
│       │   ├── path.ax             # Path handling
│       │   ├── dir.ax              # Directory operations
│       │   └── metadata.ax         # File metadata
│       │
│       ├── net/                    # Networking
│       │   ├── mod.ax
│       │   ├── tcp.ax              # TCP
│       │   ├── udp.ax              # UDP
│       │   ├── ip.ax               # IP addresses
│       │   └── socket.ax           # Socket operations
│       │
│       ├── sync/                   # Synchronization
│       │   ├── mod.ax
│       │   ├── mutex.ax            # Mutex
│       │   ├── rwlock.ax           # Read-write lock
│       │   ├── condvar.ax          # Condition variable
│       │   ├── barrier.ax          # Barrier
│       │   ├── channel.ax          # Channels
│       │   ├── atomic.ax           # Atomic types
│       │   └── once.ax             # One-time initialization
│       │
│       ├── thread/                 # Threading
│       │   ├── mod.ax
│       │   ├── thread.ax           # Thread management
│       │   ├── spawn.ax            # Thread spawning
│       │   └── join.ax             # Thread joining
│       │
│       ├── time/                   # Time
│       │   ├── mod.ax
│       │   ├── instant.ax          # Instant
│       │   ├── duration.ax         # Duration
│       │   └── system_time.ax      # System time
│       │
│       ├── fmt/                    # Formatting
│       │   ├── mod.ax
│       │   ├── formatter.ax        # Formatter
│       │   ├── display.ax          # Display trait
│       │   ├── debug.ax            # Debug trait
│       │   └── macros.ax           # format!/println! macros
│       │
│       ├── str/                    # String operations
│       │   ├── mod.ax
│       │   ├── traits.ax           # String traits
│       │   └── pattern.ax          # Pattern matching
│       │
│       ├── math/                   # Math functions
│       │   ├── mod.ax
│       │   ├── basic.ax            # Basic operations
│       │   ├── trig.ax             # Trigonometry
│       │   ├── exp.ax              # Exponential/log
│       │   └── consts.ax           # Mathematical constants
│       │
│       ├── rand/                   # Random numbers
│       │   ├── mod.ax
│       │   ├── rng.ax              # Random number generator
│       │   ├── distributions.ax    # Distributions
│       │   └── seq.ax              # Sequence operations
│       │
│       ├── serialize/              # Serialization
│       │   ├── mod.ax
│       │   ├── serialize.ax        # Serialize trait
│       │   ├── deserialize.ax      # Deserialize trait
│       │   ├── json.ax             # JSON support
│       │   └── binary.ax           # Binary serialization
│       │
│       ├── process/                # Process management
│       │   ├── mod.ax
│       │   ├── command.ax          # Command execution
│       │   ├── child.ax            # Child process
│       │   ├── exit.ax             # Exit codes
│       │   └── stdio.ax            # Stdio pipes
│       │
│       ├── env/                    # Environment
│       │   ├── mod.ax
│       │   ├── vars.ax             # Environment variables
│       │   ├── args.ax             # Command-line arguments
│       │   └── current_dir.ax      # Current directory
│       │
│       ├── mem/                    # Memory operations
│       │   ├── mod.ax
│       │   ├── manua.ax            # Manual memory management
│       │   ├── maybe_uninit.ax     # Uninitialized memory
│       │   └── transmute.ax        # Type transmutation
│       │
│       ├── ptr/                    # Pointer operations
│       │   ├── mod.ax
│       │   ├── non_null.ax         # Non-null pointers
│       │   └── addr.ax             # Address operations
│       │
│       ├── ffi/                    # Foreign function interface
│       │   ├── mod.ax
│       │   ├── c_str.ax            # C strings
│       │   ├── c_void.ax           # C void type
│       │   └── extern.ax           # External declarations
│       │
│       └── test/                   # Testing framework
│           ├── mod.ax
│           ├── test.ax             # Test attribute
│           ├── bench.ax            # Benchmarking
│           └── assert.ax           # Assertion macros
│
├── interpreter/                    # Interpreter implementation
│   ├── Axiom.toml
│   └── src/
│       ├── main.ax                 # REPL entry point
│       ├── mod.ax
│       ├── interp.ax               # Interpreter
│       ├── value.ax                # Runtime values
│       ├── env.ax                  # Environment
│       ├── gc.ax                   # Garbage collector
│       └── builtins.ax             # Built-in functions
│
├── tools/                          # Development tools
│   ├── axm/                        # Package manager
│   │   ├── Axiom.toml
│   │   └── src/
│   │       ├── main.ax
│   │       ├── build.ax            # Build command
│   │       ├── run.ax              # Run command
│   │       ├── test.ax             # Test command
│   │       ├── doc.ax              # Documentation command
│   │       ├── publish.ax          # Publish command
│   │       └── dependency.ax       # Dependency resolution
│   │
│   ├── axfmt/                      # Formatter
│   │   ├── Axiom.toml
│   │   └── src/
│   │       ├── main.ax
│   │       ├── format.ax           # Formatting logic
│   │       └── config.ax           # Configuration
│   │
│   ├── axclippy/                   # Linter
│   │   ├── Axiom.toml
│   │   └── src/
│   │       ├── main.ax
│   │       ├── lints.ax            # Lint definitions
│   │       └── passes.ax           # Lint passes
│   │
│   └── axls/                       # Language server
│       ├── Axiom.toml
│       └── src/
│           ├── main.ax
│           ├── server.ax           # LSP server
│           ├── analysis.ax         # Code analysis
│           ├── completion.ax       # Auto-completion
│           ├── hover.ax            # Hover information
│           ├── goto_def.ax         # Go to definition
│           ├── references.ax       # Find references
│           └── rename.ax           # Rename symbol
│
├── docs/                           # Documentation
│   ├── book/                       # The Axiom Book
│   │   ├── src/
│   │   │   ├── SUMMARY.md
│   │   │   ├── introduction.md
│   │   │   ├── getting-started.md
│   │   │   ├── syntax.md
│   │   │   ├── types.md
│   │   │   ├── functions.md
│   │   │   ├── structs.md
│   │   │   ├── enums.md
│   │   │   ├── traits.md
│   │   │   ├── generics.md
│   │   │   ├── ownership.md
│   │   │   ├── concurrency.md
│   │   │   ├── async.md
│   │   │   └── std-library.md
│   │   └── book.toml
│   │
│   ├── reference/                  # Language Reference
│   │   ├── language-spec.md
│   │   ├── syntax-reference.md
│   │   ├── type-system.md
│   │   └── std-api.md
│   │
│   └── internals/                  # Compiler Internals
│       ├── architecture.md
│       ├── lexer.md
│       ├── parser.md
│       ├── typeck.md
│       ├── borrowck.md
│       ├── ir.md
│       ├── optimization.md
│       └── codegen.md
│
├── tests/                          # Test suites
│   ├── ui/                         # UI tests
│   │   ├── compile-fail/           # Compilation failure tests
│   │   ├── compile-pass/           # Compilation success tests
│   │   └── run-pass/               # Execution tests
│   │
│   ├── integration/                # Integration tests
│   │   ├── basic.ax
│   │   ├── types.ax
│   │   ├── functions.ax
│   │   ├── structs.ax
│   │   ├── enums.ax
│   │   ├── generics.ax
│   │   ├── traits.ax
│   │   ├── concurrency.ax
│   │   └── ffi.ax
│   │
│   └── benchmarks/                 # Performance benchmarks
│       ├── micro/                  # Micro-benchmarks
│       ├── alloc/                  # Allocation benchmarks
│       └── real-world/             # Real-world benchmarks
│
├── examples/                       # Example programs
│   ├── hello.ax                    # Hello World
│   ├── fibonacci.ax                # Fibonacci
│   ├── http-server/                # HTTP server
│   ├── cli-tool/                   # CLI application
│   ├── web-app/                    # Web application
│   └── embedded/                   # Embedded systems
│
└── scripts/                        # Build scripts
    ├── build.sh                    # Build script
    ├── test.sh                     # Test script
    ├── release.sh                  # Release script
    └── install.sh                  # Installation script
```

## File Count Summary

| Component | Files | Lines of Code (Est.) |
|-----------|-------|---------------------|
| Compiler | ~80 | 50,000+ |
| Runtime | ~15 | 8,000+ |
| Standard Library | ~70 | 40,000+ |
| Interpreter | ~6 | 4,000+ |
| Tools | ~30 | 15,000+ |
| Tests | ~200 | 20,000+ |
| Examples | ~20 | 3,000+ |
| Documentation | ~30 | 15,000+ |
| **Total** | **~450** | **155,000+** |

## Build Commands

```bash
# Build compiler
axm build --release

# Run tests
axm test

# Format code
axfmt .

# Run linter
axclippy

# Generate documentation
axm doc

# Create new project
axm new my-project

# Build and run
axm run
```

## Technology Stack

| Component | Technology |
|-----------|------------|
| Compiler Backend | LLVM 17+ |
| Alternative Backend | Cranelift |
| Build System | Custom (axm) |
| Language Server | LSP Protocol |
| Formatter | Custom (axfmt) |
| Documentation | mdBook |
| CI/CD | GitHub Actions |
| FFI | C ABI compatible |

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64 | Tier 1 |
| Linux | AArch64 | Tier 1 |
| macOS | x86_64 | Tier 1 |
| macOS | AArch64 | Tier 1 |
| Windows | x86_64 | Tier 1 |
| FreeBSD | x86_64 | Tier 2 |
| WebAssembly | wasm32 | Tier 2 |
| ARM | armv7 | Tier 3 |
