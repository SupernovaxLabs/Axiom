# Axiom Technical Implementation Guide - Part 2

## Borrow Checker, IR, Optimization, Code Generation & Interpreter

---

# Part 4: Borrow Checker Implementation

## 4.1 Ownership and Borrowing Concepts

The borrow checker ensures memory safety by enforcing three key rules:

1. **Single Owner Rule**: Each value has exactly one owner
2. **Borrowing Rule**: Multiple immutable borrows OR one mutable borrow
3. **Lifetime Rule**: References must not outlive their referent

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         OWNERSHIP MODEL                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐      MOVE        ┌─────────────┐                         │
│   │   Owner A   │ ───────────────▶ │   Owner B   │                         │
│   │             │                  │             │                         │
│   │  owns value │                  │  owns value │                         │
│   └─────────────┘                  └─────────────┘                         │
│         │                                                                   │
│         │ BORROW                                                            │
│         ▼                                                                   │
│   ┌─────────────┐                                                          │
│   │  Reference  │                                                          │
│   │   &T or     │  ← Cannot outlive owner                                  │
│   │   &mut T    │                                                          │
│   └─────────────┘                                                          │
│                                                                             │
│   RULES:                                                                    │
│   ┌─────────────────────────────────────────────────────┐                  │
│   │  let x = String::from("hello")   // x owns          │                  │
│   │  let y = x                       // x moved to y     │                  │
│   │  // x is INVALID                                    │                  │
│   │                                                      │                  │
│   │  let a = String::from("world")  // a owns           │                  │
│   │  let r1 = &a                    // immutable borrow  │                  │
│   │  let r2 = &a                    // OK: multiple &T   │                  │
│   │  // a is VALID, r1 and r2 are VALID                 │                  │
│   │                                                      │                  │
│   │  let b = String::from("test")   // b owns           │                  │
│   │  let m1 = &mut b                // mutable borrow    │                  │
│   │  // let m2 = &mut b   // ERROR: second &mut         │                  │
│   │  // let r = &b        // ERROR: & and &mut conflict │                  │
│   └─────────────────────────────────────────────────────┘                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 4.2 Borrow Checker Implementation

```axiom
// src/compiler/borrowck/borrowck.ax

/// Borrow Checker - verifies memory safety
pub struct BorrowChecker {
    /// Control flow graph
    cfg: ControlFlowGraph,
    
    /// Active loans at each program point
    loans: Vec<Loan>,
    
    /// Move tracking
    moves: MoveTracker,
    
    /// Lifetime context
    lifetimes: LifetimeContext,
    
    /// Diagnostic messages
    diagnostics: Vec<Diagnostic>,
    
    /// Current function being checked
    current_function: Option<FunctionId>,
}

/// A loan represents a borrow
pub struct Loan {
    /// Unique loan ID
    pub id: LoanId,
    
    /// Path being borrowed
    pub path: Path,
    
    /// Kind of borrow
    pub kind: BorrowKind,
    
    /// Lifetime/region of the borrow
    pub region: Region,
    
    /// Where the borrow occurs
    pub location: Location,
    
    /// Where the borrow expires
    pub expiry: Option<Location>,
}

/// Kind of borrow
pub enum BorrowKind {
    /// Shared immutable borrow (&T)
    Shared,
    
    /// Mutable borrow (&mut T)
    Mutable,
    
    /// Shallow borrow (for pattern matching)
    Shallow,
}

/// Path represents a memory location
pub enum Path {
    /// Local variable
    Local(LocalId),
    
    /// Field access
    Field(Box<Path>, FieldId),
    
    /// Index access
    Index(Box<Path>, usize),
    
    /// Dereference
    Deref(Box<Path>),
}

/// Location in the control flow graph
pub struct Location {
    pub block: BasicBlockId,
    pub statement_index: usize,
}

impl BorrowChecker {
    /// Check a function for borrow errors
    pub fn check_function(&mut self, func: &Function) -> Result!() {
        self.current_function = Some(func.id);
        
        // Build control flow graph
        self.cfg = self.build_cfg(func);
        
        // Find all loans and moves
        self.collect_loans_and_moves(func)?;
        
        // Build loan dominator tree
        self.build_loan_dominators();
        
        // Dataflow analysis for loan liveness
        let loan_liveness = self.compute_loan_liveness();
        
        // Check for conflicts
        self.check_conflicts(&loan_liveness)?;
        
        // Check moves
        self.check_moves(func)?;
        
        Ok(())
    }
    
    /// Collect all loans and moves in a function
    fn collect_loans_and_moves(&mut self, func: &Function) -> Result!() {
        for (block_id, block) in func.body.blocks.iter() {
            for (stmt_idx, stmt) in block.statements.iter().enumerate() {
                let location = Location { block: *block_id, statement_index: stmt_idx };
                self.collect_statement_loans(stmt, location)?;
            }
        }
        Ok(())
    }
    
    /// Collect loans from a statement
    fn collect_statement_loans(&mut self, stmt: &Statement, location: Location) -> Result!() {
        match stmt {
            Statement::Let(let_stmt) => {
                if let Some(init) = &let_stmt.init {
                    self.collect_expr_loans(init, location)?;
                }
            }
            Statement::Expr(expr) => {
                self.collect_expr_loans(expr, location)?;
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Collect loans from an expression
    fn collect_expr_loans(&mut self, expr: &Expr, location: Location) -> Result!() {
        match expr {
            Expr::Reference(ref_expr) => {
                // Create a new loan
                let kind = if ref_expr.mutability == Mutability::Mutable {
                    BorrowKind::Mutable
                } else {
                    BorrowKind::Shared
                };
                
                let path = self.expr_to_path(&ref_expr.operand)?;
                let region = self.fresh_region();
                
                let loan = Loan {
                    id: LoanId::new(),
                    path,
                    kind,
                    region,
                    location,
                    expiry: None,
                };
                
                self.loans.push(loan);
            }
            
            Expr::Binary(binary) => {
                self.collect_expr_loans(&binary.left, location)?;
                self.collect_expr_loans(&binary.right, location)?;
            }
            
            Expr::Call(call) => {
                self.collect_expr_loans(&call.func, location)?;
                for arg in &call.args {
                    self.collect_expr_loans(arg, location)?;
                }
            }
            
            Expr::Assign(assign) => {
                self.collect_expr_loans(&assign.target, location)?;
                self.collect_expr_loans(&assign.value, location)?;
                
                // Track potential move
                self.moves.record_move(&assign.value, location);
            }
            
            // ... other expressions
            _ => {}
        }
        Ok(())
    }
    
    /// Compute loan liveness using dataflow analysis
    fn compute_loan_liveness(&self) -> HashMap<LoanId, Vec<Location>> {
        let mut live_loans: HashMap<LoanId, HashSet<Location>> = HashMap::new();
        
        // For each loan, compute where it is live
        for loan in &self.loans {
            let mut live_at = HashSet::new();
            
            // Start from loan location
            live_at.insert(loan.location);
            
            // Propagate through CFG until expiry
            self.propagate_loan_liveness(loan, &mut live_at);
            
            live_loans.insert(loan.id, live_at);
        }
        
        live_loans
    }
    
    /// Check for conflicting loans
    fn check_conflicts(&mut self, loan_liveness: &HashMap<LoanId, Vec<Location>>) -> Result!() {
        for loan1 in &self.loans {
            for loan2 in &self.loans {
                if loan1.id >= loan2.id {
                    continue;
                }
                
                // Check if loans overlap
                let overlap = self.loans_overlap(loan1, loan2, loan_liveness);
                
                if overlap {
                    // Check if conflict
                    if self.loans_conflict(loan1, loan2) {
                        self.diagnostics.push(Diagnostic::error(
                            loan1.location.span.merge(loan2.location.span),
                            format!(
                                "cannot borrow `{}` as {} because it is also borrowed as {}",
                                self.path_to_string(&loan1.path),
                                self.borrow_kind_str(&loan1.kind),
                                self.borrow_kind_str(&loan2.kind)
                            )
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Check if two loans conflict
    fn loans_conflict(&self, loan1: &Loan, loan2: &Loan) -> bool {
        // Same path or overlapping paths
        if !self.paths_overlap(&loan1.path, &loan2.path) {
            return false;
        }
        
        // Mutable borrows conflict with everything
        match (&loan1.kind, &loan2.kind) {
            (BorrowKind::Mutable, _) | (_, BorrowKind::Mutable) => true,
            (BorrowKind::Shared, BorrowKind::Shared) => false, // Multiple shared OK
            _ => false,
        }
    }
}

/// Move tracker
pub struct MoveTracker {
    /// Moves at each location
    moves: Vec<Move>,
    
    /// Uninitialized locals
    uninitialized: HashSet<LocalId>,
}

pub struct Move {
    pub path: Path,
    pub from: Location,
    pub to: Option<Path>,
}

impl MoveTracker {
    /// Record a move
    pub fn record_move(&mut self, expr: &Expr, location: Location) {
        if let Expr::Ident(ident) = expr {
            // Moving a local variable
            self.moves.push(Move {
                path: Path::Local(ident.id),
                from: location,
                to: None,
            });
        }
    }
    
    /// Check if a path is moved at a location
    pub fn is_moved(&self, path: &Path, location: &Location) -> bool {
        for m in &self.moves {
            if self.paths_equal(&m.path, path) && m.from < *location {
                return true;
            }
        }
        false
    }
}
```

