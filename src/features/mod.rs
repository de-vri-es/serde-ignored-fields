#[cfg(feature = "serde_json")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "serde_json")))]
mod serde_json;

#[cfg(feature = "serde_yaml")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "serde_yaml")))]
mod serde_yaml;

#[cfg(feature = "serde_yml")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "serde_yml")))]
mod serde_yml;

#[cfg(feature = "toml")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "serde_toml")))]
mod toml;
