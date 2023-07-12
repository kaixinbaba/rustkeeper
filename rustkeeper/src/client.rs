use crate::connection::ConnectionInfo;


/// The client type.
#[derive(Debug, Clone)]
pub struct Client {
    connection_info: ConnectionInfo,
}