---

# Part 5: Intermediate Representation (AIR)

## 5.1 AIR Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    AXIOM INTERMEDIATE REPRESENTATION (AIR)                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PROPERTIES:                                                                │
│  • Typed - Every value has a known type                                     │
│  • SSA Form - Each variable assigned once                                   │
│  • Explicit Control Flow - No implicit jumps                                │
│  • Explicit Memory - All memory operations visible                          │
│  • Target Independent - No architecture-specific details                    │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  AIR MODULE STRUCTURE                                                │   │
│  │                                                                      │   │
│  │  module "my_module" {                                                │   │
│  │      functions: [                                                    │   │
│  │          fn main() -> i32 { ... }                                    │   │
│  │          fn helper(x: i32) -> i32 { ... }                            │   │
│  │      ]                                                               │   │
│  │      globals: [                                                      │   │
│  │          global COUNTER: i32 = 0                                     │   │
│  │      ]                                                               │   │
│  │      types: [ ... ]                                                  │   │
│  │  }                                                                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 5.2 AIR Data Structures

```axiom
// src/compiler/air/mod.ax

/// AIR Module
pub struct AirModule {
    pub name: string,
    pub functions: Vec<AirFunction>,
    pub globals: Vec<AirGlobal>,
    pub types: Vec<AirType>,
    pub debug_info: Option<DebugInfo>,
}

/// AIR Function
pub struct AirFunction {
    pub id: FunctionId,
    pub name: string,
    pub type_: FunctionType,
    pub params: Vec<AirParam>,
    pub blocks: Vec<BasicBlock>,
    pub locals: Vec<AirLocal>,
    pub linkage: Linkage,
    pub calling_convention: CallingConvention,
}

/// Basic Block
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub is_cleanup: bool,
}

/// Instruction
pub enum Instruction {
    // ========== ARITHMETIC ==========
    /// Binary operation: result = op(lhs, rhs)
    BinOp {
        result: ValueId,
        op: BinOp,
        lhs: ValueId,
        rhs: ValueId,
        type_: AirType,
    },
    
    /// Unary operation: result = op(operand)
    UnaryOp {
        result: ValueId,
        op: UnaryOp,
        operand: ValueId,
        type_: AirType,
    },
    
    // ========== COMPARISON ==========
    /// Comparison: result = cmp(op, lhs, rhs)
    Compare {
        result: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
        type_: AirType,
    },
    
    // ========== MEMORY ==========
    /// Allocate local: result = alloca type
    Alloca {
        result: ValueId,
        type_: AirType,
        size: Option<ValueId>,
    },
    
    /// Load: result = *ptr
    Load {
        result: ValueId,
        ptr: ValueId,
        type_: AirType,
    },
    
    /// Store: *ptr = value
    Store {
        ptr: ValueId,
        value: ValueId,
        type_: AirType,
    },
    
    /// Get element pointer: result = &ptr[indices]
    GetElementPtr {
        result: ValueId,
        base: ValueId,
        indices: Vec<ValueId>,
        type_: AirType,
    },
    
    /// Memory copy
    Memcpy {
        dest: ValueId,
        src: ValueId,
        size: ValueId,
    },
    
    /// Memory set
    Memset {
        dest: ValueId,
        value: ValueId,
        size: ValueId,
    },
    
    // ========== CONTROL ==========
    /// Call function: result = func(args)
    Call {
        result: Option<ValueId>,
        func: Callable,
        args: Vec<ValueId>,
        type_: AirType,
    },
    
    /// Invoke (for exceptions)
    Invoke {
        result: Option<ValueId>,
        func: Callable,
        args: Vec<ValueId>,
        normal: BasicBlockId,
        unwind: BasicBlockId,
        type_: AirType,
    },
    
    // ========== CONVERSIONS ==========
    /// Cast between types
    Cast {
        result: ValueId,
        value: ValueId,
        from_type: AirType,
        to_type: AirType,
    },
    
    /// Zero-extend integer
    ZExt {
        result: ValueId,
        value: ValueId,
        from_type: AirType,
        to_type: AirType,
    },
    
    /// Sign-extend integer
    SExt {
        result: ValueId,
        value: ValueId,
        from_type: AirType,
        to_type: AirType,
    },
    
    /// Truncate integer
    Trunc {
        result: ValueId,
        value: ValueId,
        from_type: AirType,
        to_type: AirType,
    },
    
    /// Float to int
    FloatToInt {
        result: ValueId,
        value: ValueId,
        from_type: AirType,
        to_type: AirType,
    },
    
    /// Int to float
    IntToFloat {
        result: ValueId,
        value: ValueId,
        from_type: AirType,
        to_type: AirType,
    },
    
    // ========== AGGREGATES ==========
    /// Create aggregate: result = { elements }
    Aggregate {
        result: ValueId,
        type_: AirType,
        elements: Vec<ValueId>,
    },
    
    /// Extract element: result = agg[index]
    ExtractElement {
        result: ValueId,
        aggregate: ValueId,
        index: ValueId,
        type_: AirType,
    },
    
    /// Insert element: result = agg with [index] = value
    InsertElement {
        result: ValueId,
        aggregate: ValueId,
        index: ValueId,
        value: ValueId,
        type_: AirType,
    },
    
    /// Extract value: result = agg.field
    ExtractValue {
        result: ValueId,
        aggregate: ValueId,
        field: usize,
        type_: AirType,
    },
    
    /// Insert value: result = agg with .field = value
    InsertValue {
        result: ValueId,
        aggregate: ValueId,
        field: usize,
        value: ValueId,
        type_: AirType,
    },
    
    // ========== OTHER ==========
    /// Debug location
    DebugLoc {
        location: SourceLocation,
    },
    
    /// No-op
    Nop,
}

/// Terminator (ends a basic block)
pub enum Terminator {
    /// Return from function
    Return {
        value: Option<ValueId>,
    },
    
    /// Unconditional branch
    Branch {
        target: BasicBlockId,
    },
    
    /// Conditional branch
    CondBranch {
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    },
    
    /// Switch statement
    Switch {
        value: ValueId,
        cases: Vec<(Constant, BasicBlockId)>,
        default: BasicBlockId,
    },
    
    /// Unreachable code
    Unreachable,
    
    /// Resume unwinding
    Resume {
        value: ValueId,
    },
}

/// Binary operations
pub enum BinOp {
    Add, Sub, Mul, Div, Mod, Pow,
    Shl, Shr, UShr,
    BitAnd, BitOr, BitXor,
}

/// Unary operations
pub enum UnaryOp {
    Neg, Not, BitNot,
}

/// Comparison operations
pub enum CompareOp {
    Equal, NotEqual,
    Less, LessEqual,
    Greater, GreaterEqual,
}

/// Callable
pub enum Callable {
    Function(FunctionId),
    External(string),
    Indirect(ValueId),
}

/// Value representation
pub struct Value {
    pub id: ValueId,
    pub type_: AirType,
    pub kind: ValueKind,
}

pub enum ValueKind {
    /// Constant value
    Constant(Constant),
    
    /// Function parameter
    Param(usize),
    
    /// Instruction result
    Instruction(usize),
    
    /// Basic block argument
    BlockArg(BasicBlockId, usize),
}

/// Constant values
pub enum Constant {
    Int(i128, IntTy),
    Float(f64, FloatTy),
    Bool(bool),
    String(string),
    Null,
    Undef,
    Array(Vec<Constant>),
    Struct(Vec<Constant>),
}

/// AIR Types
pub enum AirType {
    Void,
    Int(IntTy),
    Float(FloatTy),
    Bool,
    Pointer(Box<AirType>),
    Array(Box<AirType>, usize),
    Slice(Box<AirType>),
    Struct(StructId, Vec<AirType>),
    Enum(EnumId, Vec<AirType>),
    Function(Box<FunctionType>),
    Tuple(Vec<AirType>),
    Reference(Box<AirType>),
    Optional(Box<AirType>),
}
```

