#![cfg(feature = "toml")]

use assert2::{assert, check, let_assert};
use indoc::indoc;
use serde_ignored_fields::PreverveIgnoredFields;

fn parse<T: serde::de::DeserializeOwned>(data: &str) -> Result<PreverveIgnoredFields<T, serde_yaml::Mapping>, serde_yaml::Error> {
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
fn error_on_externally_tagged_enum() {
	#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
	enum Enum {
		A {
			field1: String,
			field2: String,
		},
	}

	let value = PreverveIgnoredFields {
		value: Enum::A {
			field1: "Hello".into(),
			field2: "World!".into(),
		},
		ignored_fields: yaml! {
			extra: {
				something: 5,
			},
		},
	};

	let yaml = indoc!(r#"
		A:
		  field1: Hello
		  field2: World!
		extra:
		  something: 5
	"#);

	check!(let Err(_) = serde_yaml::to_string(&value));
	check!(let Err(_) = parse::<Enum>(yaml));
}

#[test]
#[ignore = "known broken"]
fn round_trip_internally_tagged_enum() {
	#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
	#[serde(tag = "type")]
	enum Enum {
		A {
			field1: String,
			field2: String,
		},
	}

	let value = PreverveIgnoredFields {
		value: Enum::A {
			field1: "Hello".into(),
			field2: "World!".into(),
		},
		ignored_fields: yaml! {
			extra: {
				something: 5,
			},
		},
	};

	let yaml = indoc!(r#"
		type: A
		field1: Hello
		field2: World!
		extra:
		  something: 5
	"#);

	let_assert!(Ok(serialized) = serde_yaml::to_string(&value));
	assert!(serialized == yaml);

	let_assert!(Ok(deserialized) = parse::<Enum>(yaml));
	assert!(deserialized.value == value.value);
	assert!(deserialized.ignored_fields == value.ignored_fields);
}

#[test]
fn round_trip_adjecently_tagged_enum() {
	#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
	#[serde(tag = "type", content = "data")]
	enum Enum {
		A {
			field1: String,
			field2: String,
		},
	}

	let value = PreverveIgnoredFields {
		value: Enum::A {
			field1: "Hello".into(),
			field2: "World!".into(),
		},
		ignored_fields: yaml! {
			extra: {
				something: 5,
			},
		},
	};

	let yaml = indoc!(r#"
		type: A
		data:
		  field1: Hello
		  field2: World!
		extra:
		  something: 5
	"#);

	let_assert!(Ok(serialized) = serde_yaml::to_string(&value));
	assert!(serialized == yaml);

	let_assert!(Ok(deserialized) = parse::<Enum>(yaml));
	assert!(deserialized.value == value.value);
	assert!(deserialized.ignored_fields == value.ignored_fields);
}

#[test]
#[ignore = "known broken"]
fn round_trip_untagged_enum() {
	#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
	#[serde(untagged)]
	enum Enum {
		A {
			field1: String,
			field2: String,
		},
	}

	let value = PreverveIgnoredFields {
		value: Enum::A {
			field1: "Hello".into(),
			field2: "World!".into(),
		},
		ignored_fields: yaml! {
			extra: {
				something: 5,
			},
		},
	};

	let yaml = indoc!(r#"
		field1: Hello
		field2: World!
		extra:
		  something: 5
	"#);

	let_assert!(Ok(serialized) = serde_yaml::to_string(&value));
	assert!(serialized == yaml);

	let_assert!(Ok(deserialized) = parse::<Enum>(yaml));
	assert!(deserialized.value == value.value);
	assert!(deserialized.ignored_fields == value.ignored_fields);
}
