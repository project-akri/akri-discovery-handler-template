mod discovery_handler;

use akri_discovery_utils::{
    discovery::{DISCOVERY_HANDLER_PATH, server::run_discovery_server, v0::RegisterRequest},
    registration_client::register,
};
use discovery_handler::DiscoveryHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Specify the name of this discovery handler. A discovery handler is usually, but not necessarily, identified by
    // the protocol it uses.
    let name = "name";
    // Specify whether the devices discovered by this discovery handler are locally attached or embedded to nodes or are
    // network based and usable by multiple nodes.
    let is_local = false;
    // Specify the socket that the discovery handler will advertize it's service over Instead of using uds, a network
    // based connection could also be used.
    let endpoint = format!("{}/{}.sock", DISCOVERY_HANDLER_PATH, name);
    // Optionally create a channel for shutting down the service. This could be done when a connection with the Agent is
    // lost. This may signal a need to stop the service and try to re-register with the Agent.
    let (shutdown_sender, shutdown_receiver) = tokio::sync::mpsc::channel(2);
    // Clone the endpoint for the thread that serves the discovery handler.
    let endpoint_clone = endpoint.clone();
    // Run the discovery server.
    let handle = tokio::spawn(async move {
        run_discovery_server(
            DiscoveryHandler::new(Some(shutdown_sender)),
            &endpoint_clone,
            shutdown_receiver,
        )
        .await
        .unwrap();
    });
    // Create a register request, which specifies the name of the discovery handler, the endpoint for calling it's
    // discover service, and whether it discovers local or network based devices.
    let register_request = RegisterRequest {
        protocol: name.to_string(),
        endpoint: endpoint.to_string(),
        is_local,
    }; 
    register(register_request).await?;
    handle.await?;
    Ok(())
}
