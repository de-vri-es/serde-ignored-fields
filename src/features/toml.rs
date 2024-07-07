impl<'de> crate::IgnoredFields<'de> for toml::Table {
	type Key = String;
	type Value = toml::Value;

	fn new() -> Self {
		Self::new()
	}

	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E> {
		use toml::map::Entry;
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
