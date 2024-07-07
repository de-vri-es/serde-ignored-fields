use assert2::{assert, let_assert};
use serde_ignored_fields::PreverveIgnoredFields;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
struct Person {
	name: String,
	hobby: String,
}

#[test]
fn serialize_extra() {
	let value = PreverveIgnoredFields {
		value: Person {
			name: "Zohan".to_string(),
			hobby: "hair-dressing".to_string(),
		},
		ignored_fields: toml::toml! {
			glasses = false
			friends = [
				{name = "Scrappy", species = "dog"},
				{name = "Coco", species = "dog"},
			]
		}
	};

	let_assert!(Ok(serialized) = ::toml::to_string_pretty(&value));
	assert!(serialized == unindent(r#"
		name = "Zohan"
		hobby = "hair-dressing"
		glasses = false

		[[friends]]
		name = "Scrappy"
		species = "dog"

		[[friends]]
		name = "Coco"
		species = "dog"
	"#))
}


fn unindent(data: &str) -> String {
	let data = data.strip_prefix(|c| c == '\r' || c == '\n').unwrap_or(data);
	let data = data.strip_suffix(|c| c == '\r' || c == '\n').unwrap_or(data);
	let indent = match data.find(|c: char| !c.is_ascii_whitespace()) {
		Some(x) => data.split_at(x).0,
		None => return String::new(),
	};

	let mut output = String::with_capacity(data.len());
	for line in data.lines() {
		match line.strip_prefix(indent) {
			None => output.push_str(line),
			Some(line) => output.push_str(line),
		}
		output.push('\n');
	}

	loop {
		if let Some(last) = output.pop() {
			if !last.is_ascii_whitespace() {
				output.push(last);
				output.push('\n');
				break;
			}
		}
	}

	output
}
