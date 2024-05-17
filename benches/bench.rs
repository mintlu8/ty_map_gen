use std::any::{Any, TypeId};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustc_hash::FxHashMap;
use ty_map_gen::type_map;

type_map!(
    pub MyMap where T [] => T as FxHashMap
);

pub fn bench_maps2(c: &mut Criterion) {
    c.bench_function("init_any_map", |b| {
        b.iter(|| {
            let mut map = FxHashMap::<TypeId, Box<dyn Any>>::default();
            map.insert(TypeId::of::<String>(), Box::new("Hello".to_owned()));
            map.insert(TypeId::of::<i32>(), Box::new(13));
            map.insert(TypeId::of::<Vec<u8>>(), Box::new(b"DEADBEEF".to_owned()));
            map.insert(TypeId::of::<()>(), Box::new(()));
        });
    });

    c.bench_function("init_gen_map", |b| {
        b.iter(|| {
            let mut map = MyMap::new();
            map.insert("Hello".to_owned());
            map.insert(13);
            map.insert(b"DEADBEEF".to_owned());
            map.insert(());
        });
    });

    let mut map = FxHashMap::<TypeId, Box<dyn Any>>::default();
    map.insert(TypeId::of::<String>(), Box::new("Hello".to_owned()));
    map.insert(TypeId::of::<i32>(), Box::new(13));
    map.insert(TypeId::of::<Vec<u8>>(), Box::new(b"DEADBEEF".to_owned()));
    map.insert(TypeId::of::<()>(), Box::new(()));

    c.bench_function("get_any_map", |b| {
        b.iter(|| {
            black_box(
                map.get(&TypeId::of::<String>())
                    .and_then(|x| x.downcast_ref::<String>())
                    .map(|x| x.len()),
            );
        });
    });

    let mut map = MyMap::new();
    map.insert("Hello".to_owned());
    map.insert(13);
    map.insert(b"DEADBEEF".to_owned());
    map.insert(());

    c.bench_function("get_gen_map", |b| {
        b.iter(|| {
            black_box(map.get::<String>().map(|x| x.len()));
        });
    });

    let mut map = FxHashMap::<TypeId, Box<dyn Any>>::default();

    c.bench_function("remove_any_map", |b| {
        b.iter(|| {
            map.insert(TypeId::of::<i32>(), Box::new(42));
            map.remove(&TypeId::of::<i32>());
        });
    });

    let mut map = MyMap::new();
    c.bench_function("remove__gen_map", |b| {
        b.iter(|| {
            map.insert(42);
            map.remove::<i32>();
        });
    });
}

criterion_group!(bencher, bench_maps2,);
criterion_main!(bencher);
