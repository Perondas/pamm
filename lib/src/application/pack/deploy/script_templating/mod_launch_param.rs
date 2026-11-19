use super::context::ScriptTemplateContext;
use super::strategy::ScriptTemplateReplacementStrategy;
use log::warn;

const MOD_LAUNCH_PARAM_PLACEHOLDER: &str = "{MOD_LAUNCH_PARAM}";

pub(super) struct ModLaunchParamReplacementStrategy;

impl ScriptTemplateReplacementStrategy for ModLaunchParamReplacementStrategy {
    fn apply(&self, context: &ScriptTemplateContext<'_>, script_content: &mut String) {
        if !script_content.contains(MOD_LAUNCH_PARAM_PLACEHOLDER) {
            warn!(
                "Template for script {:?} does not contain the placeholder {}",
                context.template_name, MOD_LAUNCH_PARAM_PLACEHOLDER
            );
        }

        *script_content =
            script_content.replace(MOD_LAUNCH_PARAM_PLACEHOLDER, context.mod_launch_param);
    }
}

#[cfg(test)]
mod tests {
    use super::super::context::ScriptTemplateContext;
    use super::super::strategy::ScriptTemplateReplacementStrategy;
    use super::ModLaunchParamReplacementStrategy;

    fn context() -> ScriptTemplateContext<'static> {
        ScriptTemplateContext::new("core", "some/template", "-mod=foo")
    }

    #[test]
    fn replaces_mod_launch_placeholder() {
        let mut script = format!("start {} end", super::MOD_LAUNCH_PARAM_PLACEHOLDER);

        ModLaunchParamReplacementStrategy.apply(&context(), &mut script);

        assert_eq!(script, "start -mod=foo end");
    }

    #[test]
    fn leaves_script_unchanged_when_placeholder_is_missing() {
        let mut script = "unchanged".to_string();

        ModLaunchParamReplacementStrategy.apply(&context(), &mut script);

        assert_eq!(script, "unchanged");
    }
}
