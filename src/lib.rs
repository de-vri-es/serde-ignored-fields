//! De-serialize and re-serialize a type while preserving ignored fields.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

mod deserialize;
mod features;
mod key;
mod serialize;

/// Wrapper to preserve ignored fields.
///
/// The wrapped type is stored in [`Self::value`].
/// Ignored fields are stored in [`Self::ignored_fields`].
///
/// Be sure the read the [main library documentation](crate) about the limitations.
#[derive(Debug, Clone, Default)]
pub struct PreverveIgnoredFields<T, U> {
	/// The wrapped value.
	pub value: T,

	/// The ignored fields.
	pub ignored_fields: U,
}

impl<T, U> PreverveIgnoredFields<T, U> {
	/// Create a new [`PreverveIgnoredFields`] struct from a wrapped value and the ignored fields.
	pub fn new(value: T, ignored_fields: U) -> Self {
		Self { value, ignored_fields }
	}
}

impl<T, U: Default> From<T> for PreverveIgnoredFields<T, U> {
	fn from(value: T) -> Self {
		Self::new(value, U::default())
	}
}

/// Trait for types that can collect ignored fields during deserialization.
pub trait DeserializeIgnoredFields<'de>: Default {
	/// The type of the key for the ignored fields.
	type Key: serde::Deserialize<'de>;

	/// The type of the value of ignored fields.
	type Value: serde::Deserialize<'de>;

	/// Insert an ignored field.
	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E>;
}

/// Trait for types that can be used to re-serialize ignored fields.
pub trait SerializeIgnoredFields {
	/// The type of the key for the ignored fields.
	type Key: ?Sized + serde::Serialize;

	/// The type of the value of ignored fields.
	type Value: ?Sized + serde::Serialize;

	/// The amount of ignored fields.
	fn len(&self) -> usize;

	/// Check if there are exactly 0 ignored fields.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Iterate over the ignored fields.
	fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)>;
}
