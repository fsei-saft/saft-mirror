use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static STRUCT_DEFS: Lazy<Mutex<HashMap<String, StructDescription>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
pub struct StructDescription {
    pub name: String,
    pub fields: Vec<StructFieldDescription>
}

#[derive(Clone, Debug)]
pub struct StructFieldDescription {
    pub name: String,
    pub ty: String
}

pub fn register_struct(s: StructDescription) {
    STRUCT_DEFS.lock().unwrap().insert(s.name.clone(), s);
}

pub fn get_struct(name: &str) -> Option<StructDescription> {
    STRUCT_DEFS.lock().unwrap().get(name).map(|v| v.clone())
}
