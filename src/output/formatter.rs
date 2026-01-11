use crate::config::Project;
use crate::error::Result;

pub trait Formatter {
    fn format(&self, projects: &[Project]) -> Result<String>;
}
