use anyhow::Result;
use aws_sdk_ec2::types::Instance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceSet {
    pub instances: Vec<InstanceDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceDetails {
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub instance_id: Option<String>,
    pub instance_name: Option<String>,
}

impl InstanceDetails {
    pub fn from_instance(instance: &Instance) -> Result<Self> {
        let instance_clone = instance.clone();
        Ok(InstanceDetails {
            public_ip: instance_clone.public_ip_address,
            private_ip: instance_clone.private_ip_address,
            instance_id: instance_clone.instance_id,
            instance_name: Self::extract_instance_name(&instance.clone()),
        })
    }

    fn extract_instance_name(instance: &Instance) -> Option<String> {
        instance
            .tags()
            .iter()
            .find(|tag| tag.key == Some("Name".to_string()))
            .cloned()
            .and_then(|tag| tag.value)
    }
}
