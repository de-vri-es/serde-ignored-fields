# serde-ignored-fields

De-serialize and re-serialize a type while preserving ignored fields.

Sometimes you may wish to preserve the ignored fields of something you are deserializing.
If you are in controll of the type, you could make use of the `#[serde(flatten)]` attribute:
```rust
#
#[derive(serde::Deserialize, serde::Serialize)]
struct Thing {
  name: String,

  #[serde(flatten)]
  extra_fields: serde_yaml::Mapping,
}
#
#
```

This crate can help you if you are *not* in control of the type.
You can wrap the type in [`PreserveIgnoredFields`]:

```rust
use serde_ignored_fields::PreserveIgnoredFields;

#[derive(serde::Deserialize, serde::Serialize)]
struct Thing {
  name: String,
}

let thing: PreserveIgnoredFields<Thing, serde_yaml::Mapping> = serde_yaml::from_str("
  name: Turbo Encabulator
  base_plate:
    prefabulated: true
    material: aluminite
  casing: malleable logarithmic
")?;

assert!(thing.value.name == "Turbo Encabulator");
assert!(thing.ignored_fields["base_plate"]["prefabulated"] == true);
assert!(thing.ignored_fields["base_plate"]["material"] == "aluminite");
assert!(thing.ignored_fields["casing"] == "malleable logarithmic");
```

If you enable the `schemars` feature, [`PreserveIgnoredFields<T, U>`] implements the [`schemars::JsonSchema`] trait.
It forwards directly to the [`schemars::JsonSchema`] implementation of `T`.

## Limitations
Because `serde` does not provide first class support for capturing ignored fields, there are some limitations.

### Self-describing format
First, [`PreserveIgnoredFields`] only works with a self-describing format such as JSON, YAML or TOML.
This should not come as a surprise, and will not be a real limitation in practise
(how can you have ignored fields if the data format doesn't tell you what the fields are?).

In [`serde`] terms: the [`serde::Deserializer`] must support [`serde::Deserializer::deserialize_any()`].

### Serialize/Deserialize implementations
Secondly, the type `T` being (de)serialized must be represented as a key/value map,
and it must call [`serde::Deserializer::deserialize_ignored_any()`] to deserialize ignored fields.
It must not produce an error when encountering an unknown field (so the type must not use `#[serde(deny_unknown_fields)]`).

In particular, this means that it will not work for *externally* tagged enums, *internally* tagged enums and *untagged* enums.
Externally tagged enums are not always serialized as key/value maps (the serialization format controls their layout).
Internally and untagged enums have to look at fields before knowing which of the fields are actually going to be ignored.
This crate *does* work with adjectently tagged enums.

It also means that it will not work for types that first deserialize into something like [`serde_json::Value`] before processing the value further.
When deserialized, the [`serde_json::Value`] uses all fields.
The next processing step may discard them again, but there is no way for [`PreserveIgnoredFields`] to know about this.

In summary:
Using [`PreserveIgnoredFields`] with structs that use the standard serde derive macros from [`serde`] will work, as long as you did not use `#[serde(deny_unknown_fields)]`.
Using it with enums that use the standard derive macros will only work if they are *adjectently tagged* (they have a serde `tag = "..."` *and* `content = "..."` attribute).

[`PreserveIgnoredFields`]: https://docs.rs/serde-ignored-fields/latest/serde_ignored_fields/struct.PreserveIgnoredFields.html
[`schemars::JsonSchema`]: https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html
[`serde`]: https://docs.rs/serde/
[`serde::Deserializer`]: https://docs.rs/serde/latest/serde/trait.Deserializer.html
[`serde::Deserializer::deserialize_any()`]: https://docs.rs/serde/latest/serde/trait.Deserializer.html#tymethod.deserialize_any
[`serde::Deserializer::deserialize_ignored_any()`]: https://docs.rs/serde/latest/serde/trait.Deserializer.html#tymethod.deserialize_ignored_any
[`serde_json::Value`]: https://docs.rs/serde_json/latest/serde_json/enum.Value.html
