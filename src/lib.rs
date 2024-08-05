//! De-serialize and re-serialize a type while preserving ignored fields.
//!
//! Sometimes you may wish to preserve the ignored fields of something you are deserializing.
//! If you are in controll of the type, you could make use of the `#[serde(flatten)]` attribute:
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>>{
//! # use assert2::assert;
//! #
//! #[derive(serde::Deserialize, serde::Serialize)]
//! struct Thing {
//!   name: String,
//!
//!   #[serde(flatten)]
//!   extra_fields: serde_yaml::Mapping,
//! }
//! #
//! # let thing: Thing = serde_yaml::from_str("
//! #   name: Turbo Encabulator
//! #   base_plate:
//! #     prefabulated: true
//! #     material: aluminite
//! #   casing: malleable logarithmic
//! # ")?;
//! #
//! # assert!(thing.name == "Turbo Encabulator");
//! # assert!(thing.extra_fields["base_plate"]["prefabulated"] == true);
//! # assert!(thing.extra_fields["base_plate"]["material"] == "aluminite");
//! # assert!(thing.extra_fields["casing"] == "malleable logarithmic");
//! # Ok(())
//! # }
//! ```
//!
//! This crate can help you if you are *not* in control of the type.
//! You can wrap the type in [`PreserveIgnoredFields`]:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>>{
//! # use assert2::assert;
//! use serde_ignored_fields::PreserveIgnoredFields;
//!
//! #[derive(serde::Deserialize, serde::Serialize)]
//! struct Thing {
//!   name: String,
//! }
//!
//! let thing: PreserveIgnoredFields<Thing, serde_yaml::Mapping> = serde_yaml::from_str("
//!   name: Turbo Encabulator
//!   base_plate:
//!     prefabulated: true
//!     material: aluminite
//!   casing: malleable logarithmic
//! ")?;
//!
//! assert!(thing.value.name == "Turbo Encabulator");
//! assert!(thing.ignored_fields["base_plate"]["prefabulated"] == true);
//! assert!(thing.ignored_fields["base_plate"]["material"] == "aluminite");
//! assert!(thing.ignored_fields["casing"] == "malleable logarithmic");
//! # Ok(())
//! # }
//! ```
//!
//! If you enable the `schemars` feature, [`PreserveIgnoredFields<T, U>`] implements the [`schemars::JsonSchema`] trait.
//! It forwards directly to the [`schemars::JsonSchema`] implementation of `T`.
//!
//! # Limitations
//! Because `serde` does not provide first class support for capturing ignored fields, there are some limitations.
//!
//! ## Self-describing format
//! First, [`PreserveIgnoredFields`] only works with a self-describing format such as JSON, YAML or TOML.
//! This should not come as a surprise, and will not be a real limitation in practise
//! (how can you have ignored fields if the data format doesn't tell you what the fields are?).
//!
//! In [`serde`] terms: the [`serde::Deserializer`] must support [`serde::Deserializer::deserialize_any()`].
//!
//! ## Serialize/Deserialize implementations
//! Secondly, the type `T` being (de)serialized must be represented as a key/value map,
//! and it must call [`serde::Deserializer::deserialize_ignored_any()`] to deserialize ignored fields.
//! It must not produce an error when encountering an unknown field (so the type must not use `#[serde(deny_unknown_fields)]`).
//!
//! In particular, this means that it will not work for *externally* tagged enums, *internally* tagged enums and *untagged* enums.
//! Externally tagged enums are not always serialized as key/value maps (the serialization format controls their layout).
//! Internally and untagged enums have to look at fields before knowing which of the fields are actually going to be ignored.
//! This crate *does* work with adjectently tagged enums.
//!
//! It also means that it will not work for types that first deserialize into something like [`serde_json::Value`] before processing the value further.
//! When deserialized, the [`serde_json::Value`] uses all fields.
//! The next processing step may discard them again, but there is no way for [`PreserveIgnoredFields`] to know about this.
//!
//! In summary:
//! Using [`PreserveIgnoredFields`] with structs that use the standard serde derive macros from [`serde`] will work, as long as you did not use `#[serde(deny_unknown_fields)]`.
//! Using it with enums that use the standard derive macros will only work if they are *adjectently tagged* (they have a serde `tag = "..."` *and* `content = "..."` attribute).

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![cfg_attr(feature = "doc-cfg", feature(doc_cfg))]

mod deserialize;
mod features;
mod key;
mod serialize;

/// Wrapper to preserve ignored fields.
///
/// The wrapped type is stored in the [value][Self::value] field.
/// Ignored fields are stored in the [`ignored_fields`][Self::ignored_fields] field.
///
/// The `IgnoredFields` type has to implement [`DeserializeIgnoredFields`] for this type to implement [`serde::Deserialize`],
/// and it has to implement [`SerializeIgnoredFields`] for this type to implement [`serde::Serialize`].
///
/// Be sure the read the [main library documentation](crate) about the limitations.
///
/// If you enable the `schemars` feature, this type implements the [`schemars::JsonSchema`] trait.
/// The implementation forwards directly to the implementation of `T`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreserveIgnoredFields<T, U> {
	/// The wrapped value.
	pub value: T,

	/// The ignored fields.
	pub ignored_fields: U,
}

impl<T, U> PreserveIgnoredFields<T, U> {
	/// Create a new [`PreserveIgnoredFields`] struct from a wrapped value and the ignored fields.
	pub fn new(value: T, ignored_fields: U) -> Self {
		Self { value, ignored_fields }
	}
}

impl<T, U: Default> From<T> for PreserveIgnoredFields<T, U> {
	fn from(value: T) -> Self {
		Self::new(value, U::default())
	}
}

/// Trait for types that can collect ignored fields during deserialization.
pub trait DeserializeIgnoredFields<'de>: Default + std::fmt::Debug {
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
