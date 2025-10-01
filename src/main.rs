use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

use anyhow::anyhow;
use ggez::{conf, event};

use rsoderh_gui::{
    MainState,
    chess_game::GameUi,
    network::{ChesstpMessageStream, ConnectionType, GameConnection, setup},
};

pub fn main() -> Result<(), anyhow::Error> {
    let config = setup::prompt_network_config().unwrap();
    println!("Got config {:?}", config);

    let connection = match config {
        setup::NetworkConfig::Local => GameConnection::Local,
        setup::NetworkConfig::Client(socket_addr) => {
            println!("Connecting to {}...", socket_addr);
            let stream = match TcpStream::connect(socket_addr) {
                Ok(stream) => stream,
                Err(error) => {
                    return Err(anyhow!("Could not connect to {}: {}", socket_addr, error));
                }
            };
            println!("Connected, starting game");
            GameConnection::Remote(
                ConnectionType::Client,
                socket_addr,
                ChesstpMessageStream::new(stream)?,
            )
        }
        setup::NetworkConfig::Server(port) => {
            let addr_v4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
            let listener_v4 = TcpListener::bind(addr_v4)?;
            listener_v4.set_nonblocking(true)?;
            println!("Bound ipv4");

            // let addr_v6 = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), port, 0, 0);
            // let listener_v6 = TcpListener::bind(addr_v6)?;
            // listener_v6.set_nonblocking(true)?;
            // println!("Bound ipv6");

            println!("Listening on {}", addr_v4);
            // println!("Listening on {}", addr_v6);
            println!("Waiting for connection...");

            let (stream, socket_addr) = loop {
                match listener_v4.accept() {
                    Ok((stream, socket_addr)) => break (stream, socket_addr),
                    Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {}
                    Err(error) => return Err(anyhow!(error)),
                }
                // match listener_v6.accept() {
                //     Ok(pair) => break pair,
                //     Err(ref error)  if error.kind() == io::ErrorKind::WouldBlock => {},
                //     Err(error) => return Err(anyhow!(error)),
                // }
            };

            stream.set_nonblocking(true)?;

            println!("Connected to {}, starting game", socket_addr);

            GameConnection::Remote(
                ConnectionType::Server,
                socket_addr,
                ChesstpMessageStream::new(stream)?,
            )
        }
    };

    // TcpListener::bind(addr);

    let min_size = GameUi::size();
    let cb = ggez::ContextBuilder::new("rsoderh_chess_gui", "ggez")
        .window_mode(conf::WindowMode {
            width: min_size.x,
            height: min_size.y,
            resizable: true,
            // resize_on_scale_factor_change: true,
            ..Default::default()
        })
        .window_setup(conf::WindowSetup {
            title: "Rsoderh Chess".to_owned(),
            ..conf::WindowSetup::default()
        });
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx, connection)?;
    event::run(ctx, event_loop, state)
}
