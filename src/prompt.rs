use inquire::{required, Confirm, Editor, Select, Text};
use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::error::{Error, Result};

pub fn question(message: &str) -> Result<bool> {
    match Confirm::new(&format!("{} (Yes/No)", message)).prompt() {
        Ok(value) => Ok(value),
        Err(err) => Err(Error::InquireError(err)),
    }
}

pub fn text(message: &str) -> Result<String> {
    match Text::new(message)
        .with_validator(required!("this field is required"))
        .prompt()
    {
        Ok(value) => Ok(value),
        Err(err) => Err(Error::InquireError(err)),
    }
}

pub fn optional_text(message: &str) -> Result<Option<String>> {
    let value = match Text::new(message)
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                "none".to_owned()
            } else {
                submission.into()
            }
        })
        .prompt()
    {
        Ok(value) => value,
        Err(err) => return Err(Error::InquireError(err)),
    };

    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

#[cfg(not(windows))]
pub fn editor(message: &str) -> Result<String> {
    match Editor::new(message)
        .with_validator(required!("this field is required"))
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count <= 20 {
                submission.into()
            } else {
                let mut t: String = submission.chars().take(17).collect();
                t.push_str("...");
                t
            }
        })
        .prompt()
    {
        Ok(value) => Ok(value),
        Err(err) => Err(Error::InquireError(err)),
    }
}

// Windows might not work correctly editor, so we just return the text
#[cfg(windows)]
pub fn editor(message: &str) -> Result<String> {
    text(message)
}

pub fn select<T: IntoEnumIterator + Display>(message: &str) -> Result<T> {
    let items = T::iter().collect::<Vec<_>>();

    match Select::new(message, items).prompt() {
        Ok(value) => Ok(value),
        Err(err) => Err(Error::InquireError(err)),
    }
}
