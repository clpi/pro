use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use uuid::Uuid;

/// The Config dealing with instance(s)
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct CfgInst {
    #[serde(default = "Uuid::default")]
    pub user_id: Uuid,
    pub is_running: bool,
    pub is_authorized_to_run: bool,
    #[serde(default = "ServerData::default")]
    pub server_data: ServerData,
    #[serde(default = "Option::default")]
    pub hosting_group: Option<super::group::GroupInfo>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct ServerData {
    #[serde(default = "Uuid::default")]
    pub server_id: Uuid,
    #[serde(default = "String::default")]
    pub name: String,
    pub addr: SocketAddrV4,
}
impl Default for ServerData {
    fn default() -> Self {
        Self {
            server_id: Uuid::default(),
            name: String::default(),
            addr: SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0),
        }
    }
}
impl ServerData {
    pub fn set_socket_addr(&mut self, local: Ipv4Addr, port: u16) {
        self.addr = SocketAddrV4::new(local, port);
    }
    pub fn set_port(&mut self, port: u16) {
        let ip4 = Ipv4Addr::new(0, 0, 0, 0);
        self.addr = SocketAddrV4::new(ip4, port);
    }
    fn new(server_id: Uuid, name: String, addr: SocketAddrV4) -> Self {
        Self {
            server_id,
            name,
            addr,
        }
    }
}
