use napi::{Env, JsUnknown, Result};
use teo::prelude::InterfaceEnumVariant as TeoInterfaceEnumVariant;

#[napi(js_name = "InterfaceEnumVariant")]
pub struct InterfaceEnumVariant {
    pub(crate) value: TeoInterfaceEnumVariant
}

pub fn teo_interface_enum_variant_to_js_any(interface_enum_variant: &TeoInterfaceEnumVariant, env: &Env) -> Result<JsUnknown> {
    let instance = InterfaceEnumVariant { value: interface_enum_variant.clone() }.into_instance(*env)?;
    Ok(instance.as_object(*env).into_unknown())
}