## 5.3 AIR Builder

```axiom
// src/compiler/air/builder.ax

/// AIR Builder - constructs AIR from typed AST
pub struct AirBuilder {
    /// Current function being built
    current_function: Option<AirFunction>,
    
    /// Current basic block
    current_block: Option<BasicBlockId>,
    
    /// Value counter
    next_value: ValueId,
    
    /// Block counter
    next_block: BasicBlockId,
    
    /// Local variable mapping
    locals: HashMap<LocalId, ValueId>,
    
    /// Break/continue targets
    loop_targets: Vec<LoopTarget>,
}

struct LoopTarget {
    continue_block: BasicBlockId,
    break_block: BasicBlockId,
}

impl AirBuilder {
    /// Create new AIR builder
    pub fn new() -> Self {
        Self {
            current_function: None,
            current_block: None,
            next_value: ValueId(0),
            next_block: BasicBlockId(0),
            locals: HashMap::new(),
            loop_targets: Vec::new(),
        }
    }
    
    /// Create a new basic block
    pub fn create_block(&mut self) -> BasicBlockId {
        let id = self.next_block;
        self.next_block = BasicBlockId(id.0 + 1);
        
        if let Some(func) = &mut self.current_function {
            func.blocks.push(BasicBlock {
                id,
                instructions: Vec::new(),
                terminator: Terminator::Unreachable,
                is_cleanup: false,
            });
        }
        
        id
    }
    
    /// Switch to a basic block
    pub fn position_at_end(&mut self, block: BasicBlockId) {
        self.current_block = Some(block);
    }
    
    /// Create a new value
    fn fresh_value(&mut self, type_: AirType) -> ValueId {
        let id = self.next_value;
        self.next_value = ValueId(id.0 + 1);
        id
    }
    
    /// Build binary operation
    pub fn build_binop(
        &mut self,
        op: BinOp,
        lhs: ValueId,
        rhs: ValueId,
        type_: AirType,
    ) -> ValueId {
        let result = self.fresh_value(type_.clone());
        
        self.append_instruction(Instruction::BinOp {
            result,
            op,
            lhs,
            rhs,
            type_,
        });
        
        result
    }
    
    /// Build comparison
    pub fn build_compare(
        &mut self,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
        type_: AirType,
    ) -> ValueId {
        let result = self.fresh_value(AirType::Bool);
        
        self.append_instruction(Instruction::Compare {
            result,
            op,
            lhs,
            rhs,
            type_,
        });
        
        result
    }
    
    /// Build load
    pub fn build_load(&mut self, ptr: ValueId, type_: AirType) -> ValueId {
        let result = self.fresh_value(type_.clone());
        
        self.append_instruction(Instruction::Load {
            result,
            ptr,
            type_,
        });
        
        result
    }
    
    /// Build store
    pub fn build_store(&mut self, ptr: ValueId, value: ValueId, type_: AirType) {
        self.append_instruction(Instruction::Store {
            ptr,
            value,
            type_,
        });
    }
    
    /// Build call
    pub fn build_call(
        &mut self,
        func: Callable,
        args: Vec<ValueId>,
        return_type: AirType,
    ) -> Option<ValueId> {
        if return_type == AirType::Void {
            self.append_instruction(Instruction::Call {
                result: None,
                func,
                args,
                type_: return_type,
            });
            None
        } else {
            let result = self.fresh_value(return_type.clone());
            self.append_instruction(Instruction::Call {
                result: Some(result),
                func,
                args,
                type_: return_type,
            });
            Some(result)
        }
    }
    
    /// Build conditional branch
    pub fn build_cond_br(
        &mut self,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) {
        self.set_terminator(Terminator::CondBranch {
            condition,
            then_block,
            else_block,
        });
    }
    
    /// Build unconditional branch
    pub fn build_br(&mut self, target: BasicBlockId) {
        self.set_terminator(Terminator::Branch { target });
    }
    
    /// Build return
    pub fn build_return(&mut self, value: Option<ValueId>) {
        self.set_terminator(Terminator::Return { value });
    }
    
    /// Lower AST expression to AIR
    pub fn lower_expr(&mut self, expr: &Expr) -> Result!ValueId {
        match expr {
            Expr::Literal(lit, _) => self.lower_literal(lit),
            
            Expr::Ident(ident) => {
                // Look up local variable
                let value = self.locals.get(&ident.id)
                    .ok_or_else(|| Error::undefined_variable(&ident.name))?;
                Ok(*value)
            }
            
            Expr::Binary(binary) => self.lower_binary(binary),
            
            Expr::Unary(unary) => self.lower_unary(unary),
            
            Expr::Call(call) => self.lower_call(call),
            
            Expr::If(if_expr) => self.lower_if(if_expr),
            
            Expr::Block(block) => self.lower_block(block),
            
            Expr::Assign(assign) => self.lower_assign(assign),
            
            Expr::Reference(ref_expr) => self.lower_reference(ref_expr),
            
            Expr::Field(field) => self.lower_field(field),
            
            // ... other expressions
            _ => Err(Error::not_implemented("expression")),
        }
    }
    
    /// Lower binary expression
    fn lower_binary(&mut self, binary: &BinaryExpr) -> Result!ValueId {
        let lhs = self.lower_expr(&binary.left)?;
        let rhs = self.lower_expr(&binary.right)?;
        
        let type_ = self.value_type(lhs);
        
        let op = match binary.op {
            BinaryOp::Add => BinOp::Add,
            BinaryOp::Sub => BinOp::Sub,
            BinaryOp::Mul => BinOp::Mul,
            BinaryOp::Div => BinOp::Div,
            BinaryOp::Mod => BinOp::Mod,
            BinaryOp::BitAnd => BinOp::BitAnd,
            BinaryOp::BitOr => BinOp::BitOr,
            BinaryOp::BitXor => BinOp::BitXor,
            BinaryOp::Shl => BinOp::Shl,
            BinaryOp::Shr => BinOp::Shr,
            _ => return Err(Error::invalid_binop(binary.op)),
        };
        
        Ok(self.build_binop(op, lhs, rhs, type_))
    }
    
    /// Lower if expression
    fn lower_if(&mut self, if_expr: &IfExpr) -> Result!ValueId {
        // Lower condition
        let condition = self.lower_expr(&if_expr.condition)?;
        
        // Create blocks
        let then_block = self.create_block();
        let else_block = self.create_block();
        let merge_block = self.create_block();
        
        // Branch on condition
        self.build_cond_br(condition, then_block, else_block);
        
        // Lower then block
        self.position_at_end(then_block);
        let then_value = self.lower_block(&if_expr.then_block)?;
        let then_type = self.value_type(then_value);
        self.build_br(merge_block);
        let then_end = self.current_block;
        
        // Lower else block
        self.position_at_end(else_block);
        let else_value = if let Some(else_expr) = &if_expr.else_block {
            Some(self.lower_expr(else_expr)?)
        } else {
            None
        };
        self.build_br(merge_block);
        let else_end = self.current_block;
        
        // Merge block with phi
        self.position_at_end(merge_block);
        
        // Create phi for values
        let result = self.fresh_value(then_type);
        
        // Add phi instruction (represented as block arguments)
        // ...
        
        Ok(result)
    }
    
    /// Append instruction to current block
    fn append_instruction(&mut self, instruction: Instruction) {
        if let Some(block_id) = self.current_block {
            if let Some(func) = &mut self.current_function {
                if let Some(block) = func.blocks.iter_mut().find(|b| b.id == block_id) {
                    block.instructions.push(instruction);
                }
            }
        }
    }
    
    /// Set terminator for current block
    fn set_terminator(&mut self, terminator: Terminator) {
        if let Some(block_id) = self.current_block {
            if let Some(func) = &mut self.current_function {
                if let Some(block) = func.blocks.iter_mut().find(|b| b.id == block_id) {
                    block.terminator = terminator;
                }
            }
        }
    }
}
```

