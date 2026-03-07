pub struct OptionalAddon {
    pub name: String,
    pub enabled: bool,
}

impl OptionalAddon {
    pub(crate) fn new(name: String, enabled: bool) -> Self {
        Self { name, enabled }
    }
}
