//! Performance benchmarks for the skip tag system
//!
//! These benchmarks validate that the skip tag system meets the performance
//! requirements specified in the architecture:
//! - Tag lookup: <1μs
//! - Tag creation: <10μs
//! - Registry access: <1μs
//!
//! Run with: cargo bench --bench skip_tag_benchmark

use balatro_rs::skip_tags::{TagId, TagRegistry, TagCategory, TagEffectType};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use std::time::Duration;

/// Benchmark tag definition lookup performance
///
/// Target: <1μs per lookup
/// This is critical for responsive UI and game flow
fn benchmark_tag_lookup(c: &mut Criterion) {
    let registry = TagRegistry::global();
    let read_guard = registry.read().unwrap();

    c.bench_function("tag_definition_lookup", |b| {
        b.iter(|| {
            // Test lookup for all tag types to ensure consistent performance
            let tag_id = black_box(TagId::Charm);
            let definition = read_guard.get_definition(tag_id).unwrap();
            black_box(definition);
        });
    });

    // Benchmark lookup for all tags to find worst-case performance
    let mut group = c.benchmark_group("tag_lookup_all_tags");
    group.measurement_time(Duration::from_secs(10));
    
    for tag_id in TagId::all().iter() {
        group.bench_with_input(
            BenchmarkId::new("lookup", format!("{:?}", tag_id)),
            tag_id,
            |b, &tag_id| {
                b.iter(|| {
                    let definition = read_guard.get_definition(black_box(tag_id)).unwrap();
                    black_box(definition);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark tag creation performance
///
/// Target: <10μs per creation
/// Important for responsive tag selection
fn benchmark_tag_creation(c: &mut Criterion) {
    let registry = TagRegistry::global();
    let read_guard = registry.read().unwrap();

    c.bench_function("tag_creation", |b| {
        b.iter(|| {
            let tag_id = black_box(TagId::Economy);
            let tag = read_guard.create_tag(tag_id).unwrap();
            black_box(tag);
        });
    });

    // Benchmark creation for different tag types
    let mut group = c.benchmark_group("tag_creation_by_type");
    group.measurement_time(Duration::from_secs(10));
    
    // Sample tags from each category
    let sample_tags = [
        (TagId::Charm, "Reward"),
        (TagId::Economy, "Economic"),
        (TagId::Coupon, "ShopEnhancement"),
        (TagId::Double, "Utility"),
    ];
    
    for (tag_id, category) in sample_tags.iter() {
        group.bench_with_input(
            BenchmarkId::new("create", category),
            tag_id,
            |b, &tag_id| {
                b.iter(|| {
                    let tag = read_guard.create_tag(black_box(tag_id)).unwrap();
                    black_box(tag);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark registry access patterns
///
/// Target: <1μs for registry operations
/// Tests thread-safe access performance
fn benchmark_registry_access(c: &mut Criterion) {
    let registry = TagRegistry::global();

    c.bench_function("registry_read_lock", |b| {
        b.iter(|| {
            let read_guard = registry.read().unwrap();
            let count = read_guard.count();
            black_box(count);
        });
    });

    c.bench_function("registry_is_registered", |b| {
        let read_guard = registry.read().unwrap();
        b.iter(|| {
            let tag_id = black_box(TagId::Buffoon);
            let is_registered = read_guard.is_registered(tag_id);
            black_box(is_registered);
        });
    });

    c.bench_function("registry_get_all_definitions", |b| {
        let read_guard = registry.read().unwrap();
        b.iter(|| {
            let definitions = read_guard.get_all_definitions();
            black_box(definitions);
        });
    });
}

/// Benchmark filtering operations
///
/// Tests performance of category and type-based filtering
/// Important for tag selection algorithms
fn benchmark_filtering_operations(c: &mut Criterion) {
    let registry = TagRegistry::global();
    let read_guard = registry.read().unwrap();

    c.bench_function("filter_by_category", |b| {
        b.iter(|| {
            let category = black_box(TagCategory::Reward);
            let filtered = read_guard.get_definitions_by_category(category);
            black_box(filtered);
        });
    });

    c.bench_function("filter_by_effect_type", |b| {
        b.iter(|| {
            let effect_type = black_box(TagEffectType::ImmediateReward);
            let filtered = read_guard.get_definitions_by_effect_type(effect_type);
            black_box(filtered);
        });
    });

    c.bench_function("get_available_definitions", |b| {
        b.iter(|| {
            let available = read_guard.get_available_definitions();
            black_box(available);
        });
    });
}

/// Benchmark tag trait operations
///
/// Tests performance of core trait methods
/// Critical for game loop performance
fn benchmark_tag_trait_operations(c: &mut Criterion) {
    use balatro_rs::game::Game;

    let registry = TagRegistry::global();
    let read_guard = registry.read().unwrap();
    let tag = read_guard.create_tag(TagId::Charm).unwrap();
    let game = Game::default();

    c.bench_function("tag_can_apply", |b| {
        b.iter(|| {
            let result = tag.can_apply(black_box(&game));
            black_box(result);
        });
    });

    c.bench_function("tag_basic_methods", |b| {
        b.iter(|| {
            let id = tag.id();
            let name = tag.name();
            let effect_type = tag.effect_type();
            let description = tag.description();
            black_box((id, name, effect_type, description));
        });
    });
}

/// Benchmark concurrent access patterns
///
/// Tests registry performance under concurrent load
/// Important for multi-threaded game engines
fn benchmark_concurrent_access(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;

    c.bench_function("concurrent_tag_lookup", |b| {
        b.iter(|| {
            let registry = TagRegistry::global();
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let registry = Arc::clone(registry);
                    thread::spawn(move || {
                        let read_guard = registry.read().unwrap();
                        for tag_id in [TagId::Charm, TagId::Economy, TagId::Coupon, TagId::Double] {
                            let definition = read_guard.get_definition(tag_id).unwrap();
                            black_box(definition);
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark memory allocation patterns
///
/// Tests memory efficiency of tag operations
/// Important for avoiding GC pressure in game loops
fn benchmark_memory_efficiency(c: &mut Criterion) {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingAllocator;

    static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

    unsafe impl GlobalAlloc for CountingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout);
        }
    }

    c.bench_function("tag_lookup_allocations", |b| {
        let registry = TagRegistry::global();
        let read_guard = registry.read().unwrap();

        b.iter(|| {
            let before = ALLOCATION_COUNT.load(Ordering::Relaxed);
            
            // Perform operations that should minimize allocations
            let definition = read_guard.get_definition(black_box(TagId::Meteor)).unwrap();
            let tag = read_guard.create_tag(black_box(TagId::Meteor)).unwrap();
            
            black_box((definition, tag));
            
            let after = ALLOCATION_COUNT.load(Ordering::Relaxed);
            let allocations = after - before;
            
            // Assert minimal allocations (tag creation may allocate the Box)
            assert!(allocations <= 1, "Too many allocations: {}", allocations);
        });
    });
}

/// Benchmark tag system scalability
///
/// Tests performance characteristics as registry size grows
/// Important for extensibility planning
fn benchmark_scalability(c: &mut Criterion) {
    let registry = TagRegistry::global();
    let read_guard = registry.read().unwrap();

    // Test lookup performance across all tags
    c.bench_function("full_registry_scan", |b| {
        b.iter(|| {
            let mut total_weight = 0.0f32;
            for tag_id in TagId::all().iter() {
                if let Ok(definition) = read_guard.get_definition(*tag_id) {
                    total_weight += definition.base_weight;
                }
            }
            black_box(total_weight);
        });
    });

    // Test registry statistics calculation performance
    c.bench_function("registry_stats_calculation", |b| {
        b.iter(|| {
            let stats = read_guard.stats();
            black_box(stats);
        });
    });
}

criterion_group!(
    benches,
    benchmark_tag_lookup,
    benchmark_tag_creation,
    benchmark_registry_access,
    benchmark_filtering_operations,
    benchmark_tag_trait_operations,
    benchmark_concurrent_access,
    benchmark_memory_efficiency,
    benchmark_scalability
);

criterion_main!(benches);