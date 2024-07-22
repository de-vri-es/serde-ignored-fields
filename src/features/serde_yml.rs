impl<'de> crate::DeserializeIgnoredFields<'de> for serde_yml::Mapping {
	type Key = serde_yml::Value;
	type Value = serde_yml::Value;

	fn insert<E: serde::de::Error>(&mut self, key: Self::Key, value: Self::Value) -> Result<(), E> {
		use serde_yml::mapping::Entry;
		match self.entry(key) {
			Entry::Vacant(x) => {
				x.insert(value);
				Ok(())
			},
			Entry::Occupied(x) => Err(E::custom(format!("duplicate field: {:?}", x.key()))),
		}
	}
}

impl crate::SerializeIgnoredFields for serde_yml::Mapping {
	type Key = serde_yml::Value;
	type Value = serde_yml::Value;

	fn len(&self) -> usize {
		serde_yml::Mapping::len(self)
	}

	fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
		serde_yml::Mapping::iter(self)
	}
}
