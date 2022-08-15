#[cfg(target_os = "macos")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::path::Path;
use dprint_plugin_typescript::{format_text, configuration::{resolve_config, ConfigurationBuilder}};
use dprint_core::configuration::{ConfigKeyMap, ConfigKeyValue, resolve_global_config, ResolveGlobalConfigOptions};
use napi::{JsObject, JsString, Result, JsNumber, JsBoolean, JsUnknown, ValueType};
use napi_derive::napi;

#[napi]
fn format(file_name: String, code: String, config: Option<JsObject>) -> Result<String> {
  let path = Path::new(&file_name);

  let config = match config {
    Some(obj) => {
      let mut c = ConfigKeyMap::new();
      c.insert("deno".into(), ConfigKeyValue::Bool(true));

      let properties = obj.get_property_names()?;
      let len = properties.get_named_property::<JsNumber>("length")?.get_uint32()?;
      for i in 0..len {
        let property = properties.get_element::<JsString>(i)?;
        let property_str = property.into_utf8()?;
        let k = property_str.into_owned()?;
        let value: JsUnknown = obj.get_property(property)?;
        let v = match value.get_type()? {
          ValueType::String => {
            let s = unsafe { value.cast::<JsString>() }.into_utf8()?;
            ConfigKeyValue::String(s.into_owned()?)
          },
          ValueType::Number => {
            ConfigKeyValue::Number(unsafe { value.cast::<JsNumber>() }.get_int32()?)
          },
          ValueType::Boolean => {
            ConfigKeyValue::Bool(unsafe { value.cast::<JsBoolean>() }.get_value()?)
          },
          _ => {
            return Err(napi::Error {
              status: napi::Status::InvalidArg,
              reason: format!("Unsupported type for configuration property {}", k)
            })
          }
        };

        c.insert(k, v);
      }
      let res = resolve_config(c, &resolve_global_config(ConfigKeyMap::new(), &ResolveGlobalConfigOptions::default()).config);
      if !res.diagnostics.is_empty() {
        let message = res.diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<String>>().join("\n  ");
        return Err(napi::Error {
          status: napi::Status::InvalidArg,
          reason: format!("Invalid configuration.\n  {}", message)
        })
      }
      res.config
    },
    _ => ConfigurationBuilder::new().deno().build()
  };

  match format_text(&path, &code, &config) {
    Ok(res) => Ok(res.unwrap_or(String::new())),
    Err(e) => {
      Err(napi::Error {
        reason: e.to_string(),
        status: napi::Status::GenericFailure
      })
    }
  }
}
