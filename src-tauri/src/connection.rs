use std::{sync::Mutex, net::TcpStream};
use tungstenite::{connect, WebSocket, stream::MaybeTlsStream};

pub struct ConnectionState(pub Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>);

#[tauri::command]
pub fn open_connection(
	state: tauri::State<ConnectionState>,
	endpoint: &str
) -> Result<(), String> {
	let mut process = state.0.lock().unwrap();
	let (socket, _) = connect(endpoint).map_err(|_| "Failed to open socket connection")?;

	*process = Some(socket);

	let connection = (&mut *process).as_mut().unwrap();

	loop {
        let msg = connection.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
 
	return Ok(());
}

#[tauri::command]
pub fn close_connection() -> Result<(), String> {
	return Ok(());
}

#[tauri::command]
pub fn execute_query(_query: &str) -> Result<(), String> {
	return Ok(());
}