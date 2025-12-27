use std::collections::HashMap;
use std::hash::Hash;

pub struct DiffResult<V> {
    pub added: Vec<V>,
    pub removed: Vec<V>,
    pub same: Vec<(V, V)>,
}

pub fn diff_iterators<V, K: PartialEq + Eq + Hash>(
    old: impl IntoIterator<Item = V>,
    new: impl IntoIterator<Item = V>,
    key_selector: impl Fn(&V) -> K,
) -> DiffResult<V> {
    let mut old_map = iter_to_map(old, &key_selector);

    let mut new_map = iter_to_map(new, &key_selector);

    let added: Vec<_> = new_map
        .extract_if(|path, _| !old_map.contains_key(path))
        .map(|(_, v)| v)
        .collect();

    let removed: Vec<_> = old_map
        .extract_if(|path, _| !new_map.contains_key(path))
        .map(|(_, v)| v)
        .collect();

    let same: Vec<_> = new_map
        .into_iter()
        .map(|(path, right_part)| (old_map.remove(&path).expect("Should exist"), right_part))
        .collect();

    if !old_map.is_empty() {
        unreachable!("Old map should be empty after processing all new entries");
    }

    DiffResult {
        added,
        removed,
        same,
    }
}

fn iter_to_map<V, K: PartialEq + Eq + Hash>(
    items: impl IntoIterator<Item = V>,
    key_selector: impl Fn(&V) -> K,
) -> HashMap<K, V> {
    items.into_iter().map(|e| (key_selector(&e), e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diff_iterators() {
        let old = vec!["a", "b", "c"];
        let new = vec!["b", "c", "d"];
        let diff = diff_iterators(old, new, |s| *s);
        assert_eq!(diff.added, vec!["d"]);
        assert_eq!(diff.removed, vec!["a"]);
        assert!(diff.same.contains(&("b", "b")));
        assert!(diff.same.contains(&("c", "c")));
    }
}
