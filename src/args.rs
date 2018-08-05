/// Helper functions to parse CLI args
use clap;
use libcub::note::NoteStatus;

pub enum Limit {
    INFINITE,
    FINITE(i32)
}

/// Create a list of filters from the command line args.
pub fn parse_filters(matches: &clap::ArgMatches) -> Vec<NoteStatus> {
    let mut filters = Vec::new();

    if matches.is_present("filter") {
        let matched = matches.values_of("filter").unwrap();
        for filter in matched {
            match filter {
                "archived" => filters.push(NoteStatus::ARCHIVED),
                "normal" => filters.push(NoteStatus::NORMAL),
                "trashed" => filters.push(NoteStatus::TRASHED),
                // Simply ignore all other strings
                _ => {},
            }
        }
    }

    return filters;
}

/// Parse limit arg
pub fn parse_limit(matches: &clap::ArgMatches) -> Limit {
    if matches.is_present("all") {
        return Limit::INFINITE;
    }

    if matches.is_present("limit") {
        let limit_str = matches.value_of("limit").unwrap();
        return Limit::FINITE(limit_str.parse::<i32>().unwrap_or(100));
    }

    return Limit::FINITE(100);
}

#[cfg(test)]
mod tests {
    use clap::App;
    use libcub::note::NoteStatus;
    use args::{ Limit, parse_filters, parse_limit };

    #[test]
    fn test_parse_filters() {
        let yaml = load_yaml!("cli.yml");
        let app = App::from_yaml(yaml);
        let matches = app.get_matches_from(vec!["cub", "ls", "-f", "archived"]);

        let subcommand = matches.subcommand_matches("ls").unwrap();
        let filters = parse_filters(subcommand);
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0], NoteStatus::ARCHIVED);
    }

    #[test]
    fn test_parse_limit() {
        let yaml = load_yaml!("cli.yml");
        let app = App::from_yaml(yaml);

        // Testing a valid limit value
        let matches = app.get_matches_from(vec!["cub", "ls", "-l", "42"]);
        let subcommand = matches.subcommand_matches("ls").unwrap();
        let limit = parse_limit(subcommand);

        match limit {
            Limit::FINITE(val) => assert_eq!(val, 42),
            Limit::INFINITE => {}
        }
    }

    #[test]
    fn test_parse_limit_failure() {
        let yaml = load_yaml!("cli.yml");
        let app = App::from_yaml(yaml);

        // Testing an invalid limit value
        let matches = app.get_matches_from(vec!["cub", "ls", "-l", "cheese"]);
        let subcommand = matches.subcommand_matches("ls").unwrap();
        let limit = parse_limit(subcommand);

        match limit {
            Limit::FINITE(val) => assert_eq!(val, 100),
            Limit::INFINITE => {}
        }
    }
}