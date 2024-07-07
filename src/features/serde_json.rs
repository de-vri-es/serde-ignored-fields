impl<'de> crate::DeserializeIgnoredFields<'de> for serde_json::Map<String, serde_json::Value> {
	type Key = String;
	type Value = serde_json::Value;

	fn new() -> Self {
		Self::new()
	}

	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E> {
		use serde_json::map::Entry;
		match self.entry(key) {
			Entry::Vacant(x) => {
				x.insert(value);
				Ok(())
			}
			Entry::Occupied(x) => {
				Err(E::custom(format!("duplicate field: {:?}", x.key())))
			}
		}
	}
}

impl crate::SerializeIgnoredFields for serde_json::Map<String, serde_json::Value> {
	type Key = String;
	type Value = serde_json::Value;

	fn len(&self) -> usize {
		serde_json::Map::len(self)
	}

	fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
		serde_json::Map::iter(self)
	}
}
