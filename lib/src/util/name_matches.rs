/// Compare two pack names for identity. Pack names are folder names, and NTFS
/// and APFS are case-insensitive, so two names differing only in case would
/// share a single directory.
pub(crate) fn names_match(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}
