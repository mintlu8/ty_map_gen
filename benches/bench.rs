use std::{any::{Any, TypeId}, collections::HashMap};

use criterion::{criterion_group, criterion_main, Criterion};
use rustc_hash::FxHashMap;
use ty_map_gen::type_map;


type_map!(
    pub MyMap where T [] => T as FxHashMap
);

pub fn bench_maps2(c: &mut Criterion) {
    c.bench_function("init_any_map", |b|{
        b.iter(|| {
            let mut map = HashMap::<TypeId, Box<dyn Any>>::new();
            map.insert(TypeId::of::<String>(), Box::new("Hello".to_owned()));
            map.insert(TypeId::of::<i32>(), Box::new(13));
            map.insert(TypeId::of::<Vec<u8>>(), Box::new(b"DEADBEEF".to_owned()));
            map.insert(TypeId::of::<()>(), Box::new(()));
        });
    });

    c.bench_function("init_gen_map", |b|{
        b.iter(|| {
            let mut map = MyMap::new();
            map.insert("Hello".to_owned());
            map.insert(13);
            map.insert(b"DEADBEEF".to_owned());
            map.insert(());
        });
    });
}

criterion_group!(bencher, 
    bench_maps2,
);
criterion_main!(bencher);