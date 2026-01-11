use crate::config::Project;
use crate::error::Result;
use crate::output::Formatter;

pub struct YamlFormatter;

impl Formatter for YamlFormatter {
    fn format(&self, projects: &[Project]) -> Result<String> {
        let yaml = serde_yaml::to_string(projects)?;
        Ok(yaml)
    }
}
