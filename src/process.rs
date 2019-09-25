use pulldown_cmark::{html, Parser};
use regex::Regex;
use serde_json::to_string_pretty;
use std::collections::HashMap;

const FRONTMATTER_VALS_PTN: &str = r#"(?ms)^([a-zA-Z]+): ?"?([^"]*)"?$"#;
const FRONTMATTER_BODY_SPLIT_PTN: &str = r#"(?m)^---\n\s?([\S\s]+?)\s?---$\n([\s\S]*)$"#;

// Validate the filename and return a tuple containing it's parts (slug and date)
pub fn parse_filename(filename_str: &str, matcher: &regex::Regex) -> Option<(String, String)> {
	let filename_valid = matcher.is_match(filename_str);
	if filename_valid {
		let parts = matcher.captures(&filename_str).unwrap();
		let slug = parts.get(1)?.as_str();
		let date = parts.get(2)?.as_str();
		Some((String::from(slug), String::from(date)))
	} else {
		None
	}
}

fn extract_frontmatter_str(body: &str) -> Option<String> {
	let frontmatter_matcher = Regex::new(FRONTMATTER_BODY_SPLIT_PTN).unwrap();
	let has_frontmatter = frontmatter_matcher.is_match(&body);

	if has_frontmatter {
		super::log("Frontmatter recognised");
	}
	super::log(&body);

	if !has_frontmatter {
		return None;
	}
	let body_parts = frontmatter_matcher.captures(&body)?;
	let frontmatter_str = body_parts.get(1)?.as_str();
	super::log(&frontmatter_str);
	Some(String::from(frontmatter_str))
}

pub fn extract_body_str(body_full: &str) -> Option<String> {
	let body_matcher = Regex::new(FRONTMATTER_BODY_SPLIT_PTN).unwrap();
	let mut body_str = "";

	// With posts that have no frontmatter metadata, we can just use the full body
	// string. When frontmatter is present, we will have to trim it out of the
	// final markdown string
	let has_frontmatter = body_matcher.is_match(&body_full);
	if has_frontmatter {
		let parts = body_matcher.captures(&body_full).unwrap();
		body_str = parts.get(2)?.as_str();
	}

	Some(body_str.trim().to_string())
}

// Pull the YAML frontmatter from the body string
pub fn extract_metadata(body_str: &str) -> Option<HashMap<String, String>> {
	let frontmatter_str = match extract_frontmatter_str(body_str) {
		Some(s) => s,
		_ => return None,
	};
	let meta_matcher = Regex::new(FRONTMATTER_VALS_PTN).unwrap();
	let frontmatter_valid = meta_matcher.is_match(&frontmatter_str);

	if !frontmatter_valid {
		return None;
	}

	let mut metadata = HashMap::new();
	let metadata_parts = meta_matcher.captures_iter(&frontmatter_str);
	for m in metadata_parts {
		let k = String::from(&m[1]).trim().to_string();
		let v = String::from(&m[2]).trim().to_string();
		metadata.insert(k, v);
	}

	Some(metadata)
}

pub fn md_to_html(md_string: &String) -> String {
	let parser = Parser::new(&md_string);
	let mut html_str = String::new();
	html::push_html(&mut html_str, parser);
	html_str
}

pub fn posts_vec_to_json(posts: &Vec<super::Post>) -> String {
	let mut json_str = String::new();
	for p in posts {
		let j = match to_string_pretty(p) {
			Ok(s) => s,
			_ => String::new(),
		};
		json_str = json_str + ",\n" + j.as_str();
	}
	String::from("[") + &json_str + "]"
}
