use teo::prelude::EnumVariant as TeoEnumVariant;

#[napi(js_name = "EnumVariant")]
pub struct EnumVariant {
    pub(crate) value: TeoEnumVariant
}

#[napi]
impl EnumVariant {
    #[napi]
    pub fn to_string(&self) -> String {
        self.value.value.clone()
    }
    
    #[napi]
    pub fn from_string(value: String) -> EnumVariant {
        Self { 
            value: TeoEnumVariant {
                value,
                args: None,
            } 
        }
    }
}

