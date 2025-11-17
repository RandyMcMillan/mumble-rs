use crate::config::{read_config, MetaParams};
use clap::Parser;
use log::info;

/// Mumble server (murmur)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, env = "MUMBLE_CONFIG", default_value = "murmur.ini")]
    config: String,

    #[arg(long, env = "MUMBLE_BASE_PATH")]
    base_path: Option<String>,
    #[arg(long, env = "MUMBLE_BIND_ADDRESSES")]
    bind_addresses: Vec<String>,
    #[arg(long, env = "MUMBLE_PORT")]
    port: Option<u16>,
    #[arg(long, env = "MUMBLE_TIMEOUT")]
    timeout: Option<i32>,
    #[arg(long, env = "MUMBLE_MAX_BANDWIDTH")]
    max_bandwidth: Option<i32>,
    #[arg(long, env = "MUMBLE_MAX_USERS")]
    max_users: Option<u32>,
    #[arg(long, env = "MUMBLE_MAX_USERS_PER_CHANNEL")]
    max_users_per_channel: Option<u32>,
    #[arg(long, env = "MUMBLE_MAX_LISTENERS_PER_CHANNEL")]
    max_listeners_per_channel: Option<i32>,
    #[arg(long, env = "MUMBLE_MAX_LISTENER_PROXIES_PER_USER")]
    max_listener_proxies_per_user: Option<i32>,
    #[arg(long, env = "MUMBLE_DEFAULT_CHANNEL")]
    default_channel: Option<u32>,
    #[arg(long, env = "MUMBLE_REMEMBER_CHANNEL")]
    remember_channel: Option<bool>,
    #[arg(long, env = "MUMBLE_REMEMBER_CHANNEL_DURATION")]
    remember_channel_duration: Option<i32>,
    #[arg(long, env = "MUMBLE_MAX_TEXT_MESSAGE_LENGTH")]
    max_text_message_length: Option<i32>,
    #[arg(long, env = "MUMBLE_MAX_IMAGE_MESSAGE_LENGTH")]
    max_image_message_length: Option<i32>,
    #[arg(long, env = "MUMBLE_OPUS_THRESHOLD")]
    opus_threshold: Option<i32>,
    #[arg(long, env = "MUMBLE_CHANNEL_NESTING_LIMIT")]
    channel_nesting_limit: Option<i32>,
    #[arg(long, env = "MUMBLE_CHANNEL_COUNT_LIMIT")]
    channel_count_limit: Option<i32>,
    #[arg(long, env = "MUMBLE_LEGACY_PASSWORD_HASH")]
    legacy_password_hash: Option<bool>,
    #[arg(long, env = "MUMBLE_KDF_ITERATIONS")]
    kdf_iterations: Option<i32>,
    #[arg(long, env = "MUMBLE_ALLOW_HTML")]
    allow_html: Option<bool>,
    #[arg(long, env = "MUMBLE_PASSWORD")]
    password: Option<String>,
    #[arg(long, env = "MUMBLE_WELCOME_TEXT")]
    welcome_text: Option<String>,
    #[arg(long, env = "MUMBLE_WELCOME_TEXT_FILE")]
    welcome_text_file: Option<String>,
    #[arg(long, env = "MUMBLE_CERT_REQUIRED")]
    cert_required: Option<bool>,
    #[arg(long, env = "MUMBLE_FORCE_EXTERNAL_AUTH")]
    force_external_auth: Option<bool>,
    #[arg(long, env = "MUMBLE_BAN_TRIES")]
    ban_tries: Option<i32>,
    #[arg(long, env = "MUMBLE_BAN_TIMEFRAME")]
    ban_timeframe: Option<i32>,
    #[arg(long, env = "MUMBLE_BAN_TIME")]
    ban_time: Option<i32>,
    #[arg(long, env = "MUMBLE_BAN_SUCCESSFUL")]
    ban_successful: Option<bool>,
    #[arg(long, env = "MUMBLE_DATABASE")]
    database: Option<String>,
    #[arg(long, env = "MUMBLE_SQLITE_WAL")]
    sqlite_wal: Option<i32>,
    #[arg(long, env = "MUMBLE_DB_DRIVER")]
    db_driver: Option<String>,
    #[arg(long, env = "MUMBLE_DB_USERNAME")]
    db_username: Option<String>,
    #[arg(long, env = "MUMBLE_DB_PASSWORD")]
    db_password: Option<String>,
    #[arg(long, env = "MUMBLE_DB_HOSTNAME")]
    db_hostname: Option<String>,
    #[arg(long, env = "MUMBLE_DB_PREFIX")]
    db_prefix: Option<String>,
    #[arg(long, env = "MUMBLE_DB_OPTS")]
    db_opts: Option<String>,
    #[arg(long, env = "MUMBLE_DB_PORT")]
    db_port: Option<i32>,
    #[arg(long, env = "MUMBLE_LOG_DAYS")]
    log_days: Option<i32>,
    #[arg(long, env = "MUMBLE_OBFUSCATE")]
    obfuscate: Option<i32>,
    #[arg(long, env = "MUMBLE_SEND_VERSION")]
    send_version: Option<bool>,
    #[arg(long, env = "MUMBLE_ALLOW_PING")]
    allow_ping: Option<bool>,
    #[arg(long, env = "MUMBLE_LOGFILE")]
    logfile: Option<String>,
    #[arg(long, env = "MUMBLE_PID_FILE")]
    pid_file: Option<String>,
    #[arg(long, env = "MUMBLE_ICE_ENDPOINT")]
    ice_endpoint: Option<String>,
    #[arg(long, env = "MUMBLE_ICE_SECRET_READ")]
    ice_secret_read: Option<String>,
    #[arg(long, env = "MUMBLE_ICE_SECRET_WRITE")]
    ice_secret_write: Option<String>,
    #[arg(long, env = "MUMBLE_REG_NAME")]
    reg_name: Option<String>,
    #[arg(long, env = "MUMBLE_REG_PASSWORD")]
    reg_password: Option<String>,
    #[arg(long, env = "MUMBLE_REG_HOST")]
    reg_host: Option<String>,
    #[arg(long, env = "MUMBLE_REG_LOCATION")]
    reg_location: Option<String>,
    #[arg(long, env = "MUMBLE_REG_WEB_URL")]
    reg_web_url: Option<String>,
    #[arg(long, env = "MUMBLE_BONJOUR")]
    bonjour: Option<bool>,
    #[arg(long, env = "MUMBLE_MESSAGE_LIMIT")]
    message_limit: Option<u32>,
    #[arg(long, env = "MUMBLE_MESSAGE_BURST")]
    message_burst: Option<u32>,
    #[arg(long, env = "MUMBLE_PLUGIN_MESSAGE_LIMIT")]
    plugin_message_limit: Option<u32>,
    #[arg(long, env = "MUMBLE_PLUGIN_MESSAGE_BURST")]
    plugin_message_burst: Option<u32>,
    #[arg(long, env = "MUMBLE_BROADCAST_LISTENER_VOLUME_ADJUSTMENTS")]
    broadcast_listener_volume_adjustments: Option<bool>,
    #[arg(long, env = "MUMBLE_CIPHERS")]
    ciphers: Option<String>,
    #[arg(long, env = "MUMBLE_SUGGEST_POSITIONAL")]
    suggest_positional: Option<bool>,
    #[arg(long, env = "MUMBLE_SUGGEST_PUSH_TO_TALK")]
    suggest_push_to_talk: Option<bool>,
    #[arg(long, env = "MUMBLE_LOG_GROUP_CHANGES")]
    log_group_changes: Option<bool>,
    #[arg(long, env = "MUMBLE_LOG_ACL_CHANGES")]
    log_acl_changes: Option<bool>,
    #[arg(long, env = "MUMBLE_ALLOW_RECORDING")]
    allow_recording: Option<bool>,
    #[arg(long, env = "MUMBLE_ROLLING_STATS_WINDOW")]
    rolling_stats_window: Option<u32>,
    #[arg(long, env = "MUMBLE_ABS_SETTINGS_FILE_PATH")]
    abs_settings_file_path: Option<String>,
    #[arg(long, env = "MUMBLE_SSL_CERT")]
    ssl_cert: Option<String>,
    #[arg(long, env = "MUMBLE_SSL_KEY")]
    ssl_key: Option<String>,

    /// Enable the terminal user interface
    #[arg(long)]
    tui: bool,

    /// Generate a new self-signed certificate and key
    #[arg(long)]
    generate_cert: bool,

    /// Generate a new self-signed certificate and key
    #[arg(long)]
    generate_keys: bool,

    /// Generate key from a SHA256 hash (hex string)
    #[arg(long)]
    key_from_hash: Option<String>,
}

