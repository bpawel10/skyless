use super::attribute::AttributesType;

pub trait AttributesBox {
    fn attributes(&self) -> &AttributesType;
}
