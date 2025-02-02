pub mod core;
pub mod error;
pub mod sync_protocol;
pub mod util;

pub mod preclude {
  pub use serde_json::value::Value as JsonValue;
  pub use yrs::block::Prelim;
  pub use yrs::types::{
    array::Array, Attrs, Delta as YrsDelta, EntryChange, GetString, Observable, ToJson,
    Value as YrsValue, *,
  };
  pub use yrs::*;

  pub use crate::core::array_wrapper::ArrayRefWrapper;
  pub use crate::core::collab::{Collab, CollabBuilder, CollabContext};
  pub use crate::core::collab_plugin::CollabPlugin;
  pub use crate::core::map_wrapper::CustomMapRef;
  pub use crate::core::map_wrapper::{MapRefExtension, MapRefWrapper};
  pub use crate::core::text_wrapper::TextRefWrapper;
  pub use crate::util::insert_json_value_to_map_ref;
}
