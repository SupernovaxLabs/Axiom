# AGENTS.md - AI Agent Development Guidelines for Axiom Language

## Introduction

This document provides comprehensive guidelines for AI agents (including large language models and automated development tools) contributing to the Axiom programming language project. The goal is to ensure consistent, high-quality contributions that align with the project's design philosophy and technical requirements.

---

## Agent Roles and Responsibilities

### Core Agent Types

The Axiom project defines several categories of AI agents, each with specific responsibilities and access levels. Understanding these roles is essential for any AI system contributing to the project.

**Language Design Agents** are responsible for proposing and refining language features. These agents analyze user feedback, research existing language designs, and propose new syntax or semantics that align with Axiom's philosophy of simplicity on the frontend with complexity in the backend. Language design agents must have deep knowledge of programming language theory and practical experience with multiple programming paradigms.

**Compiler Development Agents** work on the implementation of the compiler itself. These agents write code for the lexer, parser, type checker, borrow checker, optimizer, and code generator. Compiler development agents must understand compiler construction deeply and be able to implement features while maintaining performance and correctness.

**Standard Library Agents** contribute to the standard library implementation. These agents implement data structures, algorithms, I/O operations, and utility functions that form the foundation of Axiom programs. Standard library agents must prioritize API design, performance, and documentation quality.

**Documentation Agents** create and maintain project documentation. These agents write tutorials, reference documentation, API docs, and guides. Documentation agents must be able to explain complex concepts clearly and produce well-organized, comprehensive documentation.

**Testing Agents** develop and maintain the test suite. These agents write unit tests, integration tests, property-based tests, and benchmarks. Testing agents must understand testing methodologies thoroughly and be able to identify edge cases and potential failure modes.

**Tooling Agents** work on development tools such as the language server, formatter, linter, and debugger. These agents build infrastructure that improves developer experience. Tooling agents must understand how developers interact with the language and prioritize usability.

---

## Design Principles for Agent Contributions

### Philosophy Alignment

Every contribution must align with Axiom's core philosophy. AI agents should evaluate their proposed contributions against these principles before submitting.

**Simplicity First** means that any new feature must justify its complexity. When proposing a new language feature, agents must demonstrate that the feature solves a real problem that cannot be solved with existing features, that the feature is orthogonal to existing features rather than overlapping with them, that the feature has a simple mental model that developers can easily understand, and that the feature does not introduce special cases or exceptions to existing rules.

**Zero-Cost Abstractions** means that high-level features must compile to efficient code. When implementing abstractions, agents must ensure that the abstraction is fully resolved at compile time, that the generated code is as efficient as hand-written low-level code, that the abstraction does not introduce hidden allocations or runtime overhead, and that performance characteristics are documented and predictable.

**Safety by Default** means that safe operations are the default. When implementing features, agents must ensure that memory safety is guaranteed for safe code, that thread safety is enforced by the type system where applicable, that unsafe operations require explicit opt-in with clear documentation, and that the safe subset of the language is Turing complete and practical.

**Explicit Over Implicit** means that important behavior should be visible in code. When designing features, agents must ensure that side effects are declared explicitly, that resource management is visible in the code, that type conversions are explicit except where obvious, and that control flow is clear and predictable.

### Code Quality Standards

All code contributions must meet these quality standards to be accepted into the project.

**Correctness** is the most important criterion. Code must produce correct results for all inputs, handle edge cases appropriately, and not introduce memory safety violations. Every non-trivial function should have tests that verify its correctness across a range of inputs, including boundary conditions and error cases.

**Performance** is critical for a systems programming language. Code should be efficient in terms of time complexity, space complexity, and memory allocation patterns. Performance-critical code should be benchmarked and optimized, with optimization decisions documented.

**Readability** ensures maintainability. Code should be self-documenting with clear names and structure. Complex logic should include comments explaining the reasoning. The formatting should follow the project style guide.

**Documentation** is required for public APIs. Every public type, function, and module must have documentation comments explaining its purpose, parameters, return values, and any important behavior. Examples should be included where helpful.

---

## Contribution Workflow

### Issue Analysis

Before starting work on a contribution, agents must analyze the relevant issue or feature request thoroughly. This analysis should include understanding the problem being solved, researching existing solutions in other languages, identifying potential approaches and their tradeoffs, and estimating the scope and complexity of the implementation.

