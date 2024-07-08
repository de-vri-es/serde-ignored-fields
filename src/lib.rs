mod deserialize;
mod features;
mod key;
mod serialize;

#[derive(Debug, Clone, Default)]
pub struct PreverveIgnoredFields<T, U> {
	pub value: T,
	pub ignored_fields: U,
}

impl<T, U> PreverveIgnoredFields<T, U> {
	pub fn new(value: T, ignored_fields: U) -> Self {
		Self { value, ignored_fields }
	}
}

impl<T, U: Default> From<T> for PreverveIgnoredFields<T, U> {
	fn from(value: T) -> Self {
		Self::new(value, U::default())
	}
}

pub trait DeserializeIgnoredFields<'de>: Default {
	type Key: serde::Deserialize<'de>;
	type Value: serde::Deserialize<'de>;

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
