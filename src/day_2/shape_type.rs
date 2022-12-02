#[derive(PartialEq, Debug)]
pub(crate) enum ShapeType {
    RockType,
    PaperType,
    ScissorsType,
}

pub(crate) trait GetType {
    fn get_type(&self) -> ShapeType;
}
