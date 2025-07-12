# RustyFix Overhaul & Enhancement TODO

> **Comprehensive roadmap for transforming RustyFix into a complete, high-performance FIX implementation**

## 🎯 Executive Summary

This TODO consolidates analysis of all 8 crates to address:
- **Technical Debt**: 5 critical `todo!()` implementations, naming inconsistencies, performance gaps
- **Missing Features**: JSON encoder, complete FAST support, 4 new encodings, validation/error recovery
- **Performance**: Mandate compliance (quanta, SmallVec, FxHashMap), zero-copy optimizations
- **Architecture**: Complete OSI layer separation, full FIX stack implementation

**Success Criteria**: All TODO items completed, benchmarks show >50% performance improvement, full encoding support per README.md

---

## 🚀 PHASE 1: Foundation (HIGH PRIORITY - Week 1-2)

### Critical Fixes (Blocking Issues)
- [ ] **Fix `todo!()` implementations** (5 locations found)
  ```bash
  # Find all todo!() calls
  ast-grep --pattern 'todo!()' --lang rust
  
  # Priority order:
  # 1. crates/rustyfast/src/codec.rs (lines 86,90,96,100,170)
  # 2. crates/rustyfix/src/session/connection.rs (line 493)  
  # 3. crates/rustyfix/src/tagvalue/tokio_decoder.rs (line 59)
  ```

#### rustyfast/codec.rs
- [ ] Implement u64/i64 serialization/deserialization
  ```rust
  // Use big-endian encoding with proper error handling
  // Test with round-trip property tests
  ```
- [ ] Complete PresenceMap codec with bitvec optimization

#### rustyfix/session/connection.rs  
- [ ] Implement message verification logic
- [ ] Add sequence number validation
- [ ] Complete session state management

#### rustyfix/tagvalue/tokio_decoder.rs
- [ ] Implement streaming decoder integration
- [ ] Add error recovery for malformed frames

### Performance Mandates Compliance
- [ ] **Replace standard collections with performance variants**
  ```bash
  # Use fastmod for bulk replacements
  fastmod 'Vec<([^>]+)>' 'SmallVec<[${1}; 8]>' --extensions rs --dir crates/
  fastmod 'HashMap' 'FxHashMap' --extensions rs --dir crates/
  fastmod 'String' 'SmartString' --extensions rs --dir crates/
  ```

- [ ] **Implement quanta for all timing operations**
  ```bash
  # Find timing usage
  rg 'std::time|chrono::Utc::now' --type rust
  # Replace with quanta::Instant for nanosecond precision
  ```

- [ ] **Add simd-json for JSON operations**
  ```bash
  fastmod 'serde_json' 'simd_json' --extensions rs --dir crates/rustyfix/src/json/
  ```

### Naming Convention Fixes
- [ ] **Expand abbreviations using fastmod**
  ```bash
  # Core abbreviations to fix
  fastmod '\bqty\b' 'quantity' --extensions rs --dir crates/
  fastmod '\bts\b' 'timestamp' --extensions rs --dir crates/  
  fastmod '\bmsg\b' 'message' --extensions rs --dir crates/
  fastmod '\breq\b' 'request' --extensions rs --dir crates/
  fastmod '\bresp\b' 'response' --extensions rs --dir crates/
  ```

- [ ] **Rename files for clarity**
  ```bash
  # rustyfast/src/dtf.rs → data_type_field.rs
  # Update imports with ast-grep
  ```

---

## 🔧 PHASE 2: Core Features (MEDIUM PRIORITY - Week 3-4)

### Complete Partial Implementations

#### JSON Encoder (rustyfix)
- [ ] **Implement missing JSON encoder**
  ```bash
  # Create crates/rustyfix/src/json/encoder.rs
  # Mirror decoder structure, use simd-json
  # Update mod.rs to export encoder
  ```
- [ ] Add comprehensive JSON round-trip tests
- [ ] Benchmark against other JSON libraries

#### FAST Protocol Completion (rustyfast)  
- [ ] **Complete template code generation**
  ```bash
  # Fix TODO in crates/rustyfast/src/codegen.rs line 45
  # Generate getter/setter methods for template structs
  ```
- [ ] Add template validation and error recovery
- [ ] Implement field operator optimizations

#### Validation & Error Recovery
- [ ] **Add semantic validation trait**
  ```rust
  // New file: crates/rustyfix/src/validation.rs
  pub trait Validator {
      fn validate(&self, msg: &Message, dict: &Dictionary) -> Result<(), ValidationError>;
  }
  ```
- [ ] Implement required field checking using Dictionary
- [ ] Add malformed message recovery in decoders
- [ ] Create validation benchmarks

### Enhanced Error Handling
- [ ] **Replace panics with proper error handling**
  ```bash
  # Find unwrap() usage
  rg '\.unwrap\(\)' --type rust
  # Replace with expect() or proper error propagation
  ```
