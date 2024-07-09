use crate::key::Key;
use crate::{DeserializeIgnoredFields, PreserveIgnoredFields};

impl<'de, T, U> serde::de::Deserialize<'de> for PreserveIgnoredFields<T, U>
where
	T: serde::Deserialize<'de>,
	U: DeserializeIgnoredFields<'de>,
{
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let mut ignored_fields = U::default();
		let value = T::deserialize(Wrap::new(deserializer, &mut ignored_fields))?;
		Ok(Self { value, ignored_fields })
	}
}

/// Wrapper for a [`serde::Deserializer`] or [`serde::Visitor`] to preserve ignored fields of a map.
struct Wrap<'a, Inner, IgnoredFields> {
	/// The wrapped [`serde::Deserializer`] or [`serde::de::Visitor`].
	inner: Inner,

	/// Collection to add ignored fields into.
	ignored_fields: &'a mut IgnoredFields,
}

impl<'a, Inner, IgnoredFields> Wrap<'a, Inner, IgnoredFields> {
	/// Wrapper for a [`serde::Deserializer`] or [`serde::Visitor`].
	fn new(inner: Inner, ignored_fields: &'a mut IgnoredFields) -> Self {
		Self { inner, ignored_fields }
	}
}

/// Forward deserialize implementations to `self.inner` with some glue code.
macro_rules! forward_deserializer {
	(fn deserialize($self:ident, $visitor:ident) $pre:tt for [$($ident:ident),* $(,)?]) => {
		$(
			forward_deserializer!(@map_ident $ident($self, $visitor) $pre);
		)*
	};
	(@map_ident any            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_any             ($self, $visitor) $pre); };
	(@map_ident bool           ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_bool            ($self, $visitor) $pre); };
	(@map_ident i8             ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_i8              ($self, $visitor) $pre); };
	(@map_ident i16            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_i16             ($self, $visitor) $pre); };
	(@map_ident i32            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_i32             ($self, $visitor) $pre); };
	(@map_ident i64            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_i64             ($self, $visitor) $pre); };
	(@map_ident i128           ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_i128            ($self, $visitor) $pre); };
	(@map_ident u8             ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_u8              ($self, $visitor) $pre); };
	(@map_ident u16            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_u16             ($self, $visitor) $pre); };
	(@map_ident u32            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_u32             ($self, $visitor) $pre); };
	(@map_ident u64            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_u64             ($self, $visitor) $pre); };
	(@map_ident u128           ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_u128            ($self, $visitor) $pre); };
	(@map_ident f32            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_f32             ($self, $visitor) $pre); };
	(@map_ident f64            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_f64             ($self, $visitor) $pre); };
	(@map_ident char           ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_char            ($self, $visitor) $pre); };
	(@map_ident str            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_str             ($self, $visitor) $pre); };
	(@map_ident string         ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_string          ($self, $visitor) $pre); };
	(@map_ident bytes          ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_bytes           ($self, $visitor) $pre); };
	(@map_ident byte_buf       ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_byte_buf        ($self, $visitor) $pre); };
	(@map_ident option         ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_option          ($self, $visitor) $pre); };
	(@map_ident unit           ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_unit            ($self, $visitor) $pre); };
	(@map_ident unit_struct    ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_unit_struct     ($self, $visitor) (name: &'static str) $pre); };
	(@map_ident newtype_struct ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_newtype_struct  ($self, $visitor) (name: &'static str) $pre); };
	(@map_ident seq            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_seq             ($self, $visitor) $pre); };
	(@map_ident tuple          ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_tuple           ($self, $visitor) (len: usize) $pre); };
	(@map_ident tuple_struct   ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_tuple_struct    ($self, $visitor) (name: &'static str, len: usize) $pre); };
	(@map_ident map            ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_map             ($self, $visitor) $pre); };
	(@map_ident struct         ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_struct          ($self, $visitor) (name: &'static str, fields: &'static [&'static str]) $pre); };
	(@map_ident enum           ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_enum            ($self, $visitor) (name: &'static str, variants: &'static [&'static str]) $pre); };
	(@map_ident identifier     ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_identifier      ($self, $visitor) $pre); };
	(@map_ident ignored_any    ($self:ident, $visitor:ident) $pre:tt) => { forward_deserializer!(@ fn deserialize_ignored_any     ($self, $visitor) $pre); };

	(@ fn $ident:ident($self:ident, $visitor:ident) $(( $($arg_name:ident: $arg_type:ty),* $(,)?))? { $($pre:tt)* }) => {
		fn $ident<V: serde::de::Visitor<'de>>($self, $($($arg_name: $arg_type,)*)? $visitor: V) -> Result<V::Value, Self::Error> {
			$($pre)*
			$self.inner.$ident($($($arg_name,)*)? $visitor )
		}
	}
}

impl<'a, 'de, D, IgnoredFields> serde::de::Deserializer<'de> for Wrap<'a, D, IgnoredFields>
where
	D: serde::de::Deserializer<'de>,
	IgnoredFields: DeserializeIgnoredFields<'de>,
{
	type Error = D::Error;

	forward_deserializer!(
		fn deserialize(self, visitor) {
			let visitor = Wrap::new(visitor, self.ignored_fields);
		}
		for [
			any,
			bool,
			i8,
			i16,
			i32,
			i64,
			i128,
			u8,
			u16,
			u32,
			u64,
			u128,
			f32,
			f64,
			char,
			str,
			string,
			bytes,
			byte_buf,
			option,
			unit,
			unit_struct,
			newtype_struct,
			seq,
			tuple,
			tuple_struct,
			map,
			struct,
			enum,
			identifier,
			ignored_any,
		]
	);
}

impl<'a, 'de, V, IgnoredFields> serde::de::Visitor<'de> for Wrap<'a, V, IgnoredFields>
where
	V: serde::de::Visitor<'de>,
	IgnoredFields: DeserializeIgnoredFields<'de>,
{
	type Value = V::Value;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		// TODO: improve error message
		self.inner.expecting(formatter)
	}

	// fn visit_newtype_struct<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
	// 	self.inner.visit_newtype_struct(Wrap::new(deserializer, self.ignored_fields))
	// }

	fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
		self.inner.visit_map(MapAccess::new(map, self.ignored_fields))
	}
}

/// Wrapper for a [`serde::de::MapAccess`] to preserve ignored fields.
struct MapAccess<'a, 'de, M, IgnoredFields>
where
	M: serde::de::MapAccess<'de>,
	IgnoredFields: crate::DeserializeIgnoredFields<'de>,
{
	/// The parent [`serde::de::MapAccess`] being wrapped.
	parent: Option<M>,

	/// The collection to add ignored fields to.
	ignored_fields: &'a mut IgnoredFields,

	/// The previous key we encountered.
	last_key: Option<Key<'de>>,

	/// True if `next_key_seed()` was called, but not yet `next_value_seed()`.
	retrieved_key: bool,
}

impl<'a, 'de, M, IgnoredFields> MapAccess<'a, 'de, M, IgnoredFields>
where
	M: serde::de::MapAccess<'de>,
	IgnoredFields: crate::DeserializeIgnoredFields<'de>,
{
	/// Wrap an existing [`serde::de::MapAccess`].
	fn new(parent: M, ignored_fields: &'a mut IgnoredFields) -> Self {
		Self {
			parent: Some(parent),
			ignored_fields,
			last_key: None,
			retrieved_key: false,
		}
	}
}

impl<'a, 'de, M, U> serde::de::MapAccess<'de> for MapAccess<'a, 'de, M, U>
where
	M: serde::de::MapAccess<'de>,
	U: DeserializeIgnoredFields<'de>,
{
	type Error = M::Error;

	fn size_hint(&self) -> Option<usize> {
		match &self.parent {
			None => Some(0),
			Some(parent) => parent.size_hint(),
		}
	}

	fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		use serde::de::Error;

		// If someone doesn't call `next_value_seed()` for the last key we also add it to ignored fields.
		if self.retrieved_key {
			let key = self.last_key.take()
				.ok_or_else(|| Self::Error::custom("unsupported key type for ignored field"))?;
			let key = serde::Deserialize::deserialize(key.into_deserializer::<Self::Error>())?;
			let value = self.next_value()?;
			self.ignored_fields.insert(key, value)?;
		}

		let parent = match self.parent.as_mut() {
			Some(x) => x,
			None => return Ok(None),
		};

		match parent.next_key_seed(CaptureKey::new(seed, &mut self.last_key))? {
			Some(x) => {
				self.retrieved_key = true;
				Ok(Some(x))
			},
			None => {
				self.parent = None;
				Ok(None)
			},
		}
	}

	fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
		self.retrieved_key = false;

		let parent = self.parent.as_mut().expect("called `next_key_seed` without matching call to `next_key_seed`");
		let result = parent
			.next_value_seed(CaptureIgnored::new(seed, self.last_key.take(), self.ignored_fields));
		result
	}
}

impl<'a, 'de, M, IgnoredFields> Drop for MapAccess<'a, 'de, M, IgnoredFields>
where
	M: serde::de::MapAccess<'de>,
	IgnoredFields: crate::DeserializeIgnoredFields<'de>,
{
	fn drop(&mut self) {
		if let Some(mut parent) = self.parent.take() {
			loop {
				let (key, value) = match parent.next_entry() {
					Ok(None) => break,
					Ok(Some(x)) => x,
					Err(e) => {
						todo!();
					}
				};
				if let Err(e) = self.ignored_fields.insert::<M::Error>(key, value) {
					todo!();
				}
			}
		}
	}
}

/// Wrapper for a [`serde::de::DeserializeSeed`], [`serde::Deserializer`] and [`serde::de::Visitor`] to store the deserialized value in the `key` field.
///
/// Used to capture a key of a map entry that is being deserialized.
struct CaptureKey<'a, 'de, Inner> {
	/// The wrapped object.
	inner: Inner,

	/// The place to store the deserialized value.
	key: &'a mut Option<Key<'de>>,
}

impl<'a, 'de, Inner> CaptureKey<'a, 'de, Inner> {
	/// Wrap a [`serde::de::DeserializeSeed`], [`serde::Deserializer`] or [`serde::de::Visitor`].
	fn new(inner: Inner, key: &'a mut Option<Key<'de>>) -> Self {
		Self { inner, key }
	}
}

impl<'a, 'de, Seed> serde::de::DeserializeSeed<'de> for CaptureKey<'a, 'de, Seed>
where
	Seed: serde::de::DeserializeSeed<'de>,
{
	type Value = Seed::Value;

	fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
		self.inner.deserialize(CaptureKey::new(deserializer, self.key))
	}
}

impl<'a, 'de, D> serde::Deserializer<'de> for CaptureKey<'a, 'de, D>
where
	D: serde::Deserializer<'de>,
{
	type Error = D::Error;

	forward_deserializer!(
		fn deserialize(self, visitor) {
			let visitor = CaptureKey::new(visitor, self.key);
		} for [
			any,
			bool,
			i8,
			i16,
			i32,
			i64,
			i128,
			u8,
			u16,
			u32,
			u64,
			u128,
			f32,
			f64,
			char,
			str,
			string,
			bytes,
			byte_buf,
			option,
			unit,
			unit_struct,
			newtype_struct,
			seq,
			tuple,
			tuple_struct,
			map,
			struct,
			enum,
			identifier,
			ignored_any,
		]
	);

	fn is_human_readable(&self) -> bool {
		self.inner.is_human_readable()
	}
}

/// Macro to forward visitor function the `self.inner` while also capturing the value in `self.key`.
macro_rules! forward_visitor {
	($(($ident:ident, $type:ty, $variant:ident))*) => {
		$(
			fn $ident<E: serde::de::Error>(self, value: $type) -> Result<Self::Value, E> {
				*self.key = Some(Key::$variant(value.clone()));
				self.inner.$ident(value)
			}
		)*
	};
}

impl<'a, 'de, V> serde::de::Visitor<'de> for CaptureKey<'a, 'de, V>
where
	V: serde::de::Visitor<'de>,
{
	type Value = V::Value;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.inner.expecting(formatter)
	}

	forward_visitor!(
		(visit_bool, bool, Bool)
		(visit_i8, i8, I8)
		(visit_i16, i16, I16)
		(visit_i32, i32, I32)
		(visit_i64, i64, I64)
		(visit_i128, i128, I128)
		(visit_u8, u8, U8)
		(visit_u16, u16, U16)
		(visit_u32, u32, U32)
		(visit_u64, u64, U64)
		(visit_u128, u128, U128)
		(visit_f32, f32, F32)
		(visit_f64, f64, F64)
		(visit_char, char, Char)
		(visit_borrowed_str, &'de str, Str)
		(visit_string, String, String)
		(visit_borrowed_bytes, &'de [u8], Bytes)
		(visit_byte_buf, Vec<u8>, ByteBuf)
	);

	fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
		*self.key = Some(Key::String(value.into()));
		self.inner.visit_str(value)
	}

	fn visit_bytes<E: serde::de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
		*self.key = Some(Key::ByteBuf(value.into()));
		self.inner.visit_bytes(value)
	}

	// TODO: implement remaining visit functions to forward and record error details for CaptureIgnored.
}

