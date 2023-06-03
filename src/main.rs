#![allow(unused_variables)]
use colink::{CoLink, Participant, ProtocolEntry};

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct StorageMacroParam {
    key_name: String,
    payload: Vec<u8>,
}

struct Init;
#[colink::async_trait]
impl ProtocolEntry for Init {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        println!("initiator");
        Ok(())
    }
}

struct CreateEntry;
#[colink::async_trait]
impl ProtocolEntry for CreateEntry {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let params = serde_json::from_slice::<StorageMacroParam>(&*param)?;
        let result = cl.create_entry(&params.key_name, &params.payload).await?;
        cl.create_entry(format!("sm-fallback-create-result-{}", &params.key_name).as_str(), &serde_json::to_vec(&result)?).await?;
        Ok(())
    }
}

struct UpdateEntry;
#[colink::async_trait]
impl ProtocolEntry for UpdateEntry {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let params = serde_json::from_slice::<StorageMacroParam>(&*param)?;
        let result = cl.update_entry(&params.key_name, &params.payload).await?;
        cl.create_entry(format!("sm-fallback-update-result-{}", &params.key_name).as_str(), &serde_json::to_vec(&result)?).await?;
        Ok(())
    }
}

struct DeleteEntry;
#[colink::async_trait]
impl ProtocolEntry for DeleteEntry {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let params = serde_json::from_slice::<StorageMacroParam>(&*param)?;
        let result = cl.delete_entry(&params.key_name).await?;
        cl.create_entry(format!("sm-fallback-delete-result-{}", &params.key_name).as_str(), &serde_json::to_vec(&result)?).await?;
        Ok(())
    }
}

struct ReadEntry;
#[colink::async_trait]
impl ProtocolEntry for ReadEntry {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let params = serde_json::from_slice::<StorageMacroParam>(&*param)?;
        let result = cl.read_entry(&params.key_name).await?;
        cl.create_entry(format!("sm-fallback-read-result-{}", &params.key_name).as_str(), &serde_json::to_vec(&result)?).await?;
        Ok(())
    }
}


colink::protocol_start!(
    ("storage_macro_rust_fallback:init", Init),
    ("storage_macro_rust_fallback:create", CreateEntry),
    ("storage_macro_rust_fallback:update", UpdateEntry), 
    ("storage_macro_rust_fallback:delete", DeleteEntry), 
    ("storage_macro_rust_fallback:read", ReadEntry)
);