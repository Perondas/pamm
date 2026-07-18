use crate::models::index::index_node::IndexNode;

pub trait KeyedFile {
    fn file_name(identifier: &str) -> String;
}

impl KeyedFile for IndexNode {
    fn file_name(identifier: &str) -> String {
        format!("{}.index.pamm", identifier)
    }
}