impl<'de> crate::IgnoredFields<'de> for serde_json::Map<String, serde_json::Value> {
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
