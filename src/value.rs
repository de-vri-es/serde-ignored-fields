use std::marker::PhantomData;

use serde::de::IntoDeserializer;

pub type Map<'a> = indexmap::IndexMap<String, Value<'a>>;
pub type MapIteratorOwned<'a> = <Map<'a> as IntoIterator>::IntoIter;
pub type MapIteratorBorrowed<'a> = <&'a Map<'a> as IntoIterator>::IntoIter;

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
	Unit,
	None,
	Some(Box<Value<'a>>),
	BorrowedSome(&'a Value<'a>),
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
	Str(&'a str),
	String(String),
	Bytes(&'a [u8]),
	ByteBuf(Vec<u8>),
	Sequence(Vec<Value<'a>>),
	BorrowedSequence(&'a [Value<'a>]),
	Map(Map<'a>),
	BorrowedMap(&'a Map<'a>),
}

impl<'a> Value<'a> {
	pub fn borrow(&self) -> Value {
		match self {
			Self::Unit => Value::Unit,
			Self::None => Value::None,
			Self::Some(x) => Value::BorrowedSome(x),
			Self::BorrowedSome(x) => Value::BorrowedSome(x),
			Self::Bool(x) => Value::Bool(*x),
			Self::I8(x) => Value::I8(*x),
			Self::I16(x) => Value::I16(*x),
			Self::I32(x) => Value::I32(*x),
			Self::I64(x) => Value::I64(*x),
			Self::I128(x) => Value::I128(*x),
			Self::U8(x) => Value::U8(*x),
			Self::U16(x) => Value::U16(*x),
			Self::U32(x) => Value::U32(*x),
			Self::U64(x) => Value::U64(*x),
			Self::U128(x) => Value::U128(*x),
			Self::F32(x) => Value::F32(*x),
			Self::F64(x) => Value::F64(*x),
			Self::Char(x) => Value::Char(*x),
			Self::Str(x) => Value::Str(x),
			Self::String(x) => Value::Str(x.as_str()),
			Self::Bytes(x) => Value::Bytes(x),
			Self::ByteBuf(x) => Value::Bytes(x.as_slice()),
			Self::Sequence(x) => Value::BorrowedSequence(x),
			Self::BorrowedSequence(x) => Value::BorrowedSequence(x),
			Self::Map(x) => Value::BorrowedMap(x),
			Self::BorrowedMap(x) => Value::BorrowedMap(x),
		}
	}

	pub fn to_owned(&self) -> Value<'static> {
		match self {
			Self::Unit => Value::Unit,
			Self::None => Value::None,
			Self::Some(x) => Value::Some(Box::new((**x).to_owned())),
			Self::BorrowedSome(x) => Value::Some(Box::new((**x).to_owned())),
			Self::Bool(x) => Value::Bool(*x),
			Self::I8(x) => Value::I8(*x),
			Self::I16(x) => Value::I16(*x),
			Self::I32(x) => Value::I32(*x),
			Self::I64(x) => Value::I64(*x),
			Self::I128(x) => Value::I128(*x),
			Self::U8(x) => Value::U8(*x),
			Self::U16(x) => Value::U16(*x),
			Self::U32(x) => Value::U32(*x),
			Self::U64(x) => Value::U64(*x),
			Self::U128(x) => Value::U128(*x),
			Self::F32(x) => Value::F32(*x),
			Self::F64(x) => Value::F64(*x),
			Self::Char(x) => Value::Char(*x),
			Self::Str(x) => Value::String((*x).into()),
			Self::String(x) => Value::String(x.clone()),
			Self::Bytes(x) => Value::ByteBuf((*x).into()),
			Self::ByteBuf(x) => Value::ByteBuf(x.clone()),
			Self::Sequence(x) => Value::Sequence(x.iter().map(|x| x.to_owned()).collect()),
			Self::BorrowedSequence(x) => Value::Sequence(x.iter().map(|x| x.to_owned()).collect()),
			Self::Map(x) => Value::Map(x.iter().map(|(key, value)| (key.clone(), value.to_owned())).collect()),
			Self::BorrowedMap(x) => Value::Map(x.iter().map(|(key, value)| (key.clone(), value.clone().into_owned())).collect()),
		}
	}

	pub fn into_owned(self) -> Value<'static> {
		match self {
			Self::Unit => Value::Unit,
			Self::None => Value::None,
			Self::Some(x) => Value::Some(Box::new(x.into_owned())),
			Self::BorrowedSome(x) => Value::Some(Box::new((*x).to_owned())),
			Self::Bool(x) => Value::Bool(x),
			Self::I8(x) => Value::I8(x),
			Self::I16(x) => Value::I16(x),
			Self::I32(x) => Value::I32(x),
			Self::I64(x) => Value::I64(x),
			Self::I128(x) => Value::I128(x),
			Self::U8(x) => Value::U8(x),
			Self::U16(x) => Value::U16(x),
			Self::U32(x) => Value::U32(x),
			Self::U64(x) => Value::U64(x),
			Self::U128(x) => Value::U128(x),
			Self::F32(x) => Value::F32(x),
			Self::F64(x) => Value::F64(x),
			Self::Char(x) => Value::Char(x),
			Self::Str(x) => Value::String(x.into()),
			Self::String(x) => Value::String(x),
			Self::Bytes(x) => Value::ByteBuf(x.into()),
			Self::ByteBuf(x) => Value::ByteBuf(x),
			Self::Sequence(x) => Value::Sequence(x.into_iter().map(|x| x.into_owned()).collect()),
			Self::BorrowedSequence(x) => Value::Sequence(x.iter().map(|x| x.to_owned()).collect()),
			Self::Map(x) => Value::Map(x.into_iter().map(|(key, value)| (key, value.into_owned())).collect()),
			Self::BorrowedMap(x) => Value::Map(x.iter().map(|(key, value)| (key.clone(), value.clone().into_owned())).collect()),
		}
	}
}

impl<'de, E: serde::de::Error> serde::de::IntoDeserializer<'de, E> for Value<'de> {
	type Deserializer = ValueDeserializer<'de, E>;

	fn into_deserializer(self) -> Self::Deserializer {
		ValueDeserializer {
			inner: Wrap::new(self),
		}
	}
}

