use crate::{PreverveIgnoredFields, SerializeIgnoredFields};

impl<T, IgnoredFields> serde::Serialize for PreverveIgnoredFields<T, IgnoredFields>
where
	T: serde::Serialize,
	IgnoredFields: SerializeIgnoredFields,
{
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.value.serialize(Serializer::new(serializer, &self.ignored_fields))
	}
}

/// Wraper for a [`serde::Serializer`] or [`serde::ser::SerializeMap`] to inject ignored fields.
struct Serializer<'a, Inner, IgnoredFields> {
	/// The wrapped serializer.
	inner: Inner,

	/// The ignored fields to add.
	ignored_fields: &'a IgnoredFields,
}

impl<'a, Inner, IgnoredFields> Serializer<'a, Inner, IgnoredFields> {
	/// Wrap a serializer.
	fn new(inner: Inner, ignored_fields: &'a IgnoredFields) -> Self {
		Self { inner, ignored_fields }
	}
}

impl<'a, S, IgnoredFields> serde::Serializer for Serializer<'a, S, IgnoredFields>
where
	S: serde::Serializer,
	IgnoredFields: SerializeIgnoredFields,
{
	type Ok = S::Ok;
	type Error = S::Error;
	type SerializeSeq = serde::ser::Impossible<S::Ok, S::Error>;
	type SerializeTuple = serde::ser::Impossible<S::Ok, S::Error>;
	type SerializeTupleStruct = serde::ser::Impossible<S::Ok, S::Error>;
	type SerializeTupleVariant = serde::ser::Impossible<S::Ok, S::Error>;
	type SerializeMap = Serializer<'a, S::SerializeMap, IgnoredFields>;
	type SerializeStruct = Serializer<'a, S::SerializeMap, IgnoredFields>;
	type SerializeStructVariant = Serializer<'a, S::SerializeMap, IgnoredFields>;

	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		use serde::ser::SerializeMap;
		let map = self.serialize_map(Some(0))?;
		map.end()
	}

	fn serialize_some<T: ?Sized + serde::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		serde::Serialize::serialize(value, self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		use serde::ser::SerializeMap;
		let map = self.serialize_map(Some(0))?;
		map.end()
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		use serde::ser::SerializeStruct;
		let ser = self.serialize_struct(name, 0)?;
		ser.end()
	}

	fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
		use serde::ser::SerializeStructVariant;
		let ser = self.serialize_struct_variant(name, variant_index, variant, 0)?;
		ser.end()
	}

	fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
		serde::Serialize::serialize(value, self)
	}

	fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		serde::Serialize::serialize(value, self)
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(serde::ser::Error::custom("invalid type: can only re-serialize a map or struct with ignored fields"))
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		let len = len.map(|x| x + self.ignored_fields.len());
		let map = self.inner.serialize_map(len)?;
		Ok(Serializer::new(map, self.ignored_fields))
	}

	fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		let map = self.inner.serialize_map(Some(len + self.ignored_fields.len()))?;
		Ok(Serializer::new(map, self.ignored_fields))
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_vartiant_name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		let map = self.inner.serialize_map(Some(len + self.ignored_fields.len()))?;
		Ok(Serializer::new(map, self.ignored_fields))
	}
}

impl<'a, M, IgnoredFields> serde::ser::SerializeMap for Serializer<'a, M, IgnoredFields>
where
	M: serde::ser::SerializeMap,
	IgnoredFields: SerializeIgnoredFields,
{
	type Ok = M::Ok;
	type Error = M::Error;

	fn serialize_key<T: ?Sized + serde::Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
		self.inner.serialize_key(key)
	}

	fn serialize_value<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.inner.serialize_value(value)
	}

	fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
	where
		K: ?Sized + serde::Serialize,
		V: ?Sized + serde::Serialize
	{
		self.inner.serialize_entry(key, value)
	}

	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		for (key, value) in self.ignored_fields.iter() {
			self.inner.serialize_entry(key, value)?
		}
		self.inner.end()
	}
}

impl<'a, M, IgnoredFields> serde::ser::SerializeStruct for Serializer<'a, M, IgnoredFields>
where
	M: serde::ser::SerializeMap,
	IgnoredFields: SerializeIgnoredFields,
{
	type Ok = M::Ok;
	type Error = M::Error;

	fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		self.inner.serialize_entry(key, value)
	}

	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		for (key, value) in self.ignored_fields.iter() {
			self.inner.serialize_entry(key, value)?
		}
		self.inner.end()
	}
}

impl<'a, M, IgnoredFields> serde::ser::SerializeStructVariant for Serializer<'a, M, IgnoredFields>
where
	M: serde::ser::SerializeMap,
	IgnoredFields: SerializeIgnoredFields,
{
	type Ok = M::Ok;
	type Error = M::Error;

	fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		self.inner.serialize_entry(key, value)
	}

	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		for (key, value) in self.ignored_fields.iter() {
			self.inner.serialize_entry(key, value)?
		}
		self.inner.end()
	}
}
