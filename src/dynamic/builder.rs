use std::collections::BTreeMap;
use super::{create::CreateDynamicClasses, dynamic::DynamicClasses, fetch::FetchDynamicClasses, query::QueryDynamicClasses};

pub struct DynamicClassesBuilder {
    ctxs: BTreeMap<String, napi::Ref<()>>,
    classes: BTreeMap<String, napi::Ref<()>>,
    objects: BTreeMap<String, napi::Ref<()>>,
}

impl DynamicClassesBuilder {

    pub fn new() -> Self {
        Self {
            ctxs: BTreeMap::new(),
            classes: BTreeMap::new(),
            objects: BTreeMap::new(),
        }
    }

    pub fn build(self) -> DynamicClasses {
        DynamicClasses::new(self.ctxs, self.classes, self.objects)
    }
}

impl FetchDynamicClasses for DynamicClassesBuilder {
    
    fn ctxs(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.ctxs
    }

    fn classes(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.classes
    }

    fn objects(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.objects
    }
}

impl CreateDynamicClasses for DynamicClassesBuilder {

    fn ctxs_mut(&mut self) -> &mut BTreeMap<String, napi::Ref<()>> {
        &mut self.ctxs
    }

    fn classes_mut(&mut self) -> &mut BTreeMap<String, napi::Ref<()>> {
        &mut self.classes
    }

    fn objects_mut(&mut self) -> &mut BTreeMap<String, napi::Ref<()>> {
        &mut self.objects
    }
}

impl QueryDynamicClasses for DynamicClassesBuilder { }