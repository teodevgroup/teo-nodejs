use napi::{Env, JsUnknown, Result};
use teo::prelude::InterfaceEnumVariant as OriginalInterfaceEnumVariant;

#[napi(js_name = "InterfaceEnumVariant")]
pub struct InterfaceEnumVariant {
    pub(crate) original: OriginalInterfaceEnumVariant
}

pub fn teo_interface_enum_variant_to_js_any(interface_enum_variant: &OriginalInterfaceEnumVariant, env: &Env) -> Result<JsUnknown> {
    let instance = InterfaceEnumVariant { original: interface_enum_variant.clone() }.into_instance(*env)?;
    Ok(instance.as_object(*env).into_unknown())
}