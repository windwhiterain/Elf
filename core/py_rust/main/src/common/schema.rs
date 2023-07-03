mod data;
mod structure;
use structure::*;
pub struct Schema {
    structure: Structure,
    data_descriptors: DataDescriptor,
}
