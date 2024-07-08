mod deserialize;
mod features;
mod key;
mod serialize;

#[cfg(feature = "value")]
pub mod value;

#[cfg(feature = "value")]
pub use value::{Value, Map};

#[derive(Debug, Clone)]
pub struct PreverveIgnoredFields<T, U> {
	pub value: T,
	pub ignored_fields: U,
}

pub trait DeserializeIgnoredFields<'de> {
	type Key: serde::Deserialize<'de>;
	type Value: serde::Deserialize<'de>;

	fn new() -> Self;
	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E>;
}

pub trait SerializeIgnoredFields {
	type Key: ?Sized + serde::Serialize;
	type Value: ?Sized + serde::Serialize;

	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)>;
}
