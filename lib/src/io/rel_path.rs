use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct RelPath {
     components: Vec<String>,
}

impl Default for RelPath {
    fn default() -> Self {
        Self::new()
    }
}

impl RelPath {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn push(&self, component: &str) -> Self {
        let mut new_components = self.components.clone();
        new_components.push(component.to_string());
        Self {
            components: new_components,
        }
    }

    pub fn with_base_path(&self, base_path: &Path) -> PathBuf {
        let mut path = base_path.to_path_buf();
        path.extend(self.components.iter().cloned());
        path
    }

    pub fn with_base_url(&self, base_url: &Url) -> Url {
        let mut url = base_url.clone();

        url.path_segments_mut()
            .unwrap_or_else(|_| panic!("Bad base url: {:?}", base_url))
            .pop_if_empty()
            .extend(self.components.iter());

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_rel_path_fs() {
        let rel_path = RelPath::new().push("folder").push("file.txt");
        let base_path = Path::new("/base/path");
        let absolute_path = rel_path.with_base_path(base_path);
        assert_eq!(absolute_path, PathBuf::from("/base/path/folder/file.txt"));
    }

    #[test]
    fn test_rel_path_url_trailing_slash() {
        let rel_path = RelPath::new().push("folder").push("file.txt");
        let base_url = Url::parse("http://example.com/base/path/").unwrap();
        let absolute_url = rel_path.with_base_url(&base_url);
        assert_eq!(
            absolute_url.as_str(),
            "http://example.com/base/path/folder/file.txt"
        );
    }

    #[test]
    fn test_rel_path_url() {
        let rel_path = RelPath::new().push("folder").push("file.txt");
        let base_url = Url::parse("http://example.com/base/path").unwrap();
        let absolute_url = rel_path.with_base_url(&base_url);
        assert_eq!(
            absolute_url.as_str(),
            "http://example.com/base/path/folder/file.txt"
        );
    }
}
