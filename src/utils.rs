use inquire::{ui::RenderConfig, Confirm};

pub fn confirm_prompt(question: &str) -> bool {
    static DEFAULT_ASNWER: bool = false;

    Confirm {
        message: question,
        starting_input: None,
        default: None,
        placeholder: Some("y|n"),
        help_message: None,
        error_message: "Choose 'yes' or 'no'".into(),
        formatter: &|ans| match ans {
            true => "yes".to_owned(),
            false => "no".to_owned(),
        },
        parser: &|ans| match ans {
            "y" | "yeah" | "yep" | "yes" => Ok(true),
            "n" | "nope" | "nah" | "no" => Ok(false),
            _ => Err(()),
        },
        default_value_formatter: &|def| match def {
            true => String::from("yes"),
            false => String::from("no"),
        },
        render_config: RenderConfig::default(),
    }
    .prompt()
    .unwrap_or(DEFAULT_ASNWER)
}