/// Wrapper for a [`serde::de::DeserializeSeed`], [`serde::Deserializer`] and [`serde::de::Visitor`] to add `ignored_any` map entries to `self.ignored_fields`.
struct CaptureIgnored<'a, 'de, Inner, IgnoredFields> {
	/// The wrapped object.
	inner: Inner,

	/// The last deserialized key.
	key: Option<Key<'de>>,

	/// The collection of ignored fields.
	ignored_fields: &'a mut IgnoredFields,
}

impl<'a, 'de, Inner, IgnoredFields> CaptureIgnored<'a, 'de, Inner, IgnoredFields> {
	/// Wrap an object.
	fn new(inner: Inner, key: Option<Key<'de>>, ignored_fields: &'a mut IgnoredFields) -> Self {
		Self {
			inner,
			key,
			ignored_fields,
		}
	}
}

impl<'a, 'de, Seed, IgnoredFields> serde::de::DeserializeSeed<'de> for CaptureIgnored<'a, 'de, Seed, IgnoredFields>
where
	Seed: serde::de::DeserializeSeed<'de>,
	IgnoredFields: DeserializeIgnoredFields<'de>,
{
	type Value = Seed::Value;

	fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
		self.inner
			.deserialize(CaptureIgnored::new(deserializer, self.key, self.ignored_fields))
	}
}

