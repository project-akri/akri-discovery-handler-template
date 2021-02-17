use akri_discovery_utils::discovery::v0::{
    discovery_server::Discovery, Device, DiscoverRequest, DiscoverResponse,
};
use async_trait::async_trait;
use tokio::sync::mpsc;
use tonic::{Response, Status};

pub type DiscoverStream = mpsc::Receiver<Result<DiscoverResponse, Status>>;

pub struct DiscoveryHandler {
    shutdown_sender: Option<tokio::sync::mpsc::Sender<()>>,
}

impl DiscoveryHandler {
    pub fn new(shutdown_sender: Option<tokio::sync::mpsc::Sender<()>>) -> Self {
        DiscoveryHandler { shutdown_sender }
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