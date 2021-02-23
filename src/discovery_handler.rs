use akri_discovery_utils::discovery::{
    DiscoverStream,
    v0::{
    discovery_server::Discovery, Device, DiscoverRequest, DiscoverResponse,
}};
use async_trait::async_trait;
use tokio::sync::mpsc;
use tonic::{Response, Status};

/// DiscoveryHandler implements Akri's Discovery Service.
pub struct DiscoveryHandler {
    register_sender: tokio::sync::mpsc::Sender<()>,
}

impl DiscoveryHandler {
    pub fn new(register_sender: tokio::sync::mpsc::Sender<()>) -> Self {
        DiscoveryHandler { register_sender }
    }
}

#[async_trait]
impl Discovery for DiscoveryHandler {
    type DiscoverStream = DiscoverStream;
    async fn discover(
        &self,
        request: tonic::Request<DiscoverRequest>,
    ) -> Result<Response<Self::DiscoverStream>, Status> {
        unimplemented!("add your discovery logic here");
    }
}