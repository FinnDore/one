// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use ::tokio::{net::TcpStream, sync::Mutex};
use axum::extract::State;
use axum::{
    body::{self, Bytes},
    http::{self, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use mqrstt::{
    new_smol, new_tokio,
    packets::{self, Packet},
    smol::{self, NetworkStatus},
    tokio, AsyncEventHandler, ConnectOptions, MqttClient,
};

use async_trait::async_trait;

pub struct Mttq {
    pub client: MqttClient,
}

#[async_trait]
impl AsyncEventHandler for Mttq {
    // Handlers only get INCOMING packets. This can change later.
    async fn handle(&mut self, packet: Packet) -> () {
        println!("got packet: {:?}", packet);
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct ClientThing {
    pub mttq_handler: Mttq,
}

type SharedState = State<Arc<Mutex<ClientThing>>>;

async fn on_message(State(state): SharedState, Json(payload): Json<serde_json::Value>) {
    // println!("{:?}", &payload);
    let client = &state.lock().await.mttq_handler.client;

    client
        .publish("gamestate/csgo", packets::QoS::AtLeastOnce, false, "hello")
        .await
        .unwrap();
    println!("published");
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async move {
                let options = ConnectOptions::new("hub".to_string());
                let (mut network, client) = new_tokio::<TcpStream>(options);
                let stream = ::tokio::net::TcpStream::connect(("0.0.0.0", 1883))
                    .await
                    .unwrap();

                let mut m = Mttq { client };

                println!("connecting ");
                network.connect(stream, &mut m).await.unwrap();
                println!("connected");

                let state = Arc::new(Mutex::new(ClientThing { mttq_handler: m }));
                let router = Router::new()
                    .route("/gamestate/csgo", post(on_message))
                    .with_state(state);
                axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
                    .serve(router.into_make_service())
                    .await
                    .expect("error while running axum server")
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
