extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
mod process;

use chrono::NaiveDate;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

const FILENAME_PTN: &str = r"([a-z0-9-]*)-(\d{4}-\d{2}-\d{2})?.md";

#[derive(Debug, Serialize)]
pub struct Post {
    title: String,
    slug: String,
    date: String,
    metadata: Option<HashMap<String, String>>,
    html: String,
    plain: String,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn md_to_json(filename: &str, body: &str) -> String {
    // Init propper logging for panics
    console_error_panic_hook::set_once();

    // Prepare the regexp for matching valid filenames
    let filename_regex = Regex::new(&FILENAME_PTN).unwrap();

    // Process the files
    let (slug, date) = match process::parse_filename(&filename, &filename_regex) {
        Some((slug, date)) => (slug, date),
        None => (String::from(""), String::from("")),
    };
    if slug == "" || date == "" {
        println!("Cannot parse {:?}, unable to get slug or date", filename);
        return String::new();
    }
    let date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => date.to_string(),
        _ => return String::new(),
    };
    let metadata = process::extract_metadata(&body);
    let plain_body = process::extract_body_str(&body);
    if plain_body.is_none() {
        return String::new();
    }

    let plain = plain_body.unwrap();
    let html = process::md_to_html(&plain);
    let title: String = match &metadata {
        Some(m) => m.get("title").unwrap_or(&String::from("")).to_string(),
        None => String::from(""),
    };


    let json_str = process::post_to_json(&Post {
        title: title,
        metadata,
        slug,
        date,
        html,
        plain,
    });
    return json_str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_runs() -> Result<(), String> {
        md_to_json(
            "the-habit-of-reading-2018-01-22.md",
            r#"
            # Heading
            
            This is some test markdown.
        "#,
        );
        Ok(())
    }

    #[test]
    fn processor_returns_json() {
        let out = md_to_json(
            "the-habit-of-reading-2018-01-22.md",
            r#"
            # Heading
            
            This is some test markdown.
        "#,
        );
        assert!(out.len() > 0);
    }
}
