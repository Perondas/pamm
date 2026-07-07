use super::context::ScriptTemplateContext;

pub(super) trait ScriptTemplateReplacementStrategy {
    fn apply(&self, context: &ScriptTemplateContext<'_>, script_content: &mut String);
}

