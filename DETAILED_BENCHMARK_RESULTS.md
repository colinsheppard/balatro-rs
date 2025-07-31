# Detailed Benchmark Results - Phase 1 Performance Validation

## Benchmark Environment
- **Date**: 2025-07-31
- **Baseline**: phase1_baseline
- **Current**: phase1_complete
- **Platform**: Linux WSL2
- **Rust Version**: Release build with optimizations
- **Criterion Version**: 0.6.0

## Main Benchmark Suite Results

### Core Game Operations
```
run game gen actions: 179.13 µs → 184.64 µs (+5.32%)
hand evaluation performance: 1.3660 µs → 1.3927 µs (+5.29%)
hand evaluation batch: 895.04 µs → 903.99 µs (+5.56%)
```

### Joker Effect Processing
```
joker effect processing with cache: 263.87 µs (-6.25% IMPROVEMENT)
joker effect processing without cache: 212.31 µs → 215.01 µs (+12.56%)
```

### Cache Performance Analysis
```
cache_comparison/with_cache/10: 40.107 µs (+18.95%)
cache_comparison/without_cache/10: 23.876 µs (+9.62%)
cache_comparison/with_cache/50: 152.86 µs (+13.997%)
cache_comparison/without_cache/50: 111.61 µs (+13.73%)
cache_comparison/with_cache/100: 274.31 µs (+9.12%)
cache_comparison/without_cache/100: 227.97 µs (+9.21%)
cache_comparison/with_cache/500: 1.2185 ms (+11.62%)
cache_comparison/without_cache/500: 1.1381 ms (+14.19%)
```

## State Management Benchmark Results

### State Access Operations (Nanosecond Scale)
```
state_access/get_state: 20.033 ns → 20.435 ns (+8.87%)
state_access/get_accumulated_value: 20.210 ns → 20.654 ns (+2.71%)
state_access/has_triggers: 21.435 ns (-4.68% IMPROVEMENT)
```

### State Update Operations
```
state_updates/add_accumulated_value: 16.638 ns → 16.883 ns (+6.94%)
state_updates/use_trigger: 37.018 ns → 37.319 ns (+10.11%)
state_updates/set_custom_data: 37.747 ns → 38.411 ns (+9.04%)
state_updates/update_state: 16.287 ns → 16.658 ns (+5.59%)
```

### Concurrent and Bulk Operations
```
concurrent_access/parallel_reads: 102.81 µs → 105.12 µs (+17.80%)
bulk_operations/bulk_state_creation: 2.1583 µs → 2.1893 µs (+7.47%)
bulk_operations/bulk_value_updates: 1.6701 µs → 1.6857 µs (+7.29%)
```

### Memory Usage Benchmarks
```
memory_usage/state_manager_with_jokers/10: 1.4348 µs → 1.4535 µs (+10.86%)
memory_usage/state_manager_with_jokers/100: 10.827 µs → 10.886 µs (+7.67%)
memory_usage/state_manager_with_jokers/1000: 101.74 µs → 103.29 µs (+13.59%)
```

## Action Space Benchmark Results

### Basic Action Space Operations
```
actionspace_creation: 166.21 ns → 169.93 ns (+4.06%)
actionspace_to_vec_original: 143.29 ns → 146.02 ns (+6.76%)
actionspace_to_vec_cached: 12.893 ns → 13.131 ns (+5.01%)
actionspace_iter: 49.309 ns → 50.251 ns (+2.18%)
actionspace_iter_sum: 17.992 ns → 18.322 ns (+13.88%)
actionspace_is_empty: 38.221 ns → 39.168 ns (+7.49%)
```

### Critical Performance Issues
```
actionspace_repeated_to_vec: 17.924 µs → 19.233 µs (+70.97% SEVERE)
actionspace_repeated_to_vec_cached: 1.3332 µs → 1.5548 µs (+16.41%)
```

### RL Training Workflows
```
actionspace_rl_workflow_original: 3.5655 µs → 3.6523 µs (+6.50%)
actionspace_rl_workflow_optimized: 2.3710 µs → 2.4068 µs (+7.69%)
```

### Memory and Trait Operations
```
actionspace_memory_allocation: 423.89 ns → 429.91 ns (-0.45% NO CHANGE)
actionspace_from_trait: 143.83 ns → 144.77 ns (+3.22%)
actionspace_real_world_access: 2.3760 µs → 2.4056 µs (+2.59%)
```

## Performance Impact Analysis

### Regression Severity Classification

**SEVERE (>15% degradation):**
- actionspace_repeated_to_vec: +70.97%
- cache_comparison/with_cache/10: +18.95%
- concurrent_access/parallel_reads: +17.80%
- actionspace_repeated_to_vec_cached: +16.41%

**MODERATE (5-15% degradation):**
- joker effect processing without cache: +12.56%
- cache_comparison/without_cache/500: +14.19%
- memory_usage/state_manager_with_jokers/1000: +13.59%
- actionspace_iter_sum: +13.88%
- cache_comparison/with_cache/50: +13.997%
- cache_comparison/without_cache/50: +13.73%
- [Additional 15 benchmarks in this range]

**MINOR (0.1-5% degradation):**
- actionspace_creation: +4.06%
- actionspace_real_world_access: +2.59%
- state_access/get_accumulated_value: +2.71%
- actionspace_iter: +2.18%
- actionspace_from_trait: +3.22%

**IMPROVEMENTS:**
- joker effect processing with cache: -6.25%
- state_access/has_triggers: -4.68%
- actionspace_memory_allocation: -0.45%

## Technical Observations

### Memory Access Patterns
The widespread nature of the regressions (5-20% across diverse operations) suggests fundamental changes to memory access patterns or data structures introduced by the refactoring.

### Cache Coherency Issues
All cache-related benchmarks show significant degradation, indicating potential cache line pollution or increased memory footprint.

### Critical Path Analysis
Operations most critical for RL training (actionspace operations) show the most severe regressions, which could significantly impact training performance.

### Debug Code Elimination
Despite being a release build, debug-related overhead may not be properly eliminated, contributing to the widespread performance impact.

## Recommendations for Investigation

1. **Profile memory allocations** in the refactored modules
2. **Analyze debug code paths** for incomplete elimination in release builds  
3. **Review struct layouts** for cache-line alignment issues
4. **Investigate pack manager overhead** in hot paths
5. **Check for increased indirection** in state management

---
*Detailed benchmark data collected on 2025-07-31*
*All timings represent mean values from 100 samples*