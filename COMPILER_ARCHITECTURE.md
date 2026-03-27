# Axiom Compiler Architecture

## Overview

The Axiom compiler (axc) is designed with a multi-phase architecture that transforms human-readable source code into highly optimized machine code. The compiler follows the principle of "simple frontend, complex backend" where the parsing and initial processing are straightforward, while the optimization and code generation stages employ sophisticated algorithms.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Source Code (.ax)                         │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend                                 │
│  ┌──────────┐    ┌──────────┐    ┌───────────────────────────┐ │
│  │  Lexer   │───▶│  Parser  │───▶│  Semantic Analysis        │ │
│  │          │    │          │    │  - Name Resolution        │ │
│  │  Tokens  │    │   AST    │    │  - Type Checking          │ │
│  └──────────┘    └──────────┘    │  - Borrow Checking        │ │
│                                   │  - IR Generation          │ │
│                                   └───────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Middle-End (IR Layer)                         │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Axiom Intermediate Representation          │   │
│  │                      (AIR)                               │   │
│  │   - Typed, explicit control flow                        │   │
│  │   - SSA form                                            │   │
│  │   - Explicit memory operations                          │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           │                                      │
│                           ▼                                      │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                   Optimization Passes                    │   │
│  │   - Constant folding     - Dead code elimination        │   │
│  │   - Inlining             - Loop optimizations           │   │
│  │   - Common subexpression - Strength reduction           │   │
│  │   - Escape analysis      - Vectorization                │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Backend                                  │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────────┐  │
│  │ LLVM Backend │    │  Cranelift   │    │  Other Backends  │  │
│  │              │    │  Backend     │    │  (future)        │  │
│  │  - Full opt  │    │              │    │                  │  │
│  │  - Release   │    │  - Fast comp │    │  - WASM          │  │
│  └──────────────┘    │  - Debug     │    │  - GPU           │  │
│         │            └──────────────┘    └──────────────────┘  │
│         │                    │                                    │
│         └────────────────────┼────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Object Files / Executable                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Frontend Components

### Lexer (Tokenizer)

The lexer transforms source code text into a stream of tokens. It handles all lexical analysis including Unicode identifier support, numeric literal parsing, string literal processing, and comment handling.

**Implementation Details**

The lexer is implemented as a hand-written state machine rather than a generated lexer. This design choice was made to provide better error messages, more control over edge cases, and better performance. The lexer maintains a position in the source file and processes characters one at a time, building tokens as it goes.

```axiom
pub struct Lexer {
    source: String,
    position: usize,
    current_char: Option<char>,
    token_start: usize,
    diagnostics: Vec<Diagnostic>,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        self.token_start = self.position;
        
        match self.current_char {
            Some(c) if c.is_id_start() => self.lex_identifier_or_keyword(),
            Some(c) if c.is_digit() => self.lex_number(),
            Some('"') | Some('`') => self.lex_string(),
            Some('\'') => self.lex_char(),
            Some(c) => self.lex_punctuation_or_operator(),
            None => Token::new(TokenKind::EOF, self.token_start..self.position),
        }
    }
}
```

**Token Types**

The lexer produces tokens in several categories: keywords (fn, let, var, if, else, etc.), identifiers (user-defined names), literals (numbers, strings, characters), operators (+, -, *, etc.), and punctuation (brackets, commas, semicolons). Each token carries its location in the source file for diagnostic purposes.

**Error Recovery**

The lexer implements error recovery to continue tokenizing after encountering invalid characters. This allows the parser to report multiple errors in a single compilation pass rather than stopping at the first error. Common recovery strategies include skipping to the next whitespace or punctuation character.

### Parser

The parser transforms the token stream into an Abstract Syntax Tree (AST). It uses a recursive descent parsing strategy for most constructs, with a Pratt parser for expression handling.

**AST Design**

The AST is designed to be lossless, preserving all source information including comments and whitespace positions. This enables features like syntax highlighting, refactoring tools, and precise error messages.

```axiom
pub struct Module {
    pub span: Span,
    pub name: Option<String>,
    pub declarations: Vec<Declaration>,
    pub comments: Vec<Comment>,
}

