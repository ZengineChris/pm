use crate::config::Project;
use crate::error::Result;
use crate::output::Formatter;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, projects: &[Project]) -> Result<String> {
        let json = serde_json::to_string_pretty(projects)?;
        Ok(json)
    }
}