impl<'de, E: serde::de::Error> serde::de::IntoDeserializer<'de, E> for &'de Value<'de> {
	type Deserializer = ValueDeserializer<'de, E>;

	fn into_deserializer(self) -> Self::Deserializer {
		ValueDeserializer {
			inner: Wrap::new(self.borrow()),
		}
	}
}

pub struct ValueDeserializer<'de, E> {
	inner: Wrap<Value<'de>, E>,
}

struct Wrap<Inner, E> {
	inner: Inner,
	_error: PhantomData<fn () -> E>,
}

impl<Inner, E> Wrap<Inner, E> {
	fn new(inner: Inner) -> Self {
		Self {
			inner,
			_error: PhantomData,
		}
	}
}

impl<'de, E: serde::de::Error> serde::de::Deserializer<'de> for ValueDeserializer<'de, E> {
	type Error = E;

	fn deserialize_any<V: serde::de::Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
		match self.inner.inner {
			Value::Unit => v.visit_unit(),
			Value::None => v.visit_none(),
			Value::Some(x) => v.visit_some(x.into_deserializer()),
			Value::BorrowedSome(x) => v.visit_some(x.into_deserializer()),
			Value::Bool(x) => v.visit_bool(x),
			Value::I8(x) => v.visit_i8(x),
			Value::I16(x) => v.visit_i16(x),
			Value::I32(x) => v.visit_i32(x),
			Value::I64(x) => v.visit_i64(x),
			Value::I128(x) => v.visit_i128(x),
			Value::U8(x) => v.visit_u8(x),
			Value::U16(x) => v.visit_u16(x),
			Value::U32(x) => v.visit_u32(x),
			Value::U64(x) => v.visit_u64(x),
			Value::U128(x) => v.visit_u128(x),
			Value::F32(x) => v.visit_f32(x),
			Value::F64(x) => v.visit_f64(x),
			Value::Char(x) => v.visit_char(x),
			Value::Str(x) => v.visit_borrowed_str(x),
			Value::String(x) => v.visit_string(x),
			Value::Bytes(x) => v.visit_borrowed_bytes(x),
			Value::ByteBuf(x) => v.visit_byte_buf(x),
			Value::Sequence(x) => v.visit_seq(Wrap::new(x.into_iter())),
			Value::BorrowedSequence(x) => v.visit_seq(Wrap::new(x.iter())),
			Value::Map(x) => v.visit_map(Wrap::new(MapAccessOwned::new(x))),
			Value::BorrowedMap(x) => v.visit_map(Wrap::new(MapAccessBorrowed::new(x))),
		}
	}

	serde::forward_to_deserialize_any! {
		bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
		bytes byte_buf option unit unit_struct newtype_struct seq tuple
		tuple_struct map struct enum identifier ignored_any
	}
}

