/// Server checks that the client is still awake once in a while.
pub mod clientbound_keep_alive;
/// Which data packs exists on the server.
pub mod clientbound_known_packs;
/// Clientbound plugin message.
pub mod clientbound_plugin_message;
/// Which feature flags to use.
pub mod feature_flags;
/// Signals that configuration is complete.
pub mod finish_configuration;
/// Registry data sent from server to client.
pub mod registry_data;
