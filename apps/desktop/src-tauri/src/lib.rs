use rusqlite::Connection;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

struct DbConnection(Mutex<Connection>);

#[derive(Serialize)]
struct AppInfo {
    version: String,
    db_status: String,
    db_path: String,
}

#[tauri::command]
fn get_app_info(db: State<DbConnection>) -> Result<AppInfo, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let db_status = conn
        .query_row("SELECT sqlite_version()", [], |row| row.get::<_, String>(0))
        .map(|v| format!("connected (SQLite {})", v))
        .unwrap_or_else(|_| "disconnected".to_string());

    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        db_status,
        db_path: "restrosync.db".to_string(),
    })
}

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )?;

    // Check if initial migration has been applied
    let applied: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM schema_version WHERE version = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !applied {
        conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (1)",
            [],
        )?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize SQLite database
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("restrosync.db");

            let conn =
                Connection::open(&db_path).expect("failed to open SQLite database");
            init_db(&conn).expect("failed to initialize database schema");

            app.manage(DbConnection(Mutex::new(conn)));

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