When analyzing issues, agents should ask clarifying questions if the requirements are unclear. It is better to spend time clarifying requirements than to implement the wrong solution. Agents should also check for duplicate issues or related work that might affect the implementation approach.

### Design Proposals

For significant features or changes, agents must submit a design proposal before implementation. The proposal should include a clear statement of the problem being solved, a description of the proposed solution, alternatives considered and why they were rejected, the impact on existing code and compatibility, the implementation plan with milestones, and testing and documentation plans.

Design proposals should be reviewed by multiple stakeholders before implementation begins. This review process helps identify issues early and ensures that the implementation will meet the project's standards.

### Implementation Process

During implementation, agents should follow an iterative approach with frequent commits. Each commit should represent a logical unit of change with a clear commit message. Large features should be broken into smaller, reviewable pieces.

**Commit Message Format**
```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: feat, fix, docs, style, refactor, test, chore

Example:
```
feat(typeck): implement generic type inference

Add bidirectional type inference for generic function calls.
The implementation uses a constraint-based approach where
constraints are collected during type synthesis and solved
during type checking.

Closes #1234
```

### Testing Requirements

All contributions must include appropriate tests. The testing requirements vary by contribution type:

**Bug fixes** must include a regression test that would have failed before the fix and passes after the fix. The test should be minimal but complete, clearly demonstrating the bug and its fix.

**New features** must include unit tests covering all public functionality, integration tests for complex features, property-based tests where applicable, and benchmarks for performance-critical code.

**Refactoring** must ensure all existing tests continue to pass, and new tests should be added if the refactoring changes behavior or if gaps in test coverage are discovered.

### Documentation Requirements

Documentation is not optional. Every contribution must include appropriate documentation updates:

**API documentation** must be added or updated for any public types or functions. Documentation comments should explain what the item does, not how it does it. Include examples for non-trivial APIs.

**User documentation** must be updated for features that affect users. This includes tutorials, guides, and reference documentation. New features should have examples showing typical usage patterns.

**Internal documentation** should be updated for changes to internal architecture or algorithms. This helps future contributors understand the codebase.

---

## Technical Guidelines

### Lexer Development

When contributing to the lexer, agents must ensure that the implementation handles all valid tokens according to the language specification, provides accurate location information for all tokens, recovers gracefully from lexical errors, and performs efficiently on large files.

The lexer should be implemented as a deterministic finite automaton where possible, with clear state transitions and error handling. Unicode handling must follow the Unicode Standard for identifier syntax.

```axiom
// Example lexer token definition
struct Token {
    kind: TokenKind,
    span: Span,
    value: TokenValue,
}

enum TokenKind {
    // Keywords
    FN, LET, VAR, CONST,
    
    // Literals  
    INT, FLOAT, STRING, CHAR,
    
    // Identifiers
    IDENT,
    
    // Operators
    PLUS, MINUS, STAR, SLASH,
    
    // Punctuation
    LPAREN, RPAREN, LBRACE, RBRACE,
    
    // Special
    EOF, ERROR,
}
```

### Parser Development

When contributing to the parser, agents must ensure that the implementation correctly parses all valid syntax according to the grammar, produces meaningful error messages for invalid syntax, recovers from errors to allow continued parsing, and builds an accurate AST with all necessary information.

The parser should use recursive descent for most constructs, with operator precedence parsing for expressions. Error recovery should attempt to synchronize at statement boundaries.

```axiom
// Example parser implementation
impl Parser {
    fn parse_function(&mut self) -> Result!FunctionDecl {
        self.expect(TokenKind::FN)?;
        let name = self.expect_ident()?;
        let type_params = self.parse_type_params()?;
        let params = self.parse_params()?;
        let return_type = self.parse_return_type()?;
        let body = self.parse_block()?;
        
        Ok(FunctionDecl {
            name,
            type_params,
            params,
            return_type,
            body,
        })
    }
}
```

### Type Checker Development

When contributing to the type checker, agents must ensure that the implementation correctly infers types for all expressions, enforces type constraints and bounds, detects and reports type errors clearly, and integrates with the borrow checker.

The type checker should implement bidirectional type inference with constraint collection and solving. Type error messages should explain what went wrong and suggest fixes when possible.

```axiom
// Example type checker implementation
impl TypeChecker {
    fn check_expr(&mut self, expr: &Expr, expected: &Type) -> Result!Type {
        match expr {
            Expr::Literal(lit) => self.check_literal(lit, expected),
            Expr::Binary(bin) => self.check_binary(bin, expected),
            Expr::Call(call) => self.check_call(call, expected),
            // ... other cases
        }
    }
    
