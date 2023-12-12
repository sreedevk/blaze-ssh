use crate::instance_details::{InstanceDetails, InstanceSet};
use anyhow::Result;

const CACHE_FILE: &str = "/tmp/aws_instances_cache";

pub struct InstanceCache {
    pub instances: InstanceSet,
}

impl InstanceCache {
    pub fn new(instances: &InstanceSet) -> Self {
        InstanceCache {
            instances: instances.clone(),
        }
    }

    pub fn store(&self) -> Result<()> {
        let json = serde_json::to_string(&self.instances)?;
        std::fs::write(CACHE_FILE, json)?;

        Ok(())
    }

    pub fn fetch() -> Result<InstanceSet> {
        serde_json::from_str::<InstanceSet>(&std::fs::read_to_string(CACHE_FILE)?)
            .map_err(|e| e.into())
    }
}
