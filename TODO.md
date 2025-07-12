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

## 🚀 PHASE 1: Foundation (COMPLETED)

### Critical Fixes (Blocking Issues) - COMPLETED
- [x] **Fix `todo!()` implementations** (5 locations found)
  - [x] `crates/rustyfast/src/codec.rs` (u64/i64 serialization/deserialization)
  - [x] `crates/rustyfast/src/codec.rs` (PresenceMap serialization)
  - [x] `crates/rustyfix/src/session/connection.rs` (on_high_seqnum)
  - [x] `crates/rustyfix/src/tagvalue/tokio_decoder.rs` (marked as `unimplemented!` due to design issues)

### Performance Mandates Compliance - COMPLETED
- [x] **Replace standard collections with performance variants**
  - [x] `HashMap` -> `FxHashMap`
  - [x] `Vec` -> `SmallVec`
  - [x] `String` -> `SmartString`
- [x] **Implement quanta for all timing operations**
- [x] **Add simd-json for JSON operations** (Blocked by `deserialize_in_place`)

### Naming Convention Fixes - COMPLETED
- [x] **Expand abbreviations using fastmod**
  - [x] `qty` -> `quantity` (as enum variant `Quantity`)
  - [x] `ts`, `req`, `resp` abbreviations checked and not present in Rust code.
  - [x] `msg` -> `message`
- [x] **Rename files for clarity**
  - [x] `rustyfast/src/dtf.rs` → `data_type_field.rs`

---

## 🔧 PHASE 2: Core Features (IN PROGRESS)

### Complete Partial Implementations
- [ ] **JSON Encoder (rustyfix)**
  - [x] Implement missing JSON encoder structure
  - [x] Add comprehensive JSON round-trip tests
- [ ] **FAST Protocol Completion (rustyfast)**  
  - [x] Complete template code generation (getters/setters)
  - [x] Add template validation and error recovery
  - [ ] Implement field operator optimizations (stateful encoder) - **BLOCKED:
    Needs message representation for encoding.**
- [ ] **Validation & Error Recovery**
  - [ ] **Add semantic validation trait**
  - [ ] Implement required field checking using Dictionary
  - [ ] Add malformed message recovery in decoders
  - [ ] Create validation benchmarks

### Enhanced Error Handling
- [ ] **Replace panics with proper error handling**
- [ ] Use `thiserror` for better error ergonomics
- [ ] Add error context throughout call chains

---

## 🆕 PHASE 3: Missing Encodings (LOWER PRIORITY - Week 5-8)

### New Encoding Crates

#### rustyfixml (FIXML Support)
- [ ] **Create new crate for XML encoding**
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
- [ ] Add comprehensive API documentation
- [ ] Create performance comparison benchmarks
- [ ] Add architectural decision records (ADRs)

### Rust 2024 Migration
- [ ] **Adopt new language features**
  - [ ] Let chains in conditional logic
  - [ ] RPIT lifetime capture rules
  - [ ] Unsafe attributes where needed
  - [ ] Static mutable reference alternatives 