    fn infer_expr(&mut self, expr: &Expr) -> Result!Type {
        match expr {
            Expr::Literal(lit) => self.infer_literal(lit),
            Expr::Binary(bin) => self.infer_binary(bin),
            Expr::Call(call) => self.infer_call(call),
            // ... other cases
        }
    }
}
```

### Borrow Checker Development

When contributing to the borrow checker, agents must understand the ownership model deeply, implement the borrow checking algorithm correctly, provide clear error messages for violations, and handle all edge cases including closures and async functions.

The borrow checker must track all moves, loans, and lifetimes throughout the program. It must enforce the rules that immutable and mutable borrows cannot coexist, that moved values cannot be used, and that references do not outlive their referents.

```axiom
// Example borrow checker implementation
impl BorrowChecker {
    fn check_function(&mut self, func: &Function) -> Result!() {
        // Build control flow graph
        let cfg = self.build_cfg(&func.body);
        
        // Track loans and moves at each program point
        let mut state = BorrowState::new();
        
        // Dataflow analysis
        for block in cfg.blocks() {
            state = self.transfer_block(block, state)?;
        }
        
        Ok(())
    }
    
    fn check_borrow(&mut self, borrow: &Borrow) -> Result!() {
        // Ensure no conflicting borrows exist
        if let Some(conflict) = self.find_conflict(borrow) {
            return Err(BorrowError::conflict(borrow, conflict));
        }
        
        // Record the loan
        self.loans.push(borrow);
        Ok(())
    }
}
```

### Code Generator Development

When contributing to the code generator, agents must ensure that the implementation produces correct machine code for all valid programs, optimizes code appropriately for the target architecture, generates correct debug information, and handles all platform-specific details correctly.

The code generator should use LLVM as the primary backend, with support for alternative backends like Cranelift for faster debug builds. Platform ABIs must be followed correctly for proper interoperability.

```axiom
// Example code generator implementation
impl CodeGenerator {
    fn gen_function(&mut self, func: &Function) -> Result!llvm::Function {
        let llvm_func = self.module.add_function(
            &func.name,
            self.convert_signature(&func.signature)
        );
        
        let entry = llvm_func.append_block("entry");
        self.builder.position_at_end(entry);
        
        for block in &func.body.blocks {
            self.gen_block(block)?;
        }
        
        Ok(llvm_func)
    }
}
```

---

## Standard Library Guidelines

### API Design Principles

Standard library APIs must follow these design principles to ensure consistency and usability.

**Consistency** means that similar operations should have similar names and signatures. For example, all collection types should have `.len()`, `.is_empty()`, `.iter()`, and similar methods with consistent behavior.

**Completeness** means that APIs should cover common use cases. Every type should have methods for creating, inspecting, transforming, and consuming values. Missing functionality forces users to write workarounds, which is a sign of incomplete API design.

**Convenience** means that common operations should be easy. Frequently used patterns should have dedicated methods, even if they could be expressed in terms of more primitive operations. The standard library should optimize for reader experience over writer experience.

**Safety** means that APIs should prevent misuse where possible. Types should make invalid states unrepresentable. Operations that can fail should return `Result` rather than panic. Unsafe operations should be clearly marked.

### Implementation Guidelines

Standard library implementations must meet high standards for quality and performance.

**Performance** is critical for standard library code. Hot paths should be optimized using appropriate algorithms and data structures. Memory allocation should be minimized. Performance characteristics should be documented and predictable.

**Error Handling** should use `Result` for recoverable errors and panic only for programming errors. Error messages should be informative. Error types should implement the `Error` trait and be composable.

**Documentation** must be comprehensive. Every public item needs documentation. Examples should demonstrate typical usage. Edge cases and error conditions should be documented.

```axiom
/// A contiguous growable array type.
/// 
/// # Examples
/// 
/// ```
/// let mut vec = Vec::new()
/// vec.push(1)
/// vec.push(2)
/// assert_eq!(vec.len(), 2)
/// ```
/// 
/// # Capacity and reallocation
/// 
/// The capacity of a vector is the amount of space allocated
/// for future elements. When the capacity is exceeded, the
/// vector reallocates with a larger capacity.
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