---

# Part 6: Optimization Passes

## 6.1 Optimization Pipeline

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      OPTIMIZATION PIPELINE                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    EARLY OPTIMIZATIONS                               │  │
│   │                                                                      │  │
│   │   • Constant Propagation     • Dead Code Elimination                │  │
│   │   • Constant Folding         • Simplify CFG                         │  │
│   │   • Value Numbering          • Promote Memory to Registers          │  │
│   │                                                                      │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                   │                                         │
│                                   ▼                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    INLINE EXPANSION                                  │  │
│   │                                                                      │  │
│   │   • Heuristic-based inlining  • Cross-function optimization         │  │
│   │   • Force inline attributes   • Partial inlining                    │  │
│   │                                                                      │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                   │                                         │
│                                   ▼                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    LOOP OPTIMIZATIONS                                │  │
│   │                                                                      │  │
│   │   • Loop Invariant Code Motion  • Loop Unrolling                    │  │
│   │   • Induction Variable Strength Reduction                       │  │
│   │   • Loop Fusion                 • Loop Interchange                  │  │
│   │                                                                      │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                   │                                         │
│                                   ▼                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    ADVANCED OPTIMIZATIONS                            │  │
│   │                                                                      │  │
│   │   • Escape Analysis           • SIMD Vectorization                  │  │
│   │   • Tail Call Optimization    • Autovectorization                   │  │
│   │   • SROA (Scalar Replacement) • LICM (Load-Store Motion)            │  │
│   │                                                                      │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                   │                                         │
│                                   ▼                                         │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                    LATE OPTIMIZATIONS                                │  │
│   │                                                                      │  │
│   │   • Dead Store Elimination    • Branch Folding                      │  │
│   │   • Common Subexpr Elim       • Code Layout                         │  │
│   │   • Tail Duplication          • Basic Block Reordering              │  │
│   │                                                                      │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 6.2 Optimization Pass Implementation

