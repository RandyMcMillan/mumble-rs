use anyhow::{anyhow, Result};
use configparser::ini::Ini;

// Enum to represent different database connection parameters
#[derive(Debug)]
pub enum DbConnectionParameter {
    SQLite {
        path: String,
        use_wal: bool,
    },
    MySQL {
        db_name: String,
        username: String,
        password: String,
        host: String,
        port: u16,
        prefix: String,
        opts: String,
    },
    PostgreSQL {
        db_name: String,
        username: String,
        password: String,
        host: String,
        port: u16,
        prefix: String,
        opts: String,
    },
}

// Equivalent to C++ MetaParams
#[derive(Debug)]
pub struct MetaParams {
    pub base_path: String,
    pub bind_addresses: Vec<String>,
    pub port: u16,
    pub timeout: i32,
    pub max_bandwidth: i32,
    pub max_users: u32,
    pub max_users_per_channel: u32,
    pub max_listeners_per_channel: i32,
    pub max_listener_proxies_per_user: i32,
    pub default_channel: u32,
    pub remember_channel: bool,
    pub remember_channel_duration: i32,
    pub max_text_message_length: i32,
    pub max_image_message_length: i32,
    pub opus_threshold: i32,
    pub channel_nesting_limit: i32,
    pub channel_count_limit: i32,
    pub legacy_password_hash: bool,
    pub kdf_iterations: i32,
    pub allow_html: bool,
    pub password: String,
    pub welcome_text: String,
    pub welcome_text_file: String,
    pub cert_required: bool,
    pub force_external_auth: bool,
    pub ban_tries: i32,
    pub ban_timeframe: i32,
    pub ban_time: i32,
    pub ban_successful: bool,
    pub database: String,
    pub sqlite_wal: i32,
    pub db_driver: String,
    pub db_username: String,
    pub db_password: String,
    pub db_hostname: String,
    pub db_prefix: String,
    pub db_opts: String,
    pub db_port: i32,
    pub log_days: i32,
    pub obfuscate: i32,
    pub send_version: bool,
    pub allow_ping: bool,
    pub logfile: String,
    pub pid_file: String,
    pub ice_endpoint: String,
    pub ice_secret_read: String,
    pub ice_secret_write: String,
    pub reg_name: String,
    pub reg_password: String,
    pub reg_host: String,
    pub reg_location: String,
    pub reg_web_url: String,
    pub bonjour: bool,
    pub message_limit: u32,
    pub message_burst: u32,
    pub plugin_message_limit: u32,
    pub plugin_message_burst: u32,
    pub broadcast_listener_volume_adjustments: bool,
    pub ciphers: String,
    pub suggest_positional: Option<bool>,
    pub suggest_push_to_talk: Option<bool>,
    pub log_group_changes: bool,
    pub log_acl_changes: bool,
    pub allow_recording: bool,
    pub rolling_stats_window: u32,
    pub abs_settings_file_path: String,
    pub ssl_cert: String,
    pub ssl_key: String,
}

impl Default for MetaParams {
    fn default() -> Self {
        Self {
            base_path: "".to_string(),
            bind_addresses: Vec::new(),
            port: 64738,
            timeout: 5000,
            max_bandwidth: 100000,
            max_users: 100,
            max_users_per_channel: 0,
            max_listeners_per_channel: 0,
            max_listener_proxies_per_user: 0,
            default_channel: 0,
            remember_channel: true,
            remember_channel_duration: 100,
            max_text_message_length: 5000,
            max_image_message_length: 131072,
            opus_threshold: 100,
            channel_nesting_limit: 10,
            channel_count_limit: 50,
            legacy_password_hash: false,
            kdf_iterations: 0,
            allow_html: true,
            password: "".to_string(),
            welcome_text: "<br />Welcome to this Mumble server!<br />".to_string(),
            welcome_text_file: "".to_string(),
            cert_required: false,
            force_external_auth: false,
            ban_tries: 10,
            ban_timeframe: 120,
            ban_time: 300,
            ban_successful: false,
            database: "murmur.sqlite".to_string(),
            sqlite_wal: 1,
            db_driver: "QSQLITE".to_string(),
            db_username: "".to_string(),
            db_password: "".to_string(),
            db_hostname: "".to_string(),
            db_prefix: "".to_string(),
            db_opts: "".to_string(),
            db_port: 0,
            log_days: 31,
            obfuscate: 0,
            send_version: true,
            allow_ping: true,
            logfile: "".to_string(),
            pid_file: "".to_string(),
            ice_endpoint: "tcp -h 127.0.0.1 -p 6502".to_string(),
            ice_secret_read: "".to_string(),
            ice_secret_write: "".to_string(),
            reg_name: "".to_string(),
            reg_password: "".to_string(),
            reg_host: "".to_string(),
            reg_location: "".to_string(),
            reg_web_url: "".to_string(),
            bonjour: false,
            message_limit: 5,
            message_burst: 10,
            plugin_message_limit: 5,
            plugin_message_burst: 10,
            broadcast_listener_volume_adjustments: false,
            ciphers: "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:AES256-GCM-SHA384:AES128-GCM-SHA256:AES256-SHA256:AES128-SHA256".to_string(),
            suggest_positional: None,
            suggest_push_to_talk: None,
            log_group_changes: false,
            log_acl_changes: false,
            allow_recording: true,
            rolling_stats_window: 60,
            abs_settings_file_path: "".to_string(),
            ssl_cert: "".to_string(),
            ssl_key: "".to_string(),
        }
    }
}

