use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use void::Void;
use paste::paste;

// This is a Visitor that forwards string types to T's `FromStr` impl and
// forwards map types to T's `Deserialize` impl. The `PhantomData` is to
// keep the compiler from complaining about T being an unused generic type
// parameter. We need T in order to know the Value type for the Visitor
// impl.
struct StringOrStruct<T>(PhantomData<fn() -> T>);

impl<T> Clone for StringOrStruct<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for StringOrStruct<T> {}

impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err=Void>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
    {
        Ok(FromStr::from_str(value).unwrap())
    }

    fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
    {
        // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
        // into a `Deserializer`, allowing it to be used as the input to T's
        // `Deserialize` implementation. T then deserializes itself using
        // the entries from the map visitor.
        Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
    }
}

impl<'de, T> DeserializeSeed<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err=Void>,
{
    type Value = T; // 根据需要设置正确的类型

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        // 创建一个新的 StringOrStruct Visitor 来处理反序列化
        deserializer.deserialize_any(self)
    }
}

pub fn de_string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err=Void>,
        D: Deserializer<'de> {
    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

// None of this crate's error handling needs the `From::from` error conversion
// performed implicitly by the `?` operator or the standard library's `try!`
// macro. This simplified macro gives a 5.5% improvement in compile time
// compared to standard `try!`, and 9% improvement compared to `?`.
macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err),
        }
    };
}

macro_rules! map_impl {
    (
        $(#[$attr:meta])*
        $ty:ident <K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)*>,
        $name:ident,
        $access:ident,
        $with_capacity:expr,
    ) => {
        $(#[$attr])*
        paste! {
            struct [<$name Visitor>]<K, V $(, $typaram)*> (PhantomData<fn() -> $ty<K, V $(, $typaram)*>>);

            impl<'de, K, V $(, $typaram)*> Visitor<'de> for [<$name Visitor>]<K, V $(, $typaram)*>
            where
                K: Deserialize<'de> $(+ $kbound1 $(+ $kbound2)*)*,
                V: Deserialize<'de> + FromStr<Err=Void>,
                $($typaram: $bound1 $(+ $bound2)*),*
            {
                type Value = $ty<K, V $(, $typaram)*>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map")
                }

                #[inline]
                fn visit_map<A>(self, mut $access: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let inner_visitor = StringOrStruct(PhantomData);
                    let mut values = $with_capacity;

                    while let Some(key) = tri!($access.next_key()) {
                        values.insert(key, tri!($access.next_value_seed(inner_visitor)));
                    }

                    Ok(values)
                }
            }
        }
    }
}

map_impl! {
    #[cfg_attr(doc_cfg, doc(cfg(any(feature = "std", feature = "alloc"))))]
    BTreeMap<K: Ord, V>,
    BTreeMap,
    map,
    BTreeMap::new(),
}

map_impl! {
    #[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
    HashMap<K: Eq + Hash, V, S: BuildHasher + Default>,
    HashMap,
    map,
    HashMap::with_capacity_and_hasher(32, S::default()),
}

pub fn de_string_or_struct_hashmap<'de, T, D>(deserializer: D) -> Result<HashMap<String, T>, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err=Void>,
        D: Deserializer<'de> {
    deserializer.deserialize_map(HashMapVisitor::<String, T, std::collections::hash_map::RandomState>(PhantomData))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;
    use serde::Deserialize;
    use void::Void;

    #[derive(Debug, Deserialize)]
    struct Foo1 {
        foo: String,
        bar: String,
    }

    impl FromStr for Foo1 {
        type Err = Void;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Foo1 {
                foo: s.to_string(),
                bar: "".to_string(),
            })
        }
    }

    #[test]
    fn de_string_or_struct() {
        #[derive(Debug, Deserialize)]
        struct Root {
            #[serde(deserialize_with = "super::de_string_or_struct")]
            root: Foo1,
        }

        let s = r#"{"root":"hello"}"#;
        let root: Root = serde_json::from_str(s).unwrap();
        assert_eq!(root.root.foo, "hello");
        assert_eq!(root.root.bar, "");

        let j = r#"{"root":{"foo": "hello", "bar": "world"}}"#;
        let root: Root = serde_json::from_str(j).unwrap();
        assert_eq!(root.root.foo, "hello");
        assert_eq!(root.root.bar, "world");
    }

    #[test]
    fn de_string_or_struct_hashmap() {
        #[derive(Debug, Deserialize)]
        struct Root {
            #[serde(deserialize_with = "super::de_string_or_struct_hashmap")]
            root: HashMap<String, Foo1>,
        }

        let s = r#"{"root":{"a": "hello"}}"#;
        let root: Root = serde_json::from_str(s).unwrap();
        assert_eq!(root.root.len(), 1);
        let a = root.root.get("a").unwrap();
        assert_eq!(a.foo, "hello");
        assert_eq!(a.bar, "");

        let s = r#"{"root":{"a":{"foo": "hello", "bar": "world"}}}"#;
        let root: Root = serde_json::from_str(s).unwrap();
        assert_eq!(root.root.len(), 1);
        let a = root.root.get("a").unwrap();
        assert_eq!(a.foo, "hello");
        assert_eq!(a.bar, "world");

        let s = r#"{"root":{"a":{"foo": "hello", "bar": "world"},"b":"bar"}}"#;
        let root: Root = serde_json::from_str(s).unwrap();
        assert_eq!(root.root.len(), 2);
        let a = root.root.get("a").unwrap();
        assert_eq!(a.foo, "hello");
        assert_eq!(a.bar, "world");
        let b = root.root.get("b").unwrap();
        assert_eq!(b.foo, "bar");
        assert_eq!(b.bar, "");
    }
}