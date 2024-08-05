/// Forwards directly to the [`schemars::JsonSchema`] implementation of `T`.
impl<T, IgnoredFields> schemars::JsonSchema for crate::PreserveIgnoredFields<T, IgnoredFields>
where
	T: schemars::JsonSchema,
{
	fn schema_name() -> String {
		T::schema_name()
	}

	fn is_referenceable() -> bool {
		T::is_referenceable()
	}

	fn schema_id() -> std::borrow::Cow<'static, str> {
		T::schema_id()
	}

	fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
		T::json_schema(gen)
	}

	fn _schemars_private_non_optional_json_schema(
		gen: &mut schemars::gen::SchemaGenerator,
	) -> schemars::schema::Schema {
		T::_schemars_private_non_optional_json_schema(gen)
	}

	fn _schemars_private_is_option() -> bool {
		T::_schemars_private_is_option()
	}
}
