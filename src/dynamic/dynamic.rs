use std::{collections::BTreeMap, sync::Arc};
use napi::{Result, Error};
use teo::prelude::app::data::AppData;

use super::{fetch::FetchDynamicClasses, query::QueryDynamicClasses};

#[derive(Clone)]
pub struct DynamicClasses {
    inner: Arc<Inner>,
}

struct Inner {
    ctxs: BTreeMap<String, napi::Ref<()>>,
    classes: BTreeMap<String, napi::Ref<()>>,
    objects: BTreeMap<String, napi::Ref<()>>,
}

impl DynamicClasses {

    pub fn retrieve(app_data: &AppData) -> Result<Self> {
        let reference = app_data.dynamic_classes()?;
        let dynamic_classes: Option<&Self> = reference.downcast_ref();
        match dynamic_classes {
            Some(dynamic_classes) => Ok(dynamic_classes.clone()),
            None => Err(Error::from_reason("The dynamic classes attached on the app data is of wrong type")),
        }
    }

    pub fn attach(&self, app_data: AppData) -> Result<()> {
        Ok(app_data.set_dynamic_classes(Arc::new(self.clone()))?)
    }

    pub fn new(
        ctxs: BTreeMap<String, napi::Ref<()>>, 
        classes: BTreeMap<String, napi::Ref<()>>, 
        objects: BTreeMap<String, napi::Ref<()>>
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                ctxs,
                classes,
                objects,    
            }),
        }
    }
}

impl FetchDynamicClasses for DynamicClasses {
    fn ctxs(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.inner.ctxs
    }

    fn classes(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.inner.classes
    }

    fn objects(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.inner.objects
    }
}

impl QueryDynamicClasses for DynamicClasses { }