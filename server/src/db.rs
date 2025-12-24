use crate::auth;
use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // devices table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                device_type TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;

        // registration_tokens table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS registration_tokens (
                token TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL,
                used BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        // device_status table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS device_status (
                device_id TEXT PRIMARY KEY,
                status BOOLEAN NOT NULL DEFAULT 0,
                updated_at INTEGER,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        // device_credentials table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS device_credentials (
                device_id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        // iPhone-specific data
        conn.execute(
            "CREATE TABLE IF NOT EXISTS iphone_data (
                device_id TEXT PRIMARY KEY,
                battery_level INTEGER,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        // per-device settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS device_settings (
                device_id TEXT PRIMARY KEY,
                visible BOOLEAN DEFAULT 0,
                public_battery BOOLEAN DEFAULT 0,
                public_activity BOOLEAN DEFAULT 0,
                public_last_seen BOOLEAN DEFAULT 1,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn create_device(&self, name: &str, device_type: &str) -> Result<String> {
        let device_id = uuid::Uuid::new_v4().to_string();

        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn.execute(
            "INSERT INTO devices (id, name, device_type, created_at)",
            rusqlite::params![device_id.clone(), name, device_type, created_at,],
        )?;

        Ok(device_id)
    }

    pub fn create_registration_token(&self, device_id: &str) -> Result<String> {
        // declare this somewhere else later, maybe env or a config
        // 6 hour expiry placeholder
        let expire_len_secs = 60 * 60 * 6;
        let token = auth::generate_token();

        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = created_at + expire_len_secs as i64;

        self.conn.execute(
            "INSERT INTO registration_tokens (token, device_id, created_at, expires_at)
             VALUES (?, ?, ?, ?)",
            rusqlite::params![token, device_id, created_at, expires_at],
        )?;

        Ok(token)
    }

    pub fn create_device_settings(&self, device_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO device_settings (device_id) VALUES (?)",
            rusqlite::params![device_id],
        )?;

        Ok(())
    }

    pub fn create_device_credentials(
        &self,
        device_id: &str,
        username: &str,
        password: &auth::Password,
    ) -> Result<()> {
        self.conn.execute(
            "
            INSERT INTO device_credentials (device_id, username, password_hash)",
            rusqlite::params![device_id, username, password.hash],
        )?;

        Ok(())
    }

    pub fn create_device_status(&self, device_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO device_status (device_id) VALUES (?)",
            rusqlite::params![device_id],
        )?;

        Ok(())
    }

    pub fn create_iphone_data(&self, device_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO iphone_data (device_id) VALUES (?)",
            rusqlite::params![device_id],
        )?;

        Ok(())
    }

    pub fn get_device_id_from_token(&self, token: &str) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn.query_row(
            "SELECT device_id FROM registration_tokens WHERE token = ? AND used = 0 AND expires_at > ?",
            rusqlite::params![token, now],
            |row| row.get(0),
        )
    }

    pub fn mark_token_as_used(&self, token: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE registration_tokens SET used = 1 WHERE token = ?",
            rusqlite::params![token],
        )?;

        Ok(())
    }
}
