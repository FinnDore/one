// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tokio::io::AsyncWriteExt;

use tokio::net::{TcpSocket, TcpStream};
use tokio::sync::{Mutex, MutexGuard};

pub struct MutexState(Mutex<InnerState>);
struct InnerState {
    pub b: u8,
    pub tcp_stream: Option<TcpStream>,
    // pub tpc_socket: ,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn set_color(color: &str, state: tauri::State<'_, MutexState>) -> Result<(), ()> {
    println!("color: {}", color);
    if let Some(inner) = state.0.lock().await.tcp_stream.as_mut() {
        match inner.write_all(format!("{}00", color).as_bytes()).await {
            Ok(_) => println!("ok"),
            Err(e) => println!("error: {:?}", e),
        }
    }
    // state
    //     .tcp_stream
    //     .as_mut()
    //     .unwrap()
    //     .write_all(b"#00110011")
    //     .await
    //     .unwrap();
    // state.b += 1;
    // println!("b: {} {}", state.b, color);
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("10.0.0.12:1234").await;

    let tcp_stream = match stream {
        Ok(stream) => Some(stream),
        Err(e) => {
            println!("failed to connect to server: {:?}", e);
            None
        }
    };
    // Write some data.

    // let socket = Some(socket);

    // let stream = socket.connect(addr).await?;
    tauri::Builder::default()
        .manage(MutexState(Mutex::new(InnerState { b: 0, tcp_stream })))
        .invoke_handler(tauri::generate_handler![set_color])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
