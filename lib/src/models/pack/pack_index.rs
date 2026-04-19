use crate::io::fs::util::clean_path::clean_path;
use crate::models::index::index_node::IndexNode;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackIndex {
    pub addons: Vec<IndexNode>,
    pub pack_name: String,
}

impl PackIndex {
    pub fn get_addon_names(&self) -> HashSet<&str> {
        self.addons.iter().map(|node| node.name.as_ref()).collect()
    }

    pub fn get_addon_paths(&self, base_path: &Path) -> Vec<String> {
        self.addons
            .iter()
            .map(|node| {
                base_path
                    .join(&node.name)
                    .canonicalize()
                    .expect("Failed to canonicalize path")
            })
            .map(clean_path)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::index::index_node::{IndexNode, NodeKind};
    use std::fs;

    #[test]
    fn test_pack_index_get_addon_names() {
        let node1 = IndexNode {
            name: "addon1".to_string(),
            checksum: vec![],
            kind: NodeKind::Folder(vec![]),
        };
        let node2 = IndexNode {
            name: "addon2".to_string(),
            checksum: vec![],
            kind: NodeKind::Folder(vec![]),
        };

        let index = PackIndex {
            addons: vec![node1, node2],
            pack_name: "test_pack".to_string(),
        };

        let names = index.get_addon_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains("addon1"));
        assert!(names.contains("addon2"));
    }

    #[test]
    fn test_pack_index_get_addon_paths() {
        use crate::util::test_utils::TestTempDir;
        let temp_dir = TestTempDir::new("test_pack_index_get_addon_paths");
        let base_path = temp_dir.path();

        // Create the dummy directories so canonicalize succeeds
        fs::create_dir_all(base_path.join("addon1")).unwrap();
        fs::create_dir_all(base_path.join("addon2")).unwrap();

        let node1 = IndexNode {
            name: "addon1".to_string(),
            checksum: vec![],
            kind: NodeKind::Folder(vec![]),
        };
        let node2 = IndexNode {
            name: "addon2".to_string(),
            checksum: vec![],
            kind: NodeKind::Folder(vec![]),
        };

        let index = PackIndex {
            addons: vec![node1, node2],
            pack_name: "test_pack".to_string(),
        };

        let paths = index.get_addon_paths(base_path);

        // clean_path normalizes the canonical layout (which can be extended/prefixed logic on Win).
        assert_eq!(paths.len(), 2);
        assert!(paths[0].ends_with("addon1"));
        assert!(paths[1].ends_with("addon2"));
    }
}