pub enum Declaration {
    Function(FunctionDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Trait(TraitDecl),
    Impl(ImplBlock),
    Const(ConstDecl),
    Static(StaticDecl),
    Module(ModuleDecl),
    Import(ImportDecl),
}

pub struct FunctionDecl {
    pub span: Span,
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub attributes: Vec<Attribute>,
}

pub enum Expression {
    Literal(Literal),
    Ident(Ident),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    Lambda(LambdaExpr),
    If(IfExpr),
    Match(MatchExpr),
    Block(BlockExpr),
    Return(ReturnExpr),
    Break(BreakExpr),
    Continue(ContinueExpr),
    // ... more variants
}
```

**Operator Precedence**

The parser handles operator precedence using a Pratt parser, which allows for natural expression of precedence levels and easy modification of precedence rules.

| Precedence | Operators | Associativity |
|------------|-----------|---------------|
| 1 (highest) | Postfix: `()`, `[]`, `.`, `?` | Left |
| 2 | Prefix: `!`, `-`, `*`, `&` | Right |
| 3 | `**` | Right |
| 4 | `*`, `/`, `%`, `//` | Left |
| 5 | `+`, `-` | Left |
| 6 | `<<`, `>>`, `>>>` | Left |
| 7 | `&` | Left |
| 8 | `^` | Left |
| 9 | `\|` | Left |
| 10 | `==`, `!=`, `<`, `>`, `<=`, `>=`, `<=>` | Left |
| 11 | `&&` | Left |
| 12 | `\|\|` | Left |
| 13 (lowest) | `..`, `..=` | Left |

**Error Recovery**

The parser implements sophisticated error recovery to continue parsing after syntax errors. Recovery strategies include:
- Synchronizing at statement boundaries (semicolons, closing braces)
- Inserting missing tokens when confident about what's expected
- Skipping unexpected tokens to find a known-good parsing state
- Reporting multiple errors in a single compilation pass

```axiom
impl Parser {
    fn recover_to_statement_boundary(&mut self) {
        while !self.at_eof() {
            match self.current_token.kind {
                TokenKind::SEMICOLON | TokenKind::RBRACE => {
                    self.advance();
                    return;
                }
                TokenKind::FN | TokenKind::LET | TokenKind::VAR |
                TokenKind::IF | TokenKind::FOR | TokenKind::WHILE |
                TokenKind::RETURN => return,
                _ => self.advance(),
            }
        }
    }
}
```

### Semantic Analysis

Semantic analysis consists of multiple phases that validate the program and prepare it for code generation.

#### Name Resolution

Name resolution maps identifier uses to their declarations. It handles multiple scopes including module scope, function scope, block scope, and closure scope.

```axiom
pub struct NameResolver {
    scopes: Vec<Scope>,
    current_module: ModuleId,
    imports: HashMap<Ident, Vec<Import>>,
    diagnostics: Vec<Diagnostic>,
}

pub struct Scope {
    kind: ScopeKind,
    bindings: HashMap<Ident, Binding>,
    parent: Option<usize>,
}

pub struct Binding {
    kind: BindingKind,
    span: Span,
    definition: DefId,
}

pub enum BindingKind {
    Local(LocalId),
    Function(FunctionId),
    Type(TypeId),
    Module(ModuleId),
    Const(ConstId),
}
```

The name resolver also handles import resolution, checking visibility, and detecting name conflicts. It builds a complete symbol table that maps every identifier to its definition.

#### Type Checking

The type checker implements bidirectional type inference, combining type synthesis (inferring types from expressions) and type checking (verifying expressions match expected types).

```axiom
pub struct TypeChecker {
    type_context: TypeContext,
    inference: TypeInference,
    constraints: Vec<Constraint>,
    diagnostics: Vec<Diagnostic>,
}

impl TypeChecker {
    fn check_expr(&mut self, expr: &Expr, expected: &Type) -> Result!Type {
        match expr {
            Expr::Literal(lit) => self.check_literal(lit, expected),
            Expr::Binary(bin) => self.check_binary(bin, expected),
            Expr::Call(call) => self.check_call(call, expected),
            Expr::Lambda(lambda) => self.check_lambda(lambda, expected),
            // ...
        }
    }
    
    fn infer_expr(&mut self, expr: &Expr) -> Result!Type {
        match expr {
            Expr::Literal(lit) => self.infer_literal(lit),
            Expr::Ident(ident) => self.infer_ident(ident),
            Expr::Binary(bin) => self.infer_binary(bin),
            Expr::Call(call) => self.infer_call(call),
            // ...
        }
    }
}
```

**Type Inference Algorithm**

The type inference algorithm works by:
1. Collecting constraints from expressions
2. Solving the constraint system
3. Applying substitutions to get final types

```axiom
pub enum Constraint {
    Equal(Type, Type),
    TraitBound(Type, TraitId),
    Exists(TypeVar, Vec<TraitId>),
}

impl TypeInference {
    fn solve(&mut self) -> Result!() {
        while let Some(constraint) = self.constraints.pop() {
            match constraint {
                Constraint::Equal(t1, t2) => {
                    self.unify(&t1, &t2)?
                }
                Constraint::TraitBound(ty, trait_id) => {
                    self.check_trait_bound(&ty, trait_id)?
                }
                Constraint::Exists(var, traits) => {
                    self.resolve_existential(var, traits)?
                }
            }
        }
        Ok(())
    }
    
    fn unify(&mut self, t1: &Type, t2: &Type) -> Result!() {
        match (t1, t2) {
            (Type::Var(v1), Type::Var(v2)) if v1 == v2 => Ok(()),
            (Type::Var(v), t) | (t, Type::Var(v)) => {
                self.bind_var(*v, t.clone())
            }
            (Type::Int(i1), Type::Int(i2)) if i1 == i2 => Ok(()),
            (Type::Ref(r1, t1), Type::Ref(r2, t2)) => {
                self.unify_region(r1, r2)?;
                self.unify(t1, t2)
            }
            // ... more cases
            _ => Err(TypeError::mismatch(t1, t2)),
        }
    }
}
```

#### Borrow Checking

The borrow checker verifies that programs follow the ownership rules. It uses dataflow analysis to track loans and moves throughout the program.

```axiom
pub struct BorrowChecker {
    cfg: ControlFlowGraph,
    loans: Vec<Loan>,
    moves: Vec<Move>,
    lifetimes: LifetimeContext,
    diagnostics: Vec<Diagnostic>,
}

pub struct Loan {
    path: Path,
    span: Span,
    kind: BorrowKind,
    region: Region,
    point: ProgramPoint,
}

pub enum BorrowKind {
    Shared,    // &T
    Mutable,   // &mut T
    Shallow,   // &raw T
}
```

**Dataflow Analysis**

The borrow checker performs a dataflow analysis that:
1. Computes the control flow graph
2. Identifies all borrows and moves
3. Computes which borrows are active at each program point
4. Detects conflicts between active borrows

```axiom
impl BorrowChecker {
    fn check_function(&mut self, func: &Function) -> Result!() {
        // Build CFG
        self.cfg = self.build_cfg(func);
        
        // Find all loans and moves
        for block in self.cfg.blocks() {
            self.collect_loans_and_moves(block)?;
        }
        
        // Dataflow analysis
        let mut state = BorrowState::new();
        for block in self.cfg.blocks() {
            state = self.transfer_block(block, state)?;
            self.check_conflicts(block, &state)?;
        }
        
        Ok(())
    }
    
    fn check_conflicts(&self, block: BasicBlock, state: &BorrowState) -> Result!() {
        for loan in &state.active_loans {
            for other in &state.active_loans {
                if self.conflicts(loan, other) {
                    return Err(BorrowError::conflict(loan, other));
                }
            }
        }
        Ok(())
    }
}
```

---

## Intermediate Representation

### AIR (Axiom Intermediate Representation)

AIR is the primary intermediate representation used in the compiler. It is a typed, low-level representation that captures all program semantics explicitly.

**Design Goals**

AIR is designed to:
- Be easy to generate from the AST
- Be suitable for optimization passes
- Preserve type information for optimization
- Be independent of any target architecture
- Support whole-program analysis

**AIR Structure**

```axiom
pub struct AirModule {
    pub functions: Vec<AirFunction>,
    pub globals: Vec<AirGlobal>,
    pub types: Vec<AirType>,
}

pub struct AirFunction {
    pub id: FunctionId,
    pub name: String,
    pub type_: FunctionType,
    pub params: Vec<Param>,
    pub blocks: Vec<BasicBlock>,
    pub locals: Vec<Local>,
}

pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

pub enum Instruction {
    // Arithmetic
    BinOp(BinOp, Value, Value, Type),
    UnaryOp(UnaryOp, Value, Type),
    
    // Memory
    Alloc(Type),
    Load(Value, Type),
    Store(Value, Value, Type),
    GetElementPtr(Value, Vec<Value>, Type),
    
    // Control
    Call(FunctionId, Vec<Value>),
    Intrinsic(Intrinsic, Vec<Value>),
    
    // Conversions
    Cast(Value, Type, Type),
    
    // Comparisons
    Compare(CompareOp, Value, Value, Type),
    
    // Other
    Copy(Value, Type),
    Drop(Value, Type),
}

pub enum Terminator {
    Return(Option<Value>),
    Branch(BlockId),
    CondBranch(Value, BlockId, BlockId),
    Switch(Value, Vec<(Value, BlockId)>, BlockId),
    Unreachable,
}
```

**SSA Form**

AIR uses Static Single Assignment (SSA) form, where each variable is assigned exactly once. This simplifies many optimization passes.

```axiom
// Before SSA (AST)
fn example(x: i32) -> i32 {
    var result = x
    if x > 0 {
        result = x * 2
    }
    return result
}

// After SSA (AIR)
fn example(x: i32) -> i32 {
    entry:
        %0 = compare GT, x, 0
        cond_br %0, then_block, else_block
    
    then_block:
        %1 = mul x, 2
        br merge_block
    
    else_block:
        br merge_block
    
    merge_block:
        %2 = phi [%1, then_block], [x, else_block]
        return %2
}
```

---

## Optimization Passes

The compiler includes a comprehensive set of optimization passes that transform AIR into more efficient code.

### Optimization Pipeline

```axiom
pub struct OptimizationPipeline {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl OptimizationPipeline {
    pub fn run(&self, module: &mut AirModule) {
        for pass in &self.passes {
            pass.run(module);
        }
    }
}

pub trait OptimizationPass {
    fn name(&self) -> &str;
    fn run(&self, module: &mut AirModule);
}
```

### Core Optimizations

**Constant Folding**

Evaluates constant expressions at compile time.

```axiom
// Before
let x = 3 + 4
let y = x * 2

// After
let x = 7
let y = 14
```

**Dead Code Elimination**

Removes code that has no effect on program output.

```axiom
// Before
fn example(x: i32) -> i32 {
    let unused = compute_expensive_thing()
    return x
}

// After
fn example(x: i32) -> i32 {
    return x
}
```

**Inlining**

Replaces function calls with the function body.

```axiom
// Before
fn square(x: i32) -> i32 { x * x }
fn example(x: i32) -> i32 { square(x) + 1 }

// After
fn example(x: i32) -> i32 { x * x + 1 }
```

**Common Subexpression Elimination**

Eliminates redundant computations.

```axiom
// Before
let a = x + y
let b = x + y

// After
let temp = x + y
let a = temp
let b = temp
```

**Loop Optimizations**

- Loop invariant code motion
- Loop unrolling
- Loop fusion
- Strength reduction

```axiom
// Before (loop invariant)
for i in 0..n {
    let temp = expensive_computation()
    result += arr[i] * temp
}

// After
let temp = expensive_computation()
for i in 0..n {
    result += arr[i] * temp
}
```

**Escape Analysis**

Determines if allocations can be stack-allocated.

```axiom
// Before
fn example() -> i32 {
    let box = Box::new(42)  // Heap allocation
    *box
}

// After
fn example() -> i32 {
    let value = 42  // Stack allocation
    value
}
```

**Vectorization**

Transforms scalar operations into SIMD operations.

```axiom
// Before
for i in 0..n {
    c[i] = a[i] + b[i]
}

// After (SIMD)
for i in (0..n).step_by(8) {
    let va = load_vector(&a[i])
    let vb = load_vector(&b[i])
    let vc = va + vb
    store_vector(&mut c[i], vc)
}
```

---

## Backend

### LLVM Backend

The primary backend uses LLVM for code generation, providing access to mature optimization passes and support for many target architectures.

**LLVM IR Generation**

```axiom
pub struct LLVMBackend {
    context: llvm::Context,
    module: llvm::Module,
    builder: llvm::Builder,
    type_converter: TypeConverter,
}

impl LLVMBackend {
    pub fn compile(&mut self, air_module: &AirModule) -> llvm::Module {
        // Declare all functions first
        for func in &air_module.functions {
            self.declare_function(func);
        }
        
        // Compile function bodies
        for func in &air_module.functions {
            self.compile_function(func);
        }
        
        self.module.clone()
    }
    
    fn compile_instruction(&mut self, inst: &Instruction) {
        match inst {
            Instruction::BinOp(op, lhs, rhs, ty) => {
                let lhs_val = self.get_value(lhs);
                let rhs_val = self.get_value(rhs);
                let result = self.builder.binop(op, lhs_val, rhs_val);
                self.define_value(inst.dest(), result);
            }
            // ... other cases
        }
    }
}
```

**Target Support**

The LLVM backend supports multiple target architectures:
- x86_64 (Linux, macOS, Windows)
- AArch64 (Linux, macOS)
- ARM (Linux)
- RISC-V (Linux)
- WebAssembly

### Cranelift Backend

For faster debug builds, the Cranelift backend provides quick compilation without sacrificing correctness.

```axiom
pub struct CraneliftBackend {
    module: cranelift::Module,
    builder_context: cranelift::FunctionBuilderContext,
}

impl CraneliftBackend {
    pub fn compile(&mut self, air_module: &AirModule) -> cranelift::Module {
        for func in &air_module.functions {
            self.compile_function(func);
        }
        self.module.clone()
    }
}
```

---

## Linking

### Native Linking

The compiler invokes the system linker to produce the final executable.

```axiom
pub struct Linker {
    output_path: PathBuf,
    inputs: Vec<PathBuf>,
    libraries: Vec<String>,
    linker_path: Option<PathBuf>,
}

impl Linker {
    pub fn link(&self) -> Result!PathBuf {
        let mut cmd = Command::new(self.linker_path.as_ref()
            .unwrap_or(&PathBuf::from("ld")));
        
        cmd.arg("-o").arg(&self.output_path);
        
        for input in &self.inputs {
            cmd.arg(input);
        }
        
        for lib in &self.libraries {
            cmd.arg(format!("-l{}", lib));
        }
        
        let status = cmd.status()?;
        if status.success() {
            Ok(self.output_path.clone())
        } else {
            Err(LinkError::link_failed(status))
        }
    }
}
```

### LTO (Link-Time Optimization)

For release builds, the compiler supports link-time optimization, which enables whole-program optimization.

---

## Debug Information

### DWARF Generation

The compiler generates DWARF debug information for use with debuggers like GDB and LLDB.

```axiom
pub struct DebugInfoGenerator {
    dwarf: dwarf::Dwarf,
    compilation_unit: dwarf::CompilationUnit,
}

impl DebugInfoGenerator {
    fn generate_function(&mut self, func: &AirFunction, ast_func: &FunctionDecl) {
        let subprogram = self.dwarf.create_subprogram(
            self.compilation_unit,
            &func.name,
            ast_func.span,
        );
        
        for (i, param) in func.params.iter().enumerate() {
            self.dwarf.create_parameter(
                subprogram,
                &param.name,
                i as i32,
                param.type_,
                ast_func.params[i].span,
            );
        }
    }
}
```

---

## Performance Characteristics

The compiler is designed for both fast compilation and fast generated code.

### Compilation Speed Targets

| Build Type | Target (10K LOC) | Target (100K LOC) |
|------------|------------------|-------------------|
| Debug (Cranelift) | < 1 second | < 10 seconds |
| Debug (LLVM) | < 5 seconds | < 60 seconds |
| Release | < 30 seconds | < 5 minutes |

### Generated Code Performance

Generated code performance is competitive with equivalent C/C++ code:
- Numeric computations: within 5% of C
- Memory-intensive workloads: within 10% of C
- Concurrency: comparable to optimized C++ with thread pools

---

## Future Enhancements

### Planned Features

1. **Incremental Compilation**: Recompile only changed modules
2. **Parallel Compilation**: Compile multiple modules in parallel
3. **Profile-Guided Optimization**: Use runtime profiles to guide optimization
4. **JIT Compilation**: Just-in-time compilation for REPL and scripting
5. **GPU Backends**: Support for GPU code generation
6. **WASM Backend**: First-class WebAssembly support

---

*End of Compiler Architecture Documentation*
