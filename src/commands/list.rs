use crate::cli::{Cli, ListArgs, OutputFormatArg};
use crate::config::{Config, OutputFormat, Project};
use crate::error::Result;
use crate::output::{Formatter, JsonFormatter, TableFormatter, YamlFormatter};

pub fn execute(args: &ListArgs, cli: &Cli) -> Result<()> {
    let config = Config::load_or_default()?;

    let mut projects: Vec<&Project> = config.projects.iter().collect();

    if let Some(ref hosting) = args.hosting {
        projects.retain(|p| &p.hosting == hosting);
    }

    if args.worktree {
        projects.retain(|p| p.is_worktree);
    }

    if args.no_worktree {
        projects.retain(|p| !p.is_worktree);
    }

    if let Some(ref search) = args.search {
        let search_lower = search.to_lowercase();
        projects.retain(|p| {
            p.name.to_lowercase().contains(&search_lower)
                || p.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&search_lower))
                    .unwrap_or(false)
        });
    }

    let projects_owned: Vec<Project> = projects.into_iter().cloned().collect();

    let output_format = if let Some(ref fmt) = cli.output {
        match fmt {
            OutputFormatArg::Table => OutputFormat::Table,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Yaml => OutputFormat::Yaml,
        }
    } else {
        config.settings.default_output_format
    };

    let formatter: Box<dyn Formatter> = match output_format {
        OutputFormat::Table => Box::new(TableFormatter),
        OutputFormat::Json => Box::new(JsonFormatter),
        OutputFormat::Yaml => Box::new(YamlFormatter),
    };

    let output = formatter.format(&projects_owned)?;
    println!("{}", output);

    Ok(())
}