impl<'a, 'de, D, IgnoredFields> serde::Deserializer<'de> for CaptureIgnored<'a, 'de, D, IgnoredFields>
where
	D: serde::Deserializer<'de>,
	IgnoredFields: DeserializeIgnoredFields<'de>,
{
	type Error = D::Error;

	forward_deserializer!(
		fn deserialize(self, visitor) { } for [
			// any,
			bool,
			i8,
			i16,
			i32,
			i64,
			i128,
			u8,
			u16,
			u32,
			u64,
			u128,
			f32,
			f64,
			char,
			str,
			string,
			bytes,
			byte_buf,
			option,
			unit,
			unit_struct,
			newtype_struct,
			seq,
			tuple,
			tuple_struct,
			map,
			struct,
			enum,
			identifier,
		]
	);

	fn deserialize_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		eprintln!("CaptureIgnored::deserialize_any");
		self.inner.deserialize_any(visitor)
	}

	fn deserialize_ignored_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		use serde::de::{Deserialize, Error};

		// TODO: better error
		let key = self
			.key
			.ok_or_else(|| Self::Error::custom("unsupported key type for ignored field"))?
			.into_deserializer();
		let key = IgnoredFields::Key::deserialize(key)?;
		let value = IgnoredFields::Value::deserialize(self.inner)?;
		self.ignored_fields.insert(key, value)?;
		visitor.visit_unit()
	}

	fn is_human_readable(&self) -> bool {
		self.inner.is_human_readable()
	}
}