- [ ] Use `thiserror` for better error ergonomics
- [ ] Add error context throughout call chains

---

## 🆕 PHASE 3: Missing Encodings (LOWER PRIORITY - Week 5-8)

### New Encoding Crates

#### rustyfixml (FIXML Support)
- [ ] **Create new crate for XML encoding**
  ```bash
  cargo new crates/rustyfixml
  # Add to workspace Cargo.toml
  ```
- [ ] Use `quick-xml` for performance
- [ ] Integrate with Dictionary for schema validation
- [ ] Add FIXML-specific field naming (abbreviations)

#### rustysbe (Simple Binary Encoding)
- [ ] **Create SBE crate for low-latency binary**
- [ ] Use `bitvec` for bit-level operations  
- [ ] Implement zero-copy message access
- [ ] Add SBE schema generation from Dictionary

#### rustyfixgpb (Protocol Buffers)
- [ ] **Create GPB crate using `prost`**
- [ ] Generate .proto files from Dictionary
- [ ] Implement FIX-to-GPB semantic mapping
- [ ] Add compression options

#### rustyfixasn (ASN.1 Support)
- [ ] **Create ASN.1 crate (PER/BER/OER variants)**
- [ ] Use `asn1-rs` for encoding/decoding
- [ ] Focus on PER for efficiency
- [ ] Add legacy system compatibility

### Integration
- [ ] Add all new encodings to rustyfix as optional features
- [ ] Create unified codec trait across encodings
- [ ] Add encoding detection utilities
- [ ] Benchmark all encodings for size/speed

---

## 🎨 PHASE 4: Polish & Optimization (ONGOING - Week 9+)

### Advanced Performance
- [ ] **SIMD optimizations where applicable**
  ```bash
  # Use simd_aligned for buffer operations
  # Add wide crate for SIMD primitives
  # Profile with cargo flamegraph
  ```
- [ ] Memory layout optimizations (`#[repr(align(64))]`)
- [ ] Lock-free algorithms in session layer
- [ ] Custom allocators for hot paths

### Complete FIXP Session Layer
- [ ] **Implement full rustyfixp**
  - [ ] Session negotiation
  - [ ] Flow control (Recoverable, Idempotent, Unsequenced)
  - [ ] Sequence number management
  - [ ] Integration with transport layer

### Documentation & Examples
- [ ] **Complete missing examples**
  ```bash
  # Fix TODO in examples/tls_fixua_acceptor/src/main.rs
  # Complete examples/20_tokio_tradeclient/src/main.rs
  ```
- [ ] Add comprehensive API documentation
- [ ] Create performance comparison benchmarks
- [ ] Add architectural decision records (ADRs)

### Rust 2024 Migration
- [ ] **Adopt new language features**
  - [ ] Let chains in conditional logic
  - [ ] RPIT lifetime capture rules
  - [ ] Unsafe attributes where needed
  - [ ] Static mutable reference alternatives

---

## 🛠️ Development Workflow

### Tools Usage
```bash
# Code discovery and analysis
ast-grep --pattern '$PATTERN' --lang rust
rg '$PATTERN' --type rust

# Bulk refactoring  
fastmod '$OLD' '$NEW' --extensions rs --dir crates/

# Testing and validation
cargo nextest run --workspace
just test
just lint
just check-features

# Performance monitoring
cargo bench
cargo flamegraph --bench $BENCHMARK
```

### Quality Gates
- [ ] All tests pass (`cargo test --all-features`)
- [ ] Clippy clean (`cargo clippy --all-targets --all-features`)
- [ ] Documentation complete (`cargo doc --all-features`)
- [ ] Benchmarks show improvement
- [ ] Examples run successfully

### Success Metrics
- **Code Quality**: Zero `todo!()`, zero `unwrap()` in production paths
- **Performance**: >50% improvement in decode/encode benchmarks  
- **Completeness**: All README.md claimed features implemented
- **Testing**: >80% code coverage, property tests for all codecs
- **Documentation**: All public APIs documented with examples

---

## 📋 Quick Start Checklist

**Week 1 Focus:**
- [ ] Fix all 5 `todo!()` implementations
- [ ] Replace HashMap with FxHashMap
- [ ] Implement quanta timing
- [ ] Add comprehensive tests

**Ready to Start:**
```bash
git checkout -b overhaul-phase1
ast-grep --pattern 'todo!()' --lang rust > todo_locations.txt
# Begin with rustyfast/codec.rs u64/i64 implementation
```

**Completion Criteria:**
- [ ] All phases completed
- [ ] Performance benchmarks meet targets  
- [ ] Full FIX stack operational
- [ ] Production-ready codebase

---

*This TODO represents a comprehensive transformation of RustyFix from a partial implementation to a complete, high-performance FIX protocol stack. Execute incrementally with continuous testing and validation.* 