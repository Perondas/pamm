use super::context::ScriptTemplateContext;
use super::strategy::ScriptTemplateReplacementStrategy;

const DEPLOYED_PACK_PARAM_PLACEHOLDER: &str = "{DEPLOYED_PACK}";

pub(super) struct DeployedPackNameReplacementStrategy;

impl ScriptTemplateReplacementStrategy for DeployedPackNameReplacementStrategy {
    fn apply(&self, context: &ScriptTemplateContext<'_>, script_content: &mut String) {
        *script_content =
            script_content.replace(DEPLOYED_PACK_PARAM_PLACEHOLDER, context.pack_name);
    }
}

#[cfg(test)]
mod tests {
    use super::super::context::ScriptTemplateContext;
    use super::super::strategy::ScriptTemplateReplacementStrategy;
    use super::DeployedPackNameReplacementStrategy;

    fn context() -> ScriptTemplateContext<'static> {
        ScriptTemplateContext::new("core", "some/template", "-mod=foo")
    }

    #[test]
    fn replaces_deployed_pack_placeholder() {
        let mut script = format!("pack {}", super::DEPLOYED_PACK_PARAM_PLACEHOLDER);

        DeployedPackNameReplacementStrategy.apply(&context(), &mut script);

        assert_eq!(script, "pack core");
    }

    #[test]
    fn leaves_script_unchanged_when_placeholder_is_missing() {
        let mut script = "unchanged".to_string();

        DeployedPackNameReplacementStrategy.apply(&context(), &mut script);

        assert_eq!(script, "unchanged");
    }
}
