use std::collections::BTreeMap;

#[allow(unused_imports)]
use serde_json::json;
use serde_json::map::Map;
use serde_json::Value;

use super::criteria::Criteria;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Null,
    Bool,
    U64,
    I64,
    Float,
    String,
    Array(Types),
    Object(String, ObjectScheme),
}

impl Type {
    fn key(&self) -> TypeKey {
        match self {
            Type::Null => TypeKey::Null,
            Type::Bool => TypeKey::Bool,
            Type::U64 => TypeKey::U64,
            Type::I64 => TypeKey::I64,
            Type::Float => TypeKey::Float,
            Type::String => TypeKey::String,
            Type::Array(_) => TypeKey::Array,
            Type::Object(ty, _) => TypeKey::Object(ty.clone()),
        }
    }

    fn unwrap_arr(self) -> Types {
        if let Type::Array(ts) = self {
            ts
        } else {
            panic!("Type is not Type::Array")
        }
    }

    fn unwrap_obj(self) -> (String, ObjectScheme) {
        if let Type::Object(ty, obj) = self {
            (ty, obj)
        } else {
            panic!("Type is not Type::Object")
        }
    }
}

impl Type {
    pub fn from_value(criteria: &Criteria, v: Value) -> Type {
        Type::from_value_path(criteria, "", v)
    }
    fn from_value_path(criteria: &Criteria, path: &str, v: Value) -> Type {
        match v {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Number(x) if x.is_u64() => Type::U64,
            Value::Number(x) if x.is_i64() => Type::I64,
            Value::Number(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Array(xs) => {
                let mut ts = Types::empty();
                let ch_path = format!("{}[]", path);
                for x in xs {
                    ts.add(Type::from_value_path(criteria, &ch_path, x));
                }
                Type::Array(ts)
            }
            Value::Object(mut map) => {
                let mut ty = String::new();
                if let Some(type_key) = criteria.is_split_enum(path) {
                    if let Some(Value::String(t)) = map.remove(type_key) {
                        ty = t;
                    }
                }
                Type::Object(ty, ObjectScheme::from_value_path(criteria, path, map))
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Types(BTreeMap<TypeKey, Type>);

impl Types {
    pub fn empty() -> Types {
        Types(BTreeMap::new())
    }

    pub fn null() -> Types {
        Type::Null.into()
    }

    pub fn add(&mut self, ty: Type) {
        self.add_i(ty);
        self.normalize();
    }

    pub fn is_nullable(&self) -> bool {
        self.0.contains_key(&TypeKey::Null)
    }

    pub fn into_vec(self) -> Vec<Type> {
        self.into_iter().collect()
    }

    pub fn merge(&mut self, other: Types) {
        for t in other.into_iter() {
            self.add_i(t);
        }
        self.normalize();
    }

    pub fn variants_count(&self) -> usize {
        if self.is_nullable() {
            self.0.len() - 1
        } else {
            self.0.len()
        }
    }

    pub fn with_null(mut self) -> Types {
        self.add(Type::Null);
        self
    }

    fn from_value_path(criteria: &Criteria, path: &str, v: Value) -> Types {
        let mut ts = Types::empty();
        ts.add(Type::from_value_path(criteria, path, v));
        ts
    }

    fn add_i(&mut self, ty: Type) {
        let key = ty.key();

        match key {
            TypeKey::Array => {
                if let Some(exists) = self.0.remove(&TypeKey::Array) {
                    let mut exists = exists.unwrap_arr();
                    exists.merge(ty.unwrap_arr());
                    self.0.insert(key, Type::Array(exists));
                } else {
                    self.0.insert(TypeKey::Array, ty);
                }
            }
            key @ TypeKey::Object(_) => {
                if let Some(exists) = self.0.remove(&key) {
                    let (com_ty, mut exists) = exists.unwrap_obj();
                    exists.merge(ty.unwrap_obj().1);
                    self.0.insert(key, Type::Object(com_ty, exists));
                } else {
                    self.0.insert(key, ty);
                }
            }
            key => {
                self.0.insert(key, ty);
            }
        }
    }

    fn normalize(&mut self) {
        if self.0.contains_key(&TypeKey::U64) && self.0.contains_key(&TypeKey::I64) {
            self.0.remove(&TypeKey::U64);
        }

        if self.0.contains_key(&TypeKey::U64) && self.0.contains_key(&TypeKey::Float) {
            self.0.remove(&TypeKey::U64);
        }

        if self.0.contains_key(&TypeKey::I64) && self.0.contains_key(&TypeKey::Float) {
            self.0.remove(&TypeKey::I64);
        }
    }
}

impl From<Type> for Types {
    fn from(t: Type) -> Types {
        let mut ts = Types::empty();
        ts.add_i(t);
        ts
    }
}

impl From<Vec<Type>> for Types {
    fn from(orig_ts: Vec<Type>) -> Types {
        let mut ts = Types::empty();
        for t in orig_ts {
            ts.add_i(t);
        }
        ts
    }
}

impl IntoIterator for Types {
    type Item = Type;
    type IntoIter = Box<dyn Iterator<Item = Type>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.0.into_iter().map(|x| x.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum TypeKey {
    Null,
    Bool,
    U64,
    I64,
    Float,
    String,
    Array,
    Object(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectScheme {
    fields: BTreeMap<String, Types>,
}

impl ObjectScheme {
    pub fn contains_key(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    fn from_value_path(criteria: &Criteria, path: &str, v: Map<String, Value>) -> ObjectScheme {
        let mut fields = BTreeMap::new();

        for (k, v) in v.into_iter() {
            let ch_path = format!("{}.{}", path, k);
            fields.insert(k, Types::from_value_path(criteria, &ch_path, v));
        }

        ObjectScheme { fields }
    }

    fn merge(&mut self, other: ObjectScheme) {
        // Set fileld nullable that not exists in other.
        for (k, ts) in self.fields.iter_mut() {
            if !other.contains_key(k) {
                ts.add(Type::Null);
            }
        }

        // Merge filelds
        for (k, ts) in other.into_iter() {
            if let Some(tt) = self.fields.get_mut(&k) {
                tt.merge(ts);
            } else {
                self.fields.insert(k, ts.with_null());
            }
        }
    }
}

impl From<Vec<(&str, Types)>> for ObjectScheme {
    fn from(v: Vec<(&str, Types)>) -> ObjectScheme {
        let mut fields = BTreeMap::new();

        for (k, v) in v.into_iter() {
            fields.insert(k.to_owned(), v);
        }

        ObjectScheme { fields }
    }
}

impl IntoIterator for ObjectScheme {
    type Item = (String, Types);
    type IntoIter = Box<dyn Iterator<Item = (String, Types)>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.fields.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::criteria::Criteria;

    use serde_json::from_str;

    #[test]
    fn type_from_unsigned_int() {
        let c = Criteria::new();
        let j: Value = from_str("1").unwrap();
        let t = Type::from_value(&c, j);
        assert_eq!(t, Type::U64);
    }

    #[test]
    fn type_from_signed_int() {
        let c = Criteria::new();
        let j: Value = from_str("-1").unwrap();
        let t = Type::from_value(&c, j);
        assert_eq!(t, Type::I64);
    }

    #[test]
    fn type_from_float_without_fractional_part() {
        let c = Criteria::new();
        let j: Value = from_str("1.0").unwrap();
        let t = Type::from_value(&c, j);
        assert_eq!(t, Type::Float);
    }

    #[test]
    fn type_from_float_with_fractional_part() {
        let c = Criteria::new();
        let j: Value = from_str("1.1").unwrap();
        let t = Type::from_value(&c, j);
        assert_eq!(t, Type::Float);
    }

    #[test]
    fn types_add_same_primitive() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!(true));
        let t = Type::from_value(&c, json!(false));

        ts.add(t);

        assert_eq!(ts.into_vec(), vec![Type::Bool]);
    }

    #[test]
    fn types_add_different_primitive() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!(true));
        let t = Type::from_value(&c, json!(1));

        ts.add(t);

        assert_eq!(ts.into_vec(), vec![Type::Bool, Type::U64]);
    }

    #[test]
    fn types_from_unsigned_and_signed_is_signed() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!(1));
        let t = Type::from_value(&c, json!(-1));

        ts.add(t);

        assert_eq!(ts.into_vec(), vec![Type::I64]);
    }

    #[test]
    fn types_from_unsigned_and_float_is_float() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!(1));
        let t = Type::from_value(&c, json!(1.0));

        ts.add(t);

        assert_eq!(ts.into_vec(), vec![Type::Float]);
    }

    #[test]
    fn types_from_signed_and_float_is_float() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!(-1));
        let t = Type::from_value(&c, json!(1.0));

        ts.add(t);

        assert_eq!(ts.into_vec(), vec![Type::Float]);
    }

