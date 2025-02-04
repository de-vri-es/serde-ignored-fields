use std::marker::PhantomData;

/// A type used to tore keys during deserialization.
///
/// Not exposed directly to the user.
/// Instead, the key given to the user is deserialized from this type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Key<'de> {
	Bool(bool),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	F32(f32),
	F64(f64),
	Char(char),
	Str(&'de str),
	String(String),
	Bytes(&'de [u8]),
	ByteBuf(Vec<u8>),
	None,
	Some(Box<Key<'de>>),
	Unit,
	NewTypeStruct(Box<Key<'de>>),
	Seq,
	Map,
	Enum,
}

impl<'de> Key<'de> {
	/// Convert the key into a deserializer.
	pub fn into_deserializer<E: serde::de::Error>(self) -> KeyDeserializer<'de, E> {
		KeyDeserializer {
			key: self,
			_error: PhantomData,
		}
	}
}

/// Deserializer that consumes a [`Key`].
///
/// Used to attach a specific error type for use in deserialization.
pub struct KeyDeserializer<'de, E> {
	key: Key<'de>,
	_error: PhantomData<fn() -> E>,
}

impl<'de, E: serde::de::Error> serde::de::Deserializer<'de> for KeyDeserializer<'de, E> {
	type Error = E;

	serde::forward_to_deserialize_any! {
		bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
		bytes byte_buf option unit unit_struct newtype_struct seq tuple
		tuple_struct map struct enum identifier ignored_any
	}

	fn deserialize_any<V: serde::de::Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
		match self.key {
			Key::Bool(x) => v.visit_bool(x),
			Key::I8(x) => v.visit_i8(x),
			Key::I16(x) => v.visit_i16(x),
			Key::I32(x) => v.visit_i32(x),
			Key::I64(x) => v.visit_i64(x),
			Key::I128(x) => v.visit_i128(x),
			Key::U8(x) => v.visit_u8(x),
			Key::U16(x) => v.visit_u16(x),
			Key::U32(x) => v.visit_u32(x),
			Key::U64(x) => v.visit_u64(x),
			Key::U128(x) => v.visit_u128(x),
			Key::F32(x) => v.visit_f32(x),
			Key::F64(x) => v.visit_f64(x),
			Key::Char(x) => v.visit_char(x),
			Key::Str(x) => v.visit_borrowed_str(x),
			Key::String(x) => v.visit_string(x),
			Key::Bytes(x) => v.visit_borrowed_bytes(x),
			Key::ByteBuf(x) => v.visit_byte_buf(x),
			Key::None => v.visit_none(),
			Key::Some(x) => v.visit_some(x.into_deserializer()),
			Key::Unit => v.visit_unit(),
			Key::NewTypeStruct(x) => v.visit_newtype_struct(x.into_deserializer()),
			Key::Seq => Err(Self::Error::custom(
				"key of ingored field is a list, only primitive types are supported",
			)),
			Key::Map => Err(Self::Error::custom(
				"key of ingored field is a map, only primitive types are supported",
			)),
			Key::Enum => Err(Self::Error::custom(
				"key of ingored field is an enum, only primitive types are supported",
			)),
		}
	}
}
