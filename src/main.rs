use std::net::IpAddr;
use std::env;

use tokio::net::UdpSocket;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use warp::Filter;
use serde::{Deserialize, Serialize};
use dotenv;

#[derive(Deserialize)]
struct InputData {
    client_ip: String,
    client_port: u16,
    source_port: u16,
    b64_payload: String,
}

#[derive(Serialize)]
struct Response {
    success: bool,
}

#[tokio::main]
async fn main() {
    // Load configuration
    dotenv::dotenv().ok();

    let CMD_PORT = env::var("CMD_PORT").unwrap_or(
        "8080".to_string()
    ).parse::<u16>().unwrap();

    let CMD_HOST: IpAddr = env::var("CMD_HOST").unwrap_or(
        "0.0.0.0".to_string()
    ).parse::<IpAddr>().unwrap();

    // POST endpoint that receives the JSON payload
    let route = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(move |data: InputData| {
            async move {
                // Parse target socket address
                let socket_addr = format!("{}:{}", data.client_ip, data.client_port);
                println!("Sending UDP packet from source port {} to {}", data.source_port, socket_addr);

                let b64engine = STANDARD;

                // Decode base64 payload
                let Ok(payload) = b64engine.decode(&data.b64_payload) else {
                    // Failed to decode base64 payload
                    let response = Response { success: false };
                    return Ok::<_, warp::Rejection>(warp::reply::json(&response));
                };

                // Bind to UDP socket
                let Ok(source_socket) = UdpSocket::bind(("0.0.0.0", data.source_port)).await else {
                    // Failed to bind to socket
                    let response = Response { success: false };
                    return Ok::<_, warp::Rejection>(warp::reply::json(&response));
                };

                // Try to send UDP packet
                let Ok(_) = source_socket.send_to(&payload, socket_addr).await else {
                    // Failed to send UDP packet
                    let response = Response { success: false };
                    return Ok::<_, warp::Rejection>(warp::reply::json(&response));
                };
                let response = Response{ success: true };
                Ok::<_, warp::Rejection>(warp::reply::json(&response))
            }
        });

    // Start the Warp server
    println!("Server running at http://{}:{}", CMD_HOST, CMD_PORT);
    warp::serve(route).run((CMD_HOST, CMD_PORT)).await;
}
