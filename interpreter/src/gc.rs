//! Planned garbage collector module.
//!
//! The first interpreter milestone keeps values owned and reference-count free.
//! This module is intentionally small so we can evolve toward tracing GC later.

#[derive(Debug, Default)]
pub struct GcStats {
    pub allocations: usize,
}