pub fn read_config(file_path: &str) -> Result<MetaParams> {
    let mut config_parser = Ini::new();
    config_parser
        .read(file_path.to_string())
        .map_err(|e| anyhow!("Failed to read INI file: {}", e))?;

    let mut params = MetaParams::default();

    // Helper functions for parsing
    fn get_string(parser: &Ini, key: &str, default: String) -> String {
        parser.get("General", key).unwrap_or(default)
    }

    fn get_i32(parser: &Ini, key: &str, default: i32) -> i32 {
        parser
            .getint("General", key)
            .ok()
            .flatten()
            .map(|v| v as i32)
            .unwrap_or(default)
    }

    fn get_u32(parser: &Ini, key: &str, default: u32) -> u32 {
        parser
            .getint("General", key)
            .ok()
            .flatten()
            .map(|v| v as u32)
            .unwrap_or(default)
    }

    fn get_u16(parser: &Ini, key: &str, default: u16) -> u16 {
        parser
            .getint("General", key)
            .ok()
            .flatten()
            .map(|v| v as u16)
            .unwrap_or(default)
    }

    fn get_bool(parser: &Ini, key: &str, default: bool) -> bool {
        parser
            .getbool("General", key)
            .ok()
            .flatten()
            .unwrap_or(default)
    }

    params.port = get_u16(&config_parser, "port", params.port);
    params.max_users = get_u32(&config_parser, "users", params.max_users);
    params.welcome_text = get_string(&config_parser, "welcometext", params.welcome_text);
    params.ssl_cert = get_string(&config_parser, "sslCert", params.ssl_cert);
    params.ssl_key = get_string(&config_parser, "sslKey", params.ssl_key);
    params.database = get_string(&config_parser, "database", params.database);
    params.db_driver = get_string(&config_parser, "dbDriver", params.db_driver);
    params.db_username = get_string(&config_parser, "dbUsername", params.db_username);
    params.db_password = get_string(&config_parser, "dbPassword", params.db_password);
    params.db_hostname = get_string(&config_parser, "dbHost", params.db_hostname);
    params.db_port = get_i32(&config_parser, "dbPort", params.db_port);
    params.db_prefix = get_string(&config_parser, "dbPrefix", params.db_prefix);
    params.db_opts = get_string(&config_parser, "dbOpts", params.db_opts);
    params.sqlite_wal = get_i32(&config_parser, "sqlite_wal", params.sqlite_wal);
    params.timeout = get_i32(&config_parser, "timeout", params.timeout);
    params.max_bandwidth = get_i32(&config_parser, "bandwidth", params.max_bandwidth);
    params.max_users_per_channel = get_u32(
        &config_parser,
        "usersperchannel",
        params.max_users_per_channel,
    );
    params.password = get_string(&config_parser, "password", params.password);
    params.allow_html = get_bool(&config_parser, "allowhtml", params.allow_html);
    params.remember_channel = get_bool(&config_parser, "rememberchannel", params.remember_channel);
    params.log_days = get_i32(&config_parser, "logdays", params.log_days);
    params.logfile = get_string(&config_parser, "logfile", params.logfile);
    params.pid_file = get_string(&config_parser, "pidfile", params.pid_file);
    params.bonjour = get_bool(&config_parser, "bonjour", params.bonjour);
    params.reg_name = get_string(&config_parser, "registername", params.reg_name);
    params.reg_password = get_string(&config_parser, "registerpassword", params.reg_password);
    params.reg_host = get_string(&config_parser, "registerhost", params.reg_host);
    params.reg_location = get_string(&config_parser, "registerlocation", params.reg_location);
    params.reg_web_url = get_string(&config_parser, "registerurl", params.reg_web_url);
    params.ciphers = get_string(&config_parser, "ciphers", params.ciphers);
    params.send_version = get_bool(&config_parser, "sendversion", params.send_version);

    Ok(params)
}
