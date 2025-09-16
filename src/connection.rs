// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//
// use crate::configure::{make_client_endpoint, make_server_endpoint};
//
// async fn connection() {
//     let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
//     let (endpoint, certificates) = make_server_endpoint(server_addr).unwrap();
//     let ep = endpoint.clone();
//
//     tokio::spawn(async move {
//         // incoming connections are now accepted, hanshake is initiated
//         // returns none if the endpoint is closed
//         if let Some(incoming_connection) = ep.accept().await {
//             match incoming_connection.await {
//                 Ok(connection) => log::info!(
//                     "Connection established to remote server {}",
//                     connection.remote_address()
//                 ),
//                 Err(e) => log::error!(
//                     "Error occured while trying to establish a connection to the remote server {}",
//                     e
//                 ),
//             }
//         }
//     });
//
//     let client_endpoint =
//         make_client_endpoint(&[&certificates], "0.0.0.0:0".parse().unwrap()).unwrap();
//     // iniitates handshake to server and awaits it and returns a connection
//     let connection = client_endpoint
//         .connect(server_addr, "localhost")
//         .unwrap()
//         .await
//         .unwrap();
//     log::info!(
//         "Client Connection established {}",
//         connection.remote_address()
//     );
//
//     // for graceful cleaning
//     endpoint.wait_idle().await;
// }
