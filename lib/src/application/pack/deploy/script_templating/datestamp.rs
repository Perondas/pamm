use super::context::ScriptTemplateContext;
use super::strategy::ScriptTemplateReplacementStrategy;
use std::time::SystemTime;

const DATESTAMP_PLACEHOLDER: &str = "{DATESTAMP}";

pub(super) struct DatestampReplacementStrategy;

impl ScriptTemplateReplacementStrategy for DatestampReplacementStrategy {
    fn apply(&self, _context: &ScriptTemplateContext<'_>, script_content: &mut String) {
        let time = SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = time.into();
        let datestamp = datetime.format("%Y-%m-%d").to_string();

        *script_content = script_content.replace(DATESTAMP_PLACEHOLDER, &datestamp);
    }
}

#[cfg(test)]
mod tests {
    use super::super::context::ScriptTemplateContext;
    use super::super::strategy::ScriptTemplateReplacementStrategy;
    use super::DatestampReplacementStrategy;
    use chrono::NaiveDate;

    fn context() -> ScriptTemplateContext<'static> {
        ScriptTemplateContext::new("core", "some/template", "-mod=foo")
    }

    #[test]
    fn replaces_datestamp_placeholder() {
        let mut script = format!("deployed {}", super::DATESTAMP_PLACEHOLDER);

        DatestampReplacementStrategy.apply(&context(), &mut script);

        assert!(!script.contains(super::DATESTAMP_PLACEHOLDER));
        let date_part = script.strip_prefix("deployed ").unwrap();
        assert!(NaiveDate::parse_from_str(date_part, "%Y-%m-%d").is_ok());
    }

    #[test]
    fn leaves_script_unchanged_when_placeholder_is_missing() {
        let mut script = "unchanged".to_string();

        DatestampReplacementStrategy.apply(&context(), &mut script);

        assert_eq!(script, "unchanged");
    }
}
