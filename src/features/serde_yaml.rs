impl<'de> crate::DeserializeIgnoredFields<'de> for serde_yaml::Mapping {
	type Key = serde_yaml::Value;
	type Value = serde_yaml::Value;

	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E> {
		use serde_yaml::mapping::Entry;
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

impl crate::SerializeIgnoredFields for serde_yaml::Mapping {
	type Key = serde_yaml::Value;
	type Value = serde_yaml::Value;

	fn len(&self) -> usize {
		serde_yaml::Mapping::len(self)
	}

	fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
		serde_yaml::Mapping::iter(self)
	}
}
