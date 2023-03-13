#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use connection::ConnectionState;
use database::DatabaseState;

mod config;
mod schema;
mod database;
mod connection;

fn main() {
    tauri::Builder::default()
		.manage(DatabaseState(Default::default()))
		.manage(ConnectionState(Default::default()))
        .invoke_handler(tauri::generate_handler![
			config::load_config,
			config::save_config,
			schema::extract_scope_definition,
			schema::extract_table_definition,
			schema::extract_field_definition,
			schema::extract_index_definition,
			schema::extract_event_definition,
			database::start_database,
			database::stop_database,
			connection::open_connection,
			connection::close_connection,
			connection::execute_query,
		])
        .run(tauri::generate_context!())
        .expect("tauri should start successfully");
}