struct MapAccessOwned<'a> {
	iter: MapIteratorOwned<'a>,
	next_value: Option<Value<'a>>,
}

struct MapAccessBorrowed<'a> {
	iter: MapIteratorBorrowed<'a>,
	next_value: Option<&'a Value<'a>>,
}

impl<'a> MapAccessOwned<'a> {
	pub fn new(map: Map<'a>) -> Self {
		Self {
			iter: map.into_iter(),
			next_value: None,
		}
	}
}

impl<'a> MapAccessBorrowed<'a> {
	pub fn new(map: &'a Map<'a>) -> Self {
		Self {
			iter: map.into_iter(),
			next_value: None,
		}
	}
}

impl<'de, E> serde::de::SeqAccess<'de> for Wrap<std::vec::IntoIter<Value<'de>>, E>
where
	E: serde::de::Error,
{
	type Error = E;

	fn size_hint(&self) -> Option<usize> {
		Some(self.inner.len())
	}

	fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
		match self.inner.next() {
			None => Ok(None),
			Some(x) => Ok(Some(seed.deserialize(x.into_deserializer())?)),
		}
	}
}

impl<'de, E> serde::de::SeqAccess<'de> for Wrap<std::slice::Iter<'de, Value<'de>>, E>
where
	E: serde::de::Error,
{
	type Error = E;

	fn size_hint(&self) -> Option<usize> {
		Some(self.inner.len())
	}

	fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
		match self.inner.next() {
			None => Ok(None),
			Some(x) => Ok(Some(seed.deserialize(x.into_deserializer())?)),
		}
	}
}

impl<'de, E> serde::de::MapAccess<'de> for Wrap<MapAccessOwned<'de>, E>
where
	E: serde::de::Error,
{
	type Error = E;

	fn size_hint(&self) -> Option<usize> {
		Some(self.inner.iter.len())
	}

	fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		self.inner.next_value = None;
		match self.inner.iter.next() {
			None => Ok(None),
			Some((key, value)) => {
				self.inner.next_value = Some(value);
				Ok(Some(seed.deserialize(key.into_deserializer())?))
			}
		}
	}

	fn next_value_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<T::Value, Self::Error> {
		let value = self.inner.next_value.take()
			.expect("called next_value_seed without matching call to next_key_seed");
		seed.deserialize(value.into_deserializer())
	}

	fn next_entry_seed<K, V>(&mut self, key_seed: K, value_seed: V) -> Result<Option<(K::Value, V::Value)>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
		V: serde::de::DeserializeSeed<'de>,
	{
		self.inner.next_value = None;
		let (key, value) = match self.inner.iter.next() {
			Some(x) => x,
			None => return Ok(None),
		};
		let key = key_seed.deserialize(key.into_deserializer())?;
		let value = value_seed.deserialize(value.into_deserializer())?;
		Ok(Some((key, value)))
	}
}

