//! Compound serialization support.

pub(crate) mod map;
pub(crate) mod sequence;
pub(crate) mod structure;
pub(crate) mod structure_variant;
pub(crate) mod tuple;
pub(crate) mod tuple_struct;
pub(crate) mod tuple_variant;

pub use map::CbrSerializeMap;
pub use sequence::CbrSerializeSeq;
pub use structure::CbrSerializeStruct;
pub use structure_variant::CbrSerializeStructVariant;
pub use tuple::CbrSerializeTuple;
pub use tuple_struct::CbrSerializeTupleStruct;
pub use tuple_variant::CbrSerializeTupleVariant;
