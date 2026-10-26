use std::any::{Any, TypeId};

pub fn id_of(value: Box<dyn Any>) -> TypeId {
    value.type_id()
}
