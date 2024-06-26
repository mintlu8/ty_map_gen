# ty_map_gen

[![Crates.io](https://img.shields.io/crates/v/ty_map_gen.svg)](https://crates.io/crates/ty_map_gen)
[![Docs](https://docs.rs/ty_map_gen/badge.svg)](https://docs.rs/ty_map_gen/latest/ty_map_gen/)

A type projecting map generator.

## Syntax

```rust
type_map!(
    /// Asset Map
    #[derive(Clone, PartialEq, Eq)]
    pub AssetMap where T [Asset] => Handle<T> [Clone + Eq] as HashMap
);
```

This creates a type that projects `T` (a generic) to `Handle<T>` (roughly):

```rust
pub struct AssetMap(HashMap<TypeId, Box<dyn Any>>)
```

with access methods (roughly):

```rust
fn get<T: Asset + 'static>(&self) -> Option<&Handle<T>> where Handle<T>: Clone + Eq {
    self.0.get(TypeId::of::<T>()).and_then(|v| v.downcast_ref())
}
```

Since all values stored in the map are `Clone` and `Eq`, they can be derived.

The `as HashMap` field accepts all structs that has the same api
as `HashMap`, this includes `BTreeMap` and third party types
like `FxHashMap` or `VecMap`. To specify a custom hasher, you must
define a new `type` with signature `Map<Key, Value>`.

## Bounds

Bounds (in braces `[]`) are optional in the macro. The bounds on the right hand side must be object safe,
excluding `Clone`, `PartialEq`, `Eq`, `Ord`, `PartialOrd`, `Hash` and `Serialize`,
which are special handled. Additionally only one trait from `std::cmp` is allowed to be specified.
If you need `Send` and `Sync` this is where to add them.

Currently the right hand side uses a unique scope, therefore you must supply fully qualified trait paths.

## Methods and Implementations

By default these methods are generated by the macro:

`Default`, `new`, `is_empty`, `len`, `get`, `get_mut`, `insert`, `remove`, `clear`, `extend`.

## Double Keys

```rust
type_map!(
    /// Asset Map
    #[derive(Clone, PartialEq, Eq)]
    pub AssetMap where (T, String) [Asset] => Handle<T> [Clone + Eq] as HashMap
);
```

This adds another key to the map, with access methods (roughly):

```rust
fn get<T: Asset + 'static>(&self, key: &Q) -> Option<&Handle<T>> where Handle<T>: Clone + Eq {
    self.0.get(&(TypeId::of::<T>(), key)).and_then(|v| v.downcast_ref())
}
```

## Performance

This crate has faster lookup (`get` and `get_mut`) than a naive implementation with `Box<dyn Any>`
since downcasting is unchecked.

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
