#[macro_export]
macro_rules! txt {
    ($prompt:expr, $default:expr) => {
        ::inquire::Text::new($prompt)
            .with_default($default)
            .prompt()?
    };
}

#[macro_export]
macro_rules! multiselect {
    ($prompt:expr, $opts:expr) => {{
        // validator enforces at least one selection
        ::inquire::MultiSelect::new($prompt, $opts)
            .with_validator(|s: &[::inquire::list_option::ListOption<&&str>]| {
                if s.is_empty() {
                    Ok(::inquire::validator::Validation::Invalid(
                        "Select at least one option".into(),
                    ))
                } else {
                    Ok(::inquire::validator::Validation::Valid)
                }
            })
            .prompt()?
            .into_iter()
            .map(|s| {
                s.to_string()
                    .parse::<PlatformTarget>()
                    .expect("invalid platform target")
            })
            .collect::<Vec<PlatformTarget>>()
    }};
}

#[macro_export]
macro_rules! confirm {
    ($prompt:expr, $help:expr) => {
        ::inquire::Confirm::new($prompt)
            .with_help_message($help)
            .prompt()?
    };
}
