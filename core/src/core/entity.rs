use super::AttributesType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entity {
    pub attributes: AttributesType,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }
}

// TODO: move it to macros crate and define using #[proc_macro] instead
#[macro_export]
macro_rules! entity {
    ($($attribute:expr),*) => {
        {
            let mut attributes = HashMap::new();
            $(
                attributes.insert(
                    $attribute.as_name().to_string(),
                    Box::new($attribute) as Box<dyn Attribute>,
                );
            )*
            Entity { attributes }
        }
    };
}
