use std::fmt::Debug;

use rustc_hash::FxHashMap;
use ty_map_gen::type_map;

type_map!(
    #[derive(Debug)]
    pub BaseMap where T => T [std::fmt::Debug] as FxHashMap
);

#[test]
pub fn test() {
    let mut map = BaseMap::new();
    map.insert("Hello".to_owned());
    map.insert(13);
    map.insert(b"DEADBEEF".to_owned());
    map.insert(());
    assert_eq!(map.get::<String>(), Some(&"Hello".to_owned()));
    assert_eq!(map.get::<i32>(), Some(&13));
    assert_eq!(map.get::<[u8; 8]>(), Some(&b"DEADBEEF".to_owned()));
    assert_eq!(map.get::<()>(), Some(&()));
    assert_eq!(map.get::<f32>(), None);
    assert_eq!(map.get::<Box<i32>>(), None);

    let _ = map.get_mut::<String>().map(|v| *v = "World".to_owned());
    assert_eq!(map.get::<String>(), Some(&"World".to_owned()));

    let _ = map.get_mut::<i32>().map(|v| *v = 72);
    assert_eq!(map.get::<i32>(), Some(&72));

    let v = map.remove::<[u8; 8]>();
    assert_eq!(v, Some(b"DEADBEEF".to_owned()));

    let map = map.remove::<Box<()>>();
    assert_eq!(map, None);
}