impl<'de, E> serde::de::MapAccess<'de> for Wrap<MapAccessBorrowed<'de>, E>
where
	E: serde::de::Error,
{
	type Error = E;

	fn size_hint(&self) -> Option<usize> {
		Some(self.inner.iter.len())
	}

	fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		self.inner.next_value = None;
		match self.inner.iter.next() {
			None => Ok(None),
			Some((key, value)) => {
				self.inner.next_value = Some(value);
				Ok(Some(seed.deserialize(key.as_str().into_deserializer())?))
			}
		}
	}

	fn next_value_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<T::Value, Self::Error> {
		let value = self.inner.next_value.take()
			.expect("called next_value_seed without matching call to next_key_seed");
		seed.deserialize(value.into_deserializer())
	}

	fn next_entry_seed<K, V>(&mut self, key_seed: K, value_seed: V) -> Result<Option<(K::Value, V::Value)>, Self::Error>
	where
		K: serde::de::DeserializeSeed<'de>,
		V: serde::de::DeserializeSeed<'de>,
	{
		self.inner.next_value = None;
		let (key, value) = match self.inner.iter.next() {
			Some(x) => x,
			None => return Ok(None),
		};
		let key = key_seed.deserialize(key.as_str().into_deserializer())?;
		let value = value_seed.deserialize(value.into_deserializer())?;
		Ok(Some((key, value)))
	}
}

impl<'de> serde::Deserialize<'de> for Value<'de> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visit<'de>(PhantomData<fn(&'de ())>);

		impl<'de> serde::de::Visitor<'de> for Visit<'de> {
			type Value = Value<'de>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(formatter, "a value")
			}

			fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
				Ok(Value::Bool(v))
			}

			fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
				Ok(Value::I8(v))
			}

			fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
				Ok(Value::I16(v))
			}

			fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
				Ok(Value::I32(v))
			}

			fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
				Ok(Value::I64(v))
			}

			fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
				Ok(Value::I128(v))
			}

			fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
				Ok(Value::U8(v))
			}

			fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
				Ok(Value::U16(v))
			}

			fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
				Ok(Value::U32(v))
			}

			fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
				Ok(Value::U64(v))
			}

			fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
				Ok(Value::U128(v))
			}

			fn visit_f32<E: serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
				Ok(Value::F32(v))
			}

			fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
				Ok(Value::F64(v))
			}

			fn visit_char<E: serde::de::Error>(self, v: char) -> Result<Self::Value, E> {
				Ok(Value::Char(v))
			}

			fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
				Ok(Value::String(v.into()))
			}

			fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
				Ok(Value::Str(v))
			}

			fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
				Ok(Value::String(v))
			}

			fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
				Ok(Value::ByteBuf(v.into()))
			}

			fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
				Ok(Value::Bytes(v))
			}

			fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
				Ok(Value::ByteBuf(v))
			}

			fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
				Ok(Value::None)
			}

			fn visit_some<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
				let value = serde::Deserialize::deserialize(deserializer)?;
				Ok(Value::Some(value))
			}

			fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
				Ok(Value::Unit)
			}

			fn visit_newtype_struct<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
				serde::Deserialize::deserialize(deserializer)
			}

			fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
				let mut output = Vec::with_capacity(seq.size_hint().unwrap_or(0));
				while let Some(value) = seq.next_element()? {
					output.push(value);
				}
				Ok(Value::Sequence(output))
			}

			fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
				let mut output = Map::with_capacity(map.size_hint().unwrap_or(0));
				while let Some((key, value)) = map.next_entry()? {
					output.insert(key, value);
				}
				Ok(Value::Map(output))
			}
		}

		deserializer.deserialize_any(Visit(PhantomData))
	}
}
