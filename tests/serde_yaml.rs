#![cfg(feature = "toml")]

use assert2::{assert, let_assert};
use indoc::indoc;
use serde_ignored_fields::PreserveIgnoredFields;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Person {
	name: String,
	hobby: String,
}

fn yaml<T: serde::de::DeserializeOwned>(data: &str) -> Result<PreserveIgnoredFields<T, serde_yaml::Mapping>, serde_yaml::Error> {
	serde::Deserialize::deserialize(serde_yaml::Deserializer::from_str(data))
}

macro_rules! yaml {
	({ $($name:ident: $value:tt),* $(,)? }) => {
		{
			#[allow(unused_mut)]
			let mut map = serde_yaml::Mapping::new();
			$(map.insert(yaml!((::core::stringify!($name))), yaml!($value).into());)*
			map
		}
	};
	([ $($value:tt),* $(,)? ]) => {
		{
			#[allow(unused_mut)]
			let mut vec = Vec::<serde_yaml::Value>::new();
			$(vec.push(yaml!($value).into());)*
			vec
		}
	};
	($($name:ident: $value:tt),* $(,)?) => {
		yaml!({ $($name: $value,)* })
	};
	($value:tt) => {
		serde_yaml::Value::from($value)
	};
}

#[test]
fn deserialize_extra() {
	let_assert!(Ok(deserialized) = yaml::<Person>(indoc!(r#"
		name: Zohan
		hobby: hair-dressing
		glasses: false
		friends:
		- name: Scrappy
		  species: dog
		- name: Coco
		  species: dog
	"#)));

	assert!(deserialized.value == Person {
		name: "Zohan".into(),
		hobby: "hair-dressing".into(),
	});

	assert!(deserialized.ignored_fields == yaml! {
		glasses: false,
		friends: [
			{ name: "Scrappy", species: "dog" },
			{ name: "Coco", species: "dog" },
		],
	});
}

#[test]
fn serialize_extra() {
	let zohan = PreserveIgnoredFields {
		value: Person {
			name: "Zohan".to_string(),
			hobby: "hair-dressing".to_string(),
		},
		ignored_fields: yaml! {
			glasses: false,
			friends: [
				{ name: "Scrappy", species: "dog" },
				{ name: "Coco", species: "dog" },
			],
		},
	};

	let_assert!(Ok(serialized) = ::serde_yaml::to_string(&zohan));
	assert!(serialized == indoc!(r#"
		name: Zohan
		hobby: hair-dressing
		glasses: false
		friends:
		- name: Scrappy
		  species: dog
		- name: Coco
		  species: dog
	"#))
}
