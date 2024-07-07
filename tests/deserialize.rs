use assert2::{assert, let_assert};
use serde_ignored_fields::PreverveIgnoredFields;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
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
