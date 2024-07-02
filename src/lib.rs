mod key;
mod preserve_ignored_fields;

pub use preserve_ignored_fields::PreverveIgnoredFields;

pub trait IgnoredFields<'de> {
	type Key: serde::de::Deserialize<'de>;
	type Value: serde::de::Deserialize<'de>;

	fn new() -> Self;
	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E>;
}
