//! Contains functions for letting the user configure the application setup (i.e. local vs server vs
//! client) via a CLI interface.

use std::{
    fmt::Display,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

#[derive(Debug, Copy, Clone)]
pub enum NetworkMode {
    Local,
    Client,
    Server,
}

impl Display for NetworkMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NetworkMode::Local => "Local",
            NetworkMode::Client => "Client",
            NetworkMode::Server => "Server",
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NetworkConfig {
    Local,
    /// Will connect to a TCP server on the specified address.
    Client(SocketAddr),
    /// Will setup TCP server on the specified port.
    Server(u16),
}

pub fn prompt_network_config() -> Result<NetworkConfig, inquire::InquireError> {
    let mode = inquire::Select::new(
        "Game setup:",
        vec![NetworkMode::Local, NetworkMode::Client, NetworkMode::Server],
    )
    .prompt()?;

    match mode {
        NetworkMode::Local => Ok(NetworkConfig::Local),
        NetworkMode::Client => {
            let addr = inquire::CustomType::<SocketAddr>::new("IP address to connect to:")
                .with_placeholder("0.0.0.0:3000")
                .with_parser(&|string| string.parse().map_err(|_| ()))
                .with_error_message("Please type a valid IP and port.")
                .with_default(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000).into())
                .prompt()?;

            Ok(NetworkConfig::Client(addr))
        }
        NetworkMode::Server => {
            let port = inquire::CustomType::<u16>::new("Port to listen on:")
                .with_placeholder("3000")
                .with_parser(&|string| string.parse().map_err(|_| ()))
                .with_error_message("Please type an integer 0-65535 (inclusive).")
                .with_default(3000)
                .prompt()?;

            Ok(NetworkConfig::Server(port))
        }
    }
}