    #[test]
    fn types_from_same_type_array() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!([0, true]));
        let t = Type::from_value(&c, json!([false, 123]));

        ts.add(t);

        assert_eq!(
            ts.into_vec(),
            vec![Type::Array(vec![Type::Bool, Type::U64].into())]
        );
    }

    #[test]
    fn types_from_different_type_array() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!([0, true]));
        let t = Type::from_value(&c, json!([null, 1.0]));

        ts.add(t);

        assert_eq!(
            ts.into_vec(),
            vec![Type::Array(
                vec![Type::Null, Type::Bool, Type::Float].into()
            )]
        );
    }

    #[test]
    fn types_from_objects() {
        let c = Criteria::new();
        let mut ts = Types::from_value_path(&c, "", json!({"a": 0, "b": true}));
        let t = Type::from_value(&c, json!({"a": 1, "c": "test"}));

        ts.add(t);

        assert_eq!(
            ts.into_vec(),
            vec![Type::Object(
                "".to_owned(),
                ObjectScheme::from(vec![
                    ("a", vec![Type::U64].into()),
                    ("b", vec![Type::Null, Type::Bool].into()),
                    ("c", vec![Type::Null, Type::String].into()),
                ])
            )]
        );
    }

    #[test]
    fn types_from_objects_has_different_type() {
        let mut c = Criteria::new();
        c.add("", "type");

        let mut ts = Types::from_value_path(&c, "", json!({"type": "B", "a": 0, "b": true}));
        let t = Type::from_value(&c, json!({"type": "C", "a": 1, "c": "test"}));

        ts.add(t);

        assert_eq!(
            ts.into_vec(),
            vec![
                Type::Object(
                    "B".to_owned(),
                    ObjectScheme::from(vec![
                        ("a", vec![Type::U64].into()),
                        ("b", vec![Type::Bool].into()),
                    ])
                ),
                Type::Object(
                    "C".to_owned(),
                    ObjectScheme::from(vec![
                        ("a", vec![Type::U64].into()),
                        ("c", vec![Type::String].into()),
                    ])
                )
            ]
        );
    }
}