```axiom
// src/compiler/opt/passes.ax

/// Optimization pass trait
pub trait OptimizationPass {
    /// Name of the pass
    fn name(&self) -> &str;
    
    /// Run the pass on a module
    fn run(&self, module: &mut AirModule);
    
    /// Whether this pass modifies the CFG
    fn modifies_cfg(&self) -> bool { false }
}

/// Constant folding pass
pub struct ConstantFolding;

impl OptimizationPass for ConstantFolding {
    fn name(&self) -> &str { "Constant Folding" }
    
    fn run(&self, module: &mut AirModule) {
        for func in &mut module.functions {
            for block in &mut func.blocks {
                for instr in &mut block.instructions {
                    self.fold_instruction(instr);
                }
            }
        }
    }
}

impl ConstantFolding {
    fn fold_instruction(&self, instr: &mut Instruction) {
        match instr {
            Instruction::BinOp { result, op, lhs, rhs, type_ } => {
                // Check if operands are constants
                if let (Some(lhs_const), Some(rhs_const)) = 
                    (self.get_constant(lhs), self.get_constant(rhs)) 
                {
                    if let Some(folded) = self.fold_binop(op, lhs_const, rhs_const, type_) {
                        // Replace with constant
                        *instr = Instruction::Aggregate {
                            result: *result,
                            type_: type_.clone(),
                            elements: vec![],
                        };
                        // Mark value as constant
                        self.set_constant(*result, folded);
                    }
                }
            }
            
            Instruction::Compare { result, op, lhs, rhs, type_ } => {
                if let (Some(lhs_const), Some(rhs_const)) = 
                    (self.get_constant(lhs), self.get_constant(rhs))
                {
                    if let Some(folded) = self.fold_compare(op, lhs_const, rhs_const, type_) {
                        self.set_constant(*result, folded);
                    }
                }
            }
            
            _ => {}
        }
    }
    
    fn fold_binop(&self, op: &BinOp, lhs: Constant, rhs: Constant, type_: &AirType) -> Option<Constant> {
        match (op, lhs, rhs) {
            // Integer operations
            (BinOp::Add, Constant::Int(a, _), Constant::Int(b, ty)) => 
                Some(Constant::Int(a.wrapping_add(b), ty)),
            (BinOp::Sub, Constant::Int(a, _), Constant::Int(b, ty)) => 
                Some(Constant::Int(a.wrapping_sub(b), ty)),
            (BinOp::Mul, Constant::Int(a, _), Constant::Int(b, ty)) => 
                Some(Constant::Int(a.wrapping_mul(b), ty)),
            (BinOp::Div, Constant::Int(a, _), Constant::Int(b, ty)) if b != 0 => 
                Some(Constant::Int(a / b, ty)),
            
            // Float operations
            (BinOp::Add, Constant::Float(a, ty), Constant::Float(b, _)) => 
                Some(Constant::Float(a + b, ty)),
            (BinOp::Sub, Constant::Float(a, ty), Constant::Float(b, _)) => 
                Some(Constant::Float(a - b, ty)),
            (BinOp::Mul, Constant::Float(a, ty), Constant::Float(b, _)) => 
                Some(Constant::Float(a * b, ty)),
            
            _ => None,
        }
    }
    
    fn fold_compare(&self, op: &CompareOp, lhs: Constant, rhs: Constant, type_: &AirType) -> Option<Constant> {
        match (op, lhs, rhs) {
            (CompareOp::Equal, Constant::Int(a, _), Constant::Int(b, _)) => 
                Some(Constant::Bool(a == b)),
            (CompareOp::NotEqual, Constant::Int(a, _), Constant::Int(b, _)) => 
                Some(Constant::Bool(a != b)),
            (CompareOp::Less, Constant::Int(a, _), Constant::Int(b, _)) => 
                Some(Constant::Bool(a < b)),
            (CompareOp::LessEqual, Constant::Int(a, _), Constant::Int(b, _)) => 
                Some(Constant::Bool(a <= b)),
            (CompareOp::Greater, Constant::Int(a, _), Constant::Int(b, _)) => 
                Some(Constant::Bool(a > b)),
            (CompareOp::GreaterEqual, Constant::Int(a, _), Constant::Int(b, _)) => 
                Some(Constant::Bool(a >= b)),
            _ => None,
        }
    }
}

/// Dead code elimination pass
pub struct DeadCodeElimination;

impl OptimizationPass for DeadCodeElimination {
    fn name(&self) -> &str { "Dead Code Elimination" }
    
    fn run(&self, module: &mut AirModule) {
        for func in &mut module.functions {
            self.eliminate_dead_code(func);
        }
    }
}

impl DeadCodeElimination {
    fn eliminate_dead_code(&self, func: &mut AirFunction) {
        // Compute which values are used
        let mut used = HashSet::new();
        
        // Mark values used by terminators
        for block in &func.blocks {
            match &block.terminator {
                Terminator::Return { value } => {
                    if let Some(v) = value { used.insert(*v); }
                }
                Terminator::CondBranch { condition, .. } => {
                    used.insert(*condition);
                }
                Terminator::Switch { value, .. } => {
                    used.insert(*value);
                }
                _ => {}
            }
        }
        
        // Propagate usage backwards
        let mut changed = true;
        while changed {
            changed = false;
            
            for block in &func.blocks {
                for instr in block.instructions.iter().rev() {
                    for value in self.get_used_values(instr) {
                        if used.insert(value) {
                            changed = true;
                        }
                    }
                }
            }
        }
        
        // Remove unused instructions
        for block in &mut func.blocks {
            block.instructions.retain(|instr| {
                self.get_defined_values(instr).iter().any(|v| used.contains(v))
            });
        }
    }
    
    fn get_used_values(&self, instr: &Instruction) -> Vec<ValueId> {
        match instr {
            Instruction::BinOp { lhs, rhs, .. } => vec![*lhs, *rhs],
            Instruction::Compare { lhs, rhs, .. } => vec![*lhs, *rhs],
            Instruction::Load { ptr, .. } => vec![*ptr],
            Instruction::Store { ptr, value, .. } => vec![*ptr, *value],
            Instruction::Call { args, .. } => args.clone(),
            Instruction::Cast { value, .. } => vec![*value],
            Instruction::ExtractElement { aggregate, index, .. } => vec![*aggregate, *index],
            Instruction::InsertElement { aggregate, index, value, .. } => 
                vec![*aggregate, *index, *value],
            _ => vec![],
        }
    }
    
    fn get_defined_values(&self, instr: &Instruction) -> Vec<ValueId> {
        match instr {
            Instruction::BinOp { result, .. } => vec![*result],
            Instruction::Compare { result, .. } => vec![*result],
            Instruction::Load { result, .. } => vec![*result],
            Instruction::Call { result: Some(result), .. } => vec![*result],
            _ => vec![],
        }
    }
}

/// Inlining pass
pub struct Inlining {
    /// Maximum inline depth
    max_depth: usize,
    
    /// Cost threshold for inlining
    cost_threshold: usize,
}

impl OptimizationPass for Inlining {
    fn name(&self) -> &str { "Function Inlining" }
    
    fn run(&self, module: &mut AirModule) {
        // Build call graph
        let call_graph = self.build_call_graph(module);
        
        // Find functions to inline
        let inline_candidates = self.find_inline_candidates(module, &call_graph);
        
        // Inline functions
        for (caller_id, callsite_idx, callee_id) in inline_candidates {
            self.inline_function(module, caller_id, callsite_idx, callee_id);
        }
    }
}

impl Inlining {
    fn inline_function(
        &self,
        module: &mut AirModule,
        caller_id: FunctionId,
        callsite_idx: usize,
        callee_id: FunctionId,
    ) {
        // Get caller and callee
        let caller = &mut module.functions.iter_mut().find(|f| f.id == caller_id);
        let callee = module.functions.iter().find(|f| f.id == callee_id);
        
        let (Some(caller), Some(callee)) = (caller, callee) else { return };
        
        // Find the callsite
        for block in &mut caller.blocks {
            for instr in &mut block.instructions {
                if let Instruction::Call { func: Callable::Function(fid), .. } = instr {
                    if *fid == callee_id {
                        // Clone callee blocks
                        // Map callee values to new values in caller
                        // Replace call with inlined body
                        // ...
                    }
                }
            }
        }
    }
    
    fn compute_inline_cost(&self, func: &AirFunction) -> usize {
        let mut cost = 0;
        
        for block in &func.blocks {
            cost += block.instructions.len();
        }
        
        cost
    }
}

/// Loop optimization pass
pub struct LoopOptimization;

impl OptimizationPass for LoopOptimization {
    fn name(&self) -> &str { "Loop Optimization" }
    
    fn run(&self, module: &mut AirModule) {
        for func in &mut module.functions {
            // Find natural loops
            let loops = self.find_loops(func);
            
            for loop_info in loops {
                // Loop invariant code motion
                self.licm(func, &loop_info);
                
                // Induction variable optimization
                self.optimize_induction_vars(func, &loop_info);
            }
        }
    }
}

impl LoopOptimization {
    fn find_loops(&self, func: &AirFunction) -> Vec<LoopInfo> {
        let mut loops = Vec::new();
        
        // Build dominator tree
        let dom_tree = self.build_dominator_tree(func);
        
        // Find back edges (indicates loops)
        for block in &func.blocks {
            match &block.terminator {
                Terminator::Branch { target } => {
                    if dom_tree.dominates(*target, block.id) {
                        // Back edge found
                        loops.push(self.analyze_loop(func, block.id, *target));
                    }
                }
                Terminator::CondBranch { then_block, else_block, .. } => {
                    for target in [then_block, else_block] {
                        if dom_tree.dominates(*target, block.id) {
                            loops.push(self.analyze_loop(func, block.id, *target));
                        }
                    }
                }
                _ => {}
            }
        }
        
        loops
    }
    
    /// Loop invariant code motion
    fn licm(&self, func: &mut AirFunction, loop_info: &LoopInfo) {
        // Find invariant instructions
        let mut invariant = Vec::new();
        
        for block_id in &loop_info.blocks {
            let block = func.blocks.iter().find(|b| b.id == *block_id).unwrap();
            
            for (idx, instr) in block.instructions.iter().enumerate() {
                if self.is_loop_invariant(instr, loop_info) {
                    invariant.push((*block_id, idx));
                }
            }
        }
        
        // Move invariant instructions to preheader
        for (block_id, idx) in invariant.into_iter().rev() {
            // Move instruction to preheader
            // ...
        }
    }
    
    fn is_loop_invariant(&self, instr: &Instruction, loop_info: &LoopInfo) -> bool {
        match instr {
            Instruction::BinOp { lhs, rhs, .. } => {
                // Both operands must be defined outside the loop
                !loop_info.defines(*lhs) && !loop_info.defines(*rhs)
            }
            Instruction::Compare { lhs, rhs, .. } => {
                !loop_info.defines(*lhs) && !loop_info.defines(*rhs)
            }
            Instruction::Load { ptr, .. } => {
                // Pointer must be defined outside loop and no stores in loop
                !loop_info.defines(*ptr) && !loop_info.has_stores()
            }
            _ => false,
        }
    }
}

/// Optimization pipeline
pub struct OptimizationPipeline {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl OptimizationPipeline {
    pub fn new(level: OptimizationLevel) -> Self {
        let mut passes: Vec<Box<dyn OptimizationPass>> = Vec::new();
        
        // Early passes (all levels)
        passes.push(Box::new(ConstantFolding));
        passes.push(Box::new(DeadCodeElimination));
        
        if level >= OptimizationLevel::Basic {
            passes.push(Box::new(Inlining::new(3, 100)));
        }
        
        if level >= OptimizationLevel::Standard {
            passes.push(Box::new(LoopOptimization));
        }
        
        if level >= OptimizationLevel::Aggressive {
            passes.push(Box::new(SROA));
            passes.push(Box::new(Vectorization));
        }
        
        Self { passes }
    }
    
    pub fn run(&self, module: &mut AirModule) {
        // Run passes in order
        for pass in &self.passes {
            pass.run(module);
        }
        
        // Run cleanup passes
        DeadCodeElimination.run(module);
    }
}

pub enum OptimizationLevel {
    None,
    Basic,
    Standard,
    Aggressive,
}
```

