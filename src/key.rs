use std::marker::PhantomData;

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
}

impl<'de> Key<'de> {
	pub fn into_deserializer<E: serde::de::Error>(self) -> KeyDeserializer<'de, E> {
		KeyDeserializer {
			key: self,
			_error: PhantomData,
		}
	}
}

pub struct KeyDeserializer<'de, E> {
	key: Key<'de>,
	_error: PhantomData<fn() -> E>,
}

impl<'de, E: serde::de::Error> serde::de::Deserializer<'de> for KeyDeserializer<'de, E> {
	type Error = E;

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
		}
	}

	serde::forward_to_deserialize_any! {
		bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
		bytes byte_buf option unit unit_struct newtype_struct seq tuple
		tuple_struct map struct enum identifier ignored_any
	}
}
