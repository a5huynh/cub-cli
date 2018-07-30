/// Helper functions to parse CLI args
use clap;
use libcub::note::NoteStatus;

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
pub fn parse_limit(matches: &clap::ArgMatches) -> i32 {
    if matches.is_present("limit") {
        let limit_str = matches.value_of("limit").unwrap();
        return limit_str.parse::<i32>().expect("Limit needs to be a number.");
    }

    return 100;
}