---

# Part 7: Code Generation

## 7.1 LLVM Backend

```axiom
// src/compiler/codegen/llvm_backend.ax

/// LLVM IR Generator
pub struct LLVMBackend {
    /// LLVM context
    context: llvm::Context,
    
    /// LLVM module
    module: llvm::Module,
    
    /// IR builder
    builder: llvm::Builder,
    
    /// Type converter
    type_converter: TypeConverter,
    
    /// Value mapping
    values: HashMap<ValueId, llvm::Value>,
    
    /// Block mapping
    blocks: HashMap<BasicBlockId, llvm::BasicBlock>,
    
    /// Debug info builder
    debug_builder: Option<DebugInfoBuilder>,
}

impl LLVMBackend {
    /// Create new LLVM backend
    pub fn new(module_name: &str) -> Self {
        let context = llvm::Context::new();
        let module = llvm::Module::new(module_name, &context);
        let builder = llvm::Builder::new(&context);
        
        Self {
            context,
            module,
            builder,
            type_converter: TypeConverter::new(),
            values: HashMap::new(),
            blocks: HashMap::new(),
            debug_builder: None,
        }
    }
    
    /// Compile AIR module to LLVM IR
    pub fn compile(&mut self, air_module: &AirModule) -> llvm::Module {
        // Declare globals
        for global in &air_module.globals {
            self.declare_global(global);
        }
        
        // Declare functions
        for func in &air_module.functions {
            self.declare_function(func);
        }
        
        // Compile function bodies
        for func in &air_module.functions {
            self.compile_function(func);
        }
        
        // Verify module
        self.module.verify();
        
        self.module.clone()
    }
    
    /// Declare a function
    fn declare_function(&mut self, func: &AirFunction) {
        // Convert function type
        let fn_type = self.convert_function_type(&func.type_);
        
        // Create function
        let llvm_func = self.module.add_function(&func.name, fn_type);
        
        // Set linkage
        llvm_func.set_linkage(self.convert_linkage(&func.linkage));
        
        // Set calling convention
        llvm_func.set_calling_convention(func.calling_convention as usize);
        
        // Add to mapping
        self.functions.insert(func.id, llvm_func);
    }
    
    /// Compile function body
    fn compile_function(&mut self, func: &AirFunction) {
        let llvm_func = self.functions.get(&func.id).unwrap();
        
        // Create basic blocks
        for block in &func.blocks {
            let llvm_block = self.context.append_basic_block(llvm_func, &format!("bb{}", block.id.0));
            self.blocks.insert(block.id, llvm_block);
        }
        
        // Compile entry block first
        self.builder.position_at_end(self.blocks.get(&func.blocks[0].id).unwrap());
        
        // Allocate local variables
        for local in &func.locals {
            let alloca = self.builder.build_alloca(
                self.convert_type(&local.type_),
                &local.name
            );
            self.values.insert(local.id, alloca);
        }
        
        // Compile blocks
        for block in &func.blocks {
            self.compile_block(func, block);
        }
    }
    
    /// Compile a basic block
    fn compile_block(&mut self, func: &AirFunction, block: &BasicBlock) {
        let llvm_block = self.blocks.get(&block.id).unwrap();
        self.builder.position_at_end(llvm_block);
        
        // Compile instructions
        for instr in &block.instructions {
            self.compile_instruction(func, instr);
        }
        
        // Compile terminator
        self.compile_terminator(&block.terminator);
    }
    
    /// Compile an instruction
    fn compile_instruction(&mut self, func: &AirFunction, instr: &Instruction) {
        match instr {
            Instruction::BinOp { result, op, lhs, rhs, type_ } => {
                let lhs_val = self.values.get(lhs).unwrap();
                let rhs_val = self.values.get(rhs).unwrap();
                
                let llvm_instr = match op {
                    BinOp::Add => self.builder.build_add(*lhs_val, *rhs_val, "add"),
                    BinOp::Sub => self.builder.build_sub(*lhs_val, *rhs_val, "sub"),
                    BinOp::Mul => self.builder.build_mul(*lhs_val, *rhs_val, "mul"),
                    BinOp::Div => self.builder.build_sdiv(*lhs_val, *rhs_val, "div"),
                    BinOp::Shl => self.builder.build_shl(*lhs_val, *rhs_val, "shl"),
                    BinOp::Shr => self.builder.build_ashr(*lhs_val, *rhs_val, "shr"),
                    BinOp::BitAnd => self.builder.build_and(*lhs_val, *rhs_val, "and"),
                    BinOp::BitOr => self.builder.build_or(*lhs_val, *rhs_val, "or"),
                    BinOp::BitXor => self.builder.build_xor(*lhs_val, *rhs_val, "xor"),
                    _ => panic!("Unsupported binop"),
                };
                
                self.values.insert(*result, llvm_instr);
            }
            
            Instruction::Compare { result, op, lhs, rhs, type_ } => {
                let lhs_val = self.values.get(lhs).unwrap();
                let rhs_val = self.values.get(rhs).unwrap();
                
                let pred = match op {
                    CompareOp::Equal => llvm::IntPredicate::EQ,
                    CompareOp::NotEqual => llvm::IntPredicate::NE,
                    CompareOp::Less => llvm::IntPredicate::SLT,
                    CompareOp::LessEqual => llvm::IntPredicate::SLE,
                    CompareOp::Greater => llvm::IntPredicate::SGT,
                    CompareOp::GreaterEqual => llvm::IntPredicate::SGE,
                };
                
                let cmp = self.builder.build_icmp(pred, *lhs_val, *rhs_val, "cmp");
                self.values.insert(*result, cmp);
            }
            
            Instruction::Load { result, ptr, type_ } => {
                let ptr_val = self.values.get(ptr).unwrap();
                let load = self.builder.build_load(*ptr_val, &self.convert_type(type_), "load");
                self.values.insert(*result, load);
            }
            
            Instruction::Store { ptr, value, type_ } => {
                let ptr_val = self.values.get(ptr).unwrap();
                let val = self.values.get(value).unwrap();
                self.builder.build_store(*ptr_val, *val);
            }
            
            Instruction::Call { result, func: callable, args, type_ } => {
                let callee = match callable {
                    Callable::Function(id) => self.functions.get(id).unwrap(),
                    Callable::External(name) => self.module.get_function(name).unwrap(),
                    Callable::Indirect(ptr) => self.values.get(ptr).unwrap(),
                };
                
                let args: Vec<_> = args.iter()
                    .map(|a| *self.values.get(a).unwrap())
                    .collect();
                
                let call = self.builder.build_call(*callee, &args, "call");
                
                if let Some(result) = result {
                    self.values.insert(*result, call);
                }
            }
            
            Instruction::Alloca { result, type_, size } => {
                let ty = self.convert_type(type_);
                let alloca = match size {
                    Some(s) => {
                        let size_val = self.values.get(s).unwrap();
                        self.builder.build_array_alloca(ty, *size_val, "alloca")
                    }
                    None => self.builder.build_alloca(ty, "alloca"),
                };
                self.values.insert(*result, alloca);
            }
            
            Instruction::Cast { result, value, from_type, to_type } => {
                let val = self.values.get(value).unwrap();
                let from_ty = self.convert_type(from_type);
                let to_ty = self.convert_type(to_type);
                
                let cast = self.builder.build_bitcast(*val, to_ty, "cast");
                self.values.insert(*result, cast);
            }
            
            // ... other instructions
            _ => {}
        }
    }
    
    /// Compile a terminator
    fn compile_terminator(&mut self, term: &Terminator) {
        match term {
            Terminator::Return { value } => {
                match value {
                    Some(v) => {
                        let val = self.values.get(v).unwrap();
                        self.builder.build_return(Some(val));
                    }
                    None => {
                        self.builder.build_return_void();
                    }
                }
            }
            
            Terminator::Branch { target } => {
                let block = self.blocks.get(target).unwrap();
                self.builder.build_br(*block);
            }
            
            Terminator::CondBranch { condition, then_block, else_block } => {
                let cond = self.values.get(condition).unwrap();
                let then = self.blocks.get(then_block).unwrap();
                let else_ = self.blocks.get(else_block).unwrap();
                self.builder.build_cond_br(*cond, *then, *else_);
            }
            
            Terminator::Switch { value, cases, default } => {
                let val = self.values.get(value).unwrap();
                let def_block = self.blocks.get(default).unwrap();
                
                let switch = self.builder.build_switch(*val, *def_block, cases.len());
                
                for (constant, target) in cases {
                    let llvm_const = self.convert_constant(constant);
                    let target_block = self.blocks.get(target).unwrap();
                    switch.add_case(llvm_const, *target_block);
                }
            }
            
            Terminator::Unreachable => {
                self.builder.build_unreachable();
            }
        }
    }
    
    /// Convert AIR type to LLVM type
    fn convert_type(&self, type_: &AirType) -> llvm::Type {
        match type_ {
            AirType::Void => self.context.void_type(),
            AirType::Int(IntTy::I8) => self.context.i8_type(),
            AirType::Int(IntTy::I16) => self.context.i16_type(),
            AirType::Int(IntTy::I32) => self.context.i32_type(),
            AirType::Int(IntTy::I64) => self.context.i64_type(),
            AirType::Int(IntTy::I128) => self.context.i128_type(),
            AirType::Float(FloatTy::F32) => self.context.f32_type(),
            AirType::Float(FloatTy::F64) => self.context.f64_type(),
            AirType::Bool => self.context.i1_type(),
            AirType::Pointer(inner) => self.convert_type(inner).pointer_type(),
            AirType::Array(inner, size) => self.convert_type(inner).array_type(*size),
            AirType::Struct(id, args) => self.convert_struct_type(*id, args),
            AirType::Function(sig) => self.convert_function_type(sig),
            _ => self.context.i8_type(), // Fallback
        }
    }
}
```

---

This concludes Part 2. I'll continue with Part 3 covering Binary Generation, Linker, Runtime, and Interpreter...
