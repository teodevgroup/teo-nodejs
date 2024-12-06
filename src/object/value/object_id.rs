use napi::{Error, Result};
use bson::oid::ObjectId as BsonObjectId;

#[napi(js_name = "ObjectId")]
pub struct ObjectId {
    pub(crate) original: BsonObjectId,
}

#[napi]
impl ObjectId {

    #[napi(constructor)]
    pub fn new(value: String) -> Result<Self> {
        match BsonObjectId::parse_str(&value) {
            Ok(value) => Ok(Self { original: value }),
            Err(_) => Err(Error::from_reason("string doesn't represent valid ObjectId")),
        }
    }

    #[napi]
    pub fn to_string(&self) -> String {
        self.original.to_hex()
    }
}