impl<T> Vec<T> {
    /// Constructs a new, empty `Vec<T>`.
    /// 
    /// The vector will not allocate until elements are pushed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let vec: Vec<i32> = Vec::new()
    /// ```
    pub fn new() -> Self {
        Self { ptr: null_mut(), len: 0, cap: 0, _marker: PhantomData }
    }
    
    /// Appends an element to the back of the vector.
    /// 
    /// # Panics
    /// 
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut vec = vec![1, 2]
    /// vec.push(3)
    /// assert_eq!(vec, [1, 2, 3])
    /// ```
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow()
        }
        unsafe {
            self.ptr.add(self.len).write(value)
        }
        self.len += 1
    }
}
```

---

## Testing Standards

### Test Categories

The project requires multiple categories of tests to ensure quality.

**Unit Tests** test individual functions and methods in isolation. They should be placed in the same file as the code they test using a `#[cfg(test)]` module. Unit tests should cover all code paths including edge cases and error conditions.

```axiom
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_push() {
        let mut v = Vec::new()
        v.push(1)
        v.push(2)
        assert_eq!(v.len(), 2)
        assert_eq!(v[0], 1)
        assert_eq!(v[1], 2)
    }
    
    #[test]
    fn test_vec_push_capacity() {
        let mut v = Vec::with_capacity(1)
        v.push(1)
        assert_eq!(v.capacity(), 1)
        v.push(2)  // Should reallocate
        assert!(v.capacity() > 1)
    }
}
```

**Integration Tests** test multiple components working together. They should be placed in the `tests/` directory and test realistic usage scenarios. Integration tests should verify that components interact correctly.

**Property-Based Tests** use random inputs to find edge cases. They should define properties that should hold for all inputs and use a testing framework to generate and check inputs.

```axiom
use std::test::quickcheck

#[quickcheck]
fn prop_vec_push_len(items: Vec<i32>) {
    let mut v = Vec::new()
    for &item in &items {
        v.push(item)
    }
    assert_eq!(v.len(), items.len())
}

#[quickcheck]
fn prop_vec_reverse_twice(items: Vec<i32>) {
    let mut v = items.clone()
    v.reverse()
    v.reverse()
    assert_eq!(v, items)
}
```

**Benchmarks** measure performance of critical operations. They should be placed in a `benches/` directory and measure operations that are performance-sensitive. Benchmarks should run enough iterations to produce stable measurements.

```axiom
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

## Communication Guidelines

### Issue Tracking

When creating issues, agents must provide clear and complete information. Issue titles should be concise but descriptive. Issue descriptions should include the problem or feature request, steps to reproduce (for bugs), expected and actual behavior, environment information, and any relevant code or error messages.

### Code Review

When reviewing code, agents should focus on correctness, design, and maintainability. Reviews should be constructive and specific, explaining what should change and why. Nitpicks should be labeled as such and not block approval.

When receiving code review feedback, agents should address all comments. Disagreements should be discussed respectfully. Changes should be made promptly to avoid blocking progress.

### Documentation Contributions

When contributing documentation, agents should ensure accuracy, completeness, and clarity. Documentation should be written for the target audience, whether beginners or experts. Examples should be tested to ensure they work correctly.

---

## Best Practices Summary

1. **Understand the philosophy** - All contributions must align with Axiom's core principles
2. **Start small** - Begin with documentation, tests, or bug fixes before tackling large features
3. **Test thoroughly** - Every contribution needs tests covering all functionality
4. **Document completely** - Public APIs must have documentation
5. **Communicate clearly** - Issues, PRs, and reviews should be clear and constructive
6. **Follow the style guide** - Code should be formatted consistently
7. **Review your own code** - Before submitting, review your changes as if reviewing someone else's
8. **Be responsive** - Address feedback promptly and thoroughly
9. **Learn from feedback** - Use code review as an opportunity to improve
10. **Ask for help** - When stuck, ask questions rather than guessing

---

## Contact and Resources

- **GitHub Repository**: https://github.com/axiom-lang/axiom
- **Documentation**: https://axiom-lang.org/docs
- **Discord**: https://discord.gg/axiom-lang
- **Forum**: https://forum.axiom-lang.org

For questions about these guidelines or the contribution process, please reach out through the channels above.

---

*This document is maintained by the Axiom Core Team. Last updated: 2024*
