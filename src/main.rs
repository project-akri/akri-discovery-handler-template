mod discovery_handler;

use akri_discovery_utils::discovery::discovery_handler::run_discovery_handler;
use discovery_handler::DiscoveryHandlerImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Specify the name of this discovery handler. A discovery handler is usually, but not necessarily, identified by
    // the protocol it uses.
    let name = "todo";
    // Specify whether the devices discovered by this discovery handler are locally attached (or embedded) to nodes or are
    // network based and usable by multiple nodes.
    let is_local = false;
    // A Discovery Handler must handle the Agent dropping a connection due to a Configuration that utilizes this
    // Discovery Handler being deleted or the Agent erroring. It is impossible to determine the cause of the
    // disconnection, so in case the Agent did error out, the Discovery Handler should try to re-register.
    let (register_sender, register_receiver) = tokio::sync::mpsc::channel(2);
    // Create a discovery handler
    let discovery_handler = DiscoveryHandlerImpl::new(register_sender);
    run_discovery_handler(discovery_handler, register_receiver, name, is_local).await?;
    Ok(())
}
