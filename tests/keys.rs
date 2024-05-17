use std::fmt::Debug;

use rustc_hash::FxHashMap;
use ty_map_gen::type_map;

type_map!(
    #[derive(Debug)]
    pub BaseMap where (T, String) => T [std::fmt::Debug] as FxHashMap
);

#[test]
pub fn test() {
    let mut map = BaseMap::new();
    map.insert("Dog".to_owned(), "Bud");
    map.insert("Cat".to_owned(), "Kate");
    map.insert("Dog".to_owned(), 6);
    map.insert("Cat".to_owned(), 5);
    assert_eq!(map.get::<&str, _>("Dog"), Some(&"Bud"));
    assert_eq!(map.get::<&str, _>("Cat"), Some(&"Kate"));
    assert_eq!(map.get::<&str, _>("Fish"), None);
    assert_eq!(map.get::<i32, _>("Dog"), Some(&6));
    assert_eq!(map.get::<i32, _>("Cat"), Some(&5));
    assert_eq!(map.get::<i32, _>("Cat"), Some(&5));
    assert_eq!(map.get::<i32, _>("Fish"), None);
}
