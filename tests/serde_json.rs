#![cfg(feature = "toml")]

use assert2::{assert, let_assert};
use indoc::indoc;
use serde_ignored_fields::PreserveIgnoredFields;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Person {
	name: String,
	hobby: String,
}

fn json<T: serde::de::DeserializeOwned>(data: &str) -> Result<PreserveIgnoredFields<T, serde_json::Map<String, serde_json::Value>>, serde_json::Error> {
	serde::Deserialize::deserialize(&mut serde_json::Deserializer::from_str(data))
}

macro_rules! json {
	({ $($name:ident: $value:tt),* $(,)? }) => {
		{
			#[allow(unused_mut)]
			let mut map = serde_json::Map::new();
			$(map.insert(String::from(::core::stringify!($name)), json!($value).into());)*
			map
		}
	};
	([ $($value:tt),* $(,)? ]) => {
		{
			#[allow(unused_mut)]
			let mut vec = Vec::<serde_json::Value>::new();
			$(vec.push(json!($value).into());)*
			vec
		}
	};
	($($name:ident: $value:tt),* $(,)?) => {
		json!({ $($name: $value,)* })
	};
	($value:expr) => {
		serde_json::Value::from($value)
	};
}

#[test]
fn deserialize_extra() {
	let_assert!(Ok(deserialized) = json::<Person>(indoc!(r#"
	{
	  "name": "Zohan",
	  "hobby": "hair-dressing",
	  "glasses": false,
	  "friends": [
	    {"name": "Scrappy", "species": "dog"},
	    {"name": "Coco", "species": "dog"}
	  ]
	}
	"#)));

	assert!(deserialized.value == Person {
		name: "Zohan".into(),
		hobby: "hair-dressing".into(),
	});

	assert!(deserialized.ignored_fields == json! {
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
		ignored_fields: json! {
			glasses: false,
			friends: [
				{ name: "Scrappy", species: "dog" },
				{ name: "Coco", species: "dog" },
			],
		},
	};

	let_assert!(Ok(serialized) = ::serde_json::to_string_pretty(&zohan));
	assert!(serialized == indoc!(r#"
	{
	  "name": "Zohan",
	  "hobby": "hair-dressing",
	  "glasses": false,
	  "friends": [
	    {
	      "name": "Scrappy",
	      "species": "dog"
	    },
	    {
	      "name": "Coco",
	      "species": "dog"
	    }
	  ]
	}
	"#).trim())
}
