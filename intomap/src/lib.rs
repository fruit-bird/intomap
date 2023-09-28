use std::collections::BTreeMap;

pub trait IntoMap {
    fn as_map(&self) -> BTreeMap<String, String>;
}