pub struct Config {
    pub params: MetaParams,
    pub tui: bool,
    pub generate_cert: bool,
    pub generate_keys: bool,
    pub key_from_hash: Option<String>,
}

pub fn load_and_merge_config() -> Config {
    let args = Args::parse();
    let tui = args.tui;
    let generate_cert = args.generate_cert;
    let generate_keys = args.generate_keys;
    let key_from_hash = args.key_from_hash;

    // Load configuration from file, or use defaults if file is not found or invalid
    let mut params = read_config(&args.config).unwrap_or_else(|e| {
        info!(
            "Cannot read config file '{}': {}. Using defaults.",
            &args.config, e
        );
        MetaParams::default()
    });

    // Override with CLI arguments if provided
    if let Some(val) = args.base_path {
        params.base_path = val;
    }
    if !args.bind_addresses.is_empty() {
        params.bind_addresses = args.bind_addresses;
    }
    if let Some(val) = args.port {
        params.port = val;
    }
    if let Some(val) = args.timeout {
        params.timeout = val;
    }
    if let Some(val) = args.max_bandwidth {
        params.max_bandwidth = val;
    }
    if let Some(val) = args.max_users {
        params.max_users = val;
    }
    if let Some(val) = args.max_users_per_channel {
        params.max_users_per_channel = val;
    }
    if let Some(val) = args.max_listeners_per_channel {
        params.max_listeners_per_channel = val;
    }
    if let Some(val) = args.max_listener_proxies_per_user {
        params.max_listener_proxies_per_user = val;
    }
    if let Some(val) = args.default_channel {
        params.default_channel = val;
    }
    if let Some(val) = args.remember_channel {
        params.remember_channel = val;
    }
    if let Some(val) = args.remember_channel_duration {
        params.remember_channel_duration = val;
    }
    if let Some(val) = args.max_text_message_length {
        params.max_text_message_length = val;
    }
    if let Some(val) = args.max_image_message_length {
        params.max_image_message_length = val;
    }
    if let Some(val) = args.opus_threshold {
        params.opus_threshold = val;
    }
    if let Some(val) = args.channel_nesting_limit {
        params.channel_nesting_limit = val;
    }
    if let Some(val) = args.channel_count_limit {
        params.channel_count_limit = val;
    }
    if let Some(val) = args.legacy_password_hash {
        params.legacy_password_hash = val;
    }
    if let Some(val) = args.kdf_iterations {
        params.kdf_iterations = val;
    }
    if let Some(val) = args.allow_html {
        params.allow_html = val;
    }
    if let Some(val) = args.password {
        params.password = val;
    }
    if let Some(val) = args.welcome_text {
        params.welcome_text = val;
    }
    if let Some(val) = args.welcome_text_file {
        params.welcome_text_file = val;
    }
    if let Some(val) = args.cert_required {
        params.cert_required = val;
    }
    if let Some(val) = args.force_external_auth {
        params.force_external_auth = val;
    }
    if let Some(val) = args.ban_tries {
        params.ban_tries = val;
    }
    if let Some(val) = args.ban_timeframe {
        params.ban_timeframe = val;
    }
    if let Some(val) = args.ban_time {
        params.ban_time = val;
    }
    if let Some(val) = args.ban_successful {
        params.ban_successful = val;
    }
    if let Some(val) = args.database {
        params.database = val;
    }
    if let Some(val) = args.sqlite_wal {
        params.sqlite_wal = val;
    }
    if let Some(val) = args.db_driver {
        params.db_driver = val;
    }
    if let Some(val) = args.db_username {
        params.db_username = val;
    }
    if let Some(val) = args.db_password {
        params.db_password = val;
    }
    if let Some(val) = args.db_hostname {
        params.db_hostname = val;
    }
    if let Some(val) = args.db_prefix {
        params.db_prefix = val;
    }
    if let Some(val) = args.db_opts {
        params.db_opts = val;
    }
    if let Some(val) = args.db_port {
        params.db_port = val;
    }
    if let Some(val) = args.log_days {
        params.log_days = val;
    }
    if let Some(val) = args.obfuscate {
        params.obfuscate = val;
    }
    if let Some(val) = args.send_version {
        params.send_version = val;
    }
    if let Some(val) = args.allow_ping {
        params.allow_ping = val;
    }
    if let Some(val) = args.logfile {
        params.logfile = val;
    }
    if let Some(val) = args.pid_file {
        params.pid_file = val;
    }
    if let Some(val) = args.ice_endpoint {
        params.ice_endpoint = val;
    }
    if let Some(val) = args.ice_secret_read {
        params.ice_secret_read = val;
    }
    if let Some(val) = args.ice_secret_write {
        params.ice_secret_write = val;
    }
    if let Some(val) = args.reg_name {
        params.reg_name = val;
    }
    if let Some(val) = args.reg_password {
        params.reg_password = val;
    }
    if let Some(val) = args.reg_host {
        params.reg_host = val;
    }
    if let Some(val) = args.reg_location {
        params.reg_location = val;
    }
    if let Some(val) = args.reg_web_url {
        params.reg_web_url = val;
    }
    if let Some(val) = args.bonjour {
        params.bonjour = val;
    }
    if let Some(val) = args.message_limit {
        params.message_limit = val;
    }
    if let Some(val) = args.message_burst {
        params.message_burst = val;
    }
    if let Some(val) = args.plugin_message_limit {
        params.plugin_message_limit = val;
    }
    if let Some(val) = args.plugin_message_burst {
        params.plugin_message_burst = val;
    }
    if let Some(val) = args.broadcast_listener_volume_adjustments {
        params.broadcast_listener_volume_adjustments = val;
    }
    if let Some(val) = args.ciphers {
        params.ciphers = val;
    }
    if let Some(val) = args.suggest_positional {
        params.suggest_positional = Some(val);
    }
    if let Some(val) = args.suggest_push_to_talk {
        params.suggest_push_to_talk = Some(val);
    }
    if let Some(val) = args.log_group_changes {
        params.log_group_changes = val;
    }
    if let Some(val) = args.log_acl_changes {
        params.log_acl_changes = val;
    }
    if let Some(val) = args.allow_recording {
        params.allow_recording = val;
    }
    if let Some(val) = args.rolling_stats_window {
        params.rolling_stats_window = val;
    }
    if let Some(val) = args.abs_settings_file_path {
        params.abs_settings_file_path = val;
    }
    if let Some(val) = args.ssl_cert {
        params.ssl_cert = val;
    }
    if let Some(val) = args.ssl_key {
        params.ssl_key = val;
    }

    Config {
        params,
        tui,
        generate_cert,
        generate_keys,
        key_from_hash,
    }
}
