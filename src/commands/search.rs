use crate::cli::{Cli, OutputFormatArg, SearchArgs};
use crate::config::{Config, OutputFormat, Project};
use crate::error::Result;
use crate::output::{Formatter, JsonFormatter, TableFormatter, YamlFormatter};

pub fn execute(args: &SearchArgs, cli: &Cli) -> Result<()> {
    let config = Config::load_or_default()?;

    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    };

    let mut projects: Vec<&Project> = config
        .projects
        .iter()
        .filter(|p| {
            let name_match = if args.ignore_case {
                p.name.to_lowercase().contains(&pattern)
            } else {
                p.name.contains(&pattern)
            };

            let desc_match = p
                .description
                .as_ref()
                .map(|d| {
                    if args.ignore_case {
                        d.to_lowercase().contains(&pattern)
                    } else {
                        d.contains(&pattern)
                    }
                })
                .unwrap_or(false);

            name_match || desc_match
        })
        .collect();

    if let Some(ref hosting) = args.hosting {
        projects.retain(|p| &p.hosting == hosting);
    }

    if args.worktree {
        projects.retain(|p| p.is_worktree);
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
