use super::{AttributesBox, AttributesType};
use std::collections::HashMap;

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

impl AttributesBox for Entity {
    fn attributes(&self) -> &AttributesType {
        &self.attributes
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
