# Dahlia Element Fix Plan

## Task Info
- **Task ID**: 91
- **Character**: Dahlia (Hydro / Polearm)
- **Issue**: Currently in `pyro/` with Pyro element - WRONG

## Problem Confirmed
- **Fandom Wiki**: "Dahlia is a playable Hydro character"
- **Current Location**: `pyro/dahlia.rs` ❌
- **Correct Location**: `hydro/dahlia.rs` ✅

## Actions Required

### 1. Copy Files
- `pyro/dahlia.rs` → `hydro/dahlia.rs`
- `pyro/dahlia_default.rs` → `hydro/dahlia_default.rs`

### 2. Update File Contents
- Change `Element::Pyro` → `Element::Hydro`
- Update module comments

### 3. Update mod.rs Exports
**hydro/mod.rs**:
```rust
pub mod dahlia;
pub use dahlia::Dahlia;
```

**hydro/mod.rs (target function)**:
```rust
pub mod dahlia_default;
pub use dahlia_default::DahliaDefaultTargetFunction;
```

### 4. Remove Old Files
- Delete `pyro/dahlia.rs`
- Delete `pyro/dahlia_default.rs`
- Remove exports from `pyro/mod.rs`

### 5. Update CharacterConfig
**character_config.rs**:
- Dahlia should be in Hydro section (already correct - config is shared)

## Branch
`feature/dahlia-hydro-20260207`
