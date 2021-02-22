mod discovery_handler;

use akri_discovery_utils::{
    discovery::{DISCOVERY_HANDLER_PATH, server::run_discovery_server, v0::RegisterRequest},
    registration_client::{register, register_again},
};
use discovery_handler::DiscoveryHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Specify the name of this discovery handler. A discovery handler is usually, but not necessarily, identified by
    // the protocol it uses.
    let name = "protocol";
    // Specify whether the devices discovered by this discovery handler are locally attached (or embedded) to nodes or are
    // network based and usable by multiple nodes.
    let is_local = false;
    // Specify the socket that the discovery handler will advertize it's service over Instead of using uds, a network
    // based connection could also be used.
    let endpoint = format!("{}/{}.sock", DISCOVERY_HANDLER_PATH, name);
    // A Discovery Handler must handle the Agent dropping a connection due to a Configuration that utilizes this 
    // Discovery Handler being deleted or the Agent erroring. It is impossible to determine the cause of the 
    // disconnection, so in case the Agent did error out, the Discovery Handler should try to re-register.
    let (register_sender, register_receiver) = tokio::sync::mpsc::channel(2);
    // Clone the endpoint for the thread that serves the discovery handler.
    let endpoint_clone = endpoint.clone();
    // Run the discovery server.
    let discovery_handle = tokio::spawn(async move {
        run_discovery_server(
            DiscoveryHandler::new(Some(register_sender)),
            &endpoint_clone,
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
    register(&register_request).await?;
    let registration_handle = tokio::spawn(async move {
        register_again(register_receiver, &register_request).await;
    });
    tokio::try_join!(discovery_handle, registration_handle)?;
    Ok(())
}
