use napi::{Env, JsUnknown, Result};
use bson::oid::ObjectId as BsonObjectId;
#[napi(js_name = "ObjectId")]
pub struct ObjectId {
    pub(crate) value: BsonObjectId,
}


#[napi]
impl ObjectId {

    #[napi]
    pub fn to_string(&self) -> String {
        self.value.to_hex()
    }

    #[napi]
    pub fn from_string(string: String, env: Env) -> Result<JsUnknown> {
        match BsonObjectId::parse_str(&string) {
            Ok(value) => Ok(Self { value }.into_instance(env)?.as_object(env).into_unknown()),
            Err(_) => {
                env.throw_type_error("string doesn't represent valid ObjectId", None)?;
                Ok(env.get_undefined()?.into_unknown())
            }
        }
    }
}