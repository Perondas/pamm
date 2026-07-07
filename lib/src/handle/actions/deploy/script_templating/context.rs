pub(super) struct ScriptTemplateContext<'a> {
    pub pack_name: &'a str,
    pub template_name: &'a str,
    pub mod_launch_param: &'a str,
}

impl<'a> ScriptTemplateContext<'a> {
    pub fn new(pack_name: &'a str, template_name: &'a str, mod_launch_param: &'a str) -> Self {
        Self {
            pack_name,
            template_name,
            mod_launch_param,
        }
    }
}
