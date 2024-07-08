#![cfg(feature = "toml")]

use assert2::{assert, let_assert};
use indoc::indoc;
use serde_ignored_fields::PreverveIgnoredFields;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Person {
	name: String,
	hobby: String,
}

fn toml<T: serde::de::DeserializeOwned>(data: &str) -> Result<PreverveIgnoredFields<T, toml::Table>, toml::de::Error> {
	serde::Deserialize::deserialize(toml::Deserializer::new(data))
}

#[test]
fn deserialize_extra() {
	let_assert!(Ok(deserialized) = toml::<Person>(r#"
		name = "Zohan"
		hobby = "hair-dressing"
		glasses = false
		friends = [
			{name = "Scrappy", species = "dog"},
			{name = "Coco", species = "dog"},
		]
	"#));

	assert!(deserialized.value == Person {
		name: "Zohan".into(),
		hobby: "hair-dressing".into(),
	});

	assert!(deserialized.ignored_fields == toml::toml! {
		glasses = false
		friends = [
			{name = "Scrappy", species = "dog"},
			{name = "Coco", species = "dog"},
		]
	});
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
	assert!(serialized == indoc!(r#"
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
