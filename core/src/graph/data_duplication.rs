use super::dependency;

pub struct DataDuplication {
    from: usize,
    to: usize,
    const_denpendencies: dependency::Const,
}
