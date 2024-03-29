/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// Global : HAProxy global configuration



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Global {
    #[serde(rename = "anonkey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub anonkey: Option<Option<i32>>,
    #[serde(rename = "busy_polling", skip_serializing_if = "Option::is_none")]
    pub busy_polling: Option<bool>,
    #[serde(rename = "ca_base", skip_serializing_if = "Option::is_none")]
    pub ca_base: Option<String>,
    #[serde(rename = "chroot", skip_serializing_if = "Option::is_none")]
    pub chroot: Option<String>,
    #[serde(rename = "close_spread_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub close_spread_time: Option<Option<i32>>,
    #[serde(rename = "cluster_secret", skip_serializing_if = "Option::is_none")]
    pub cluster_secret: Option<String>,
    #[serde(rename = "cpu_maps", skip_serializing_if = "Option::is_none")]
    pub cpu_maps: Option<Vec<crate::models::GlobalCpuMapsInner>>,
    #[serde(rename = "crt_base", skip_serializing_if = "Option::is_none")]
    pub crt_base: Option<String>,
    #[serde(rename = "daemon", skip_serializing_if = "Option::is_none")]
    pub daemon: Option<Daemon>,
    #[serde(rename = "default_path", skip_serializing_if = "Option::is_none")]
    pub default_path: Option<Box<crate::models::GlobalDefaultPath>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "device_atlas_options", skip_serializing_if = "Option::is_none")]
    pub device_atlas_options: Option<Box<crate::models::GlobalDeviceAtlasOptions>>,
    #[serde(rename = "expose_experimental_directives", skip_serializing_if = "Option::is_none")]
    pub expose_experimental_directives: Option<bool>,
    #[serde(rename = "external_check", skip_serializing_if = "Option::is_none")]
    pub external_check: Option<bool>,
    #[serde(rename = "fifty_one_degrees_options", skip_serializing_if = "Option::is_none")]
    pub fifty_one_degrees_options: Option<Box<crate::models::GlobalFiftyOneDegreesOptions>>,
    #[serde(rename = "gid", skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[serde(rename = "grace", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub grace: Option<Option<i32>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "h1_case_adjust", skip_serializing_if = "Option::is_none")]
    pub h1_case_adjust: Option<Vec<crate::models::GlobalH1CaseAdjustInner>>,
    #[serde(rename = "h1_case_adjust_file", skip_serializing_if = "Option::is_none")]
    pub h1_case_adjust_file: Option<String>,
    #[serde(rename = "h2_workaround_bogus_websocket_clients", skip_serializing_if = "Option::is_none")]
    pub h2_workaround_bogus_websocket_clients: Option<bool>,
    #[serde(rename = "hard_stop_after", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hard_stop_after: Option<Option<i32>>,
    #[serde(rename = "httpclient_resolvers_disabled", skip_serializing_if = "Option::is_none")]
    pub httpclient_resolvers_disabled: Option<HttpclientResolversDisabled>,
    #[serde(rename = "httpclient_resolvers_id", skip_serializing_if = "Option::is_none")]
    pub httpclient_resolvers_id: Option<String>,
    #[serde(rename = "httpclient_resolvers_prefer", skip_serializing_if = "Option::is_none")]
    pub httpclient_resolvers_prefer: Option<HttpclientResolversPrefer>,
    #[serde(rename = "httpclient_retries", skip_serializing_if = "Option::is_none")]
    pub httpclient_retries: Option<i32>,
    #[serde(rename = "httpclient_ssl_ca_file", skip_serializing_if = "Option::is_none")]
    pub httpclient_ssl_ca_file: Option<String>,
    #[serde(rename = "httpclient_ssl_verify", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub httpclient_ssl_verify: Option<Option<HttpclientSslVerify>>,
    #[serde(rename = "httpclient_timeout_connect", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub httpclient_timeout_connect: Option<Option<i32>>,
    #[serde(rename = "insecure_fork_wanted", skip_serializing_if = "Option::is_none")]
    pub insecure_fork_wanted: Option<bool>,
    #[serde(rename = "insecure_setuid_wanted", skip_serializing_if = "Option::is_none")]
    pub insecure_setuid_wanted: Option<bool>,
    #[serde(rename = "issuers_chain_path", skip_serializing_if = "Option::is_none")]
    pub issuers_chain_path: Option<String>,
    #[serde(rename = "limited_quic", skip_serializing_if = "Option::is_none")]
    pub limited_quic: Option<bool>,
    #[serde(rename = "load_server_state_from_file", skip_serializing_if = "Option::is_none")]
    pub load_server_state_from_file: Option<LoadServerStateFromFile>,
    #[serde(rename = "localpeer", skip_serializing_if = "Option::is_none")]
    pub localpeer: Option<String>,
    #[serde(rename = "log_send_hostname", skip_serializing_if = "Option::is_none")]
    pub log_send_hostname: Option<Box<crate::models::GlobalLogSendHostname>>,
    #[serde(rename = "lua_load_per_thread", skip_serializing_if = "Option::is_none")]
    pub lua_load_per_thread: Option<String>,
    #[serde(rename = "lua_loads", skip_serializing_if = "Option::is_none")]
    pub lua_loads: Option<Vec<crate::models::GlobalLuaLoadsInner>>,
    #[serde(rename = "lua_prepend_path", skip_serializing_if = "Option::is_none")]
    pub lua_prepend_path: Option<Vec<crate::models::GlobalLuaPrependPathInner>>,
    #[serde(rename = "master-worker", skip_serializing_if = "Option::is_none")]
    pub master_worker: Option<bool>,
    #[serde(rename = "max_spread_checks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_spread_checks: Option<Option<i32>>,
    #[serde(rename = "maxcompcpuusage", skip_serializing_if = "Option::is_none")]
    pub maxcompcpuusage: Option<i32>,
    #[serde(rename = "maxcomprate", skip_serializing_if = "Option::is_none")]
    pub maxcomprate: Option<i32>,
    #[serde(rename = "maxconn", skip_serializing_if = "Option::is_none")]
    pub maxconn: Option<i32>,
    #[serde(rename = "maxconnrate", skip_serializing_if = "Option::is_none")]
    pub maxconnrate: Option<i32>,
    #[serde(rename = "maxpipes", skip_serializing_if = "Option::is_none")]
    pub maxpipes: Option<i32>,
    #[serde(rename = "maxsessrate", skip_serializing_if = "Option::is_none")]
    pub maxsessrate: Option<i32>,
    #[serde(rename = "maxsslconn", skip_serializing_if = "Option::is_none")]
    pub maxsslconn: Option<i32>,
    #[serde(rename = "maxsslrate", skip_serializing_if = "Option::is_none")]
    pub maxsslrate: Option<i32>,
    #[serde(rename = "maxzlibmem", skip_serializing_if = "Option::is_none")]
    pub maxzlibmem: Option<i32>,
    #[serde(rename = "mworker_max_reloads", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mworker_max_reloads: Option<Option<i32>>,
    #[serde(rename = "nbproc", skip_serializing_if = "Option::is_none")]
    pub nbproc: Option<i32>,
    #[serde(rename = "nbthread", skip_serializing_if = "Option::is_none")]
    pub nbthread: Option<i32>,
    #[serde(rename = "no-quic", skip_serializing_if = "Option::is_none")]
    pub no_quic: Option<bool>,
    #[serde(rename = "node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[serde(rename = "noepoll", skip_serializing_if = "Option::is_none")]
    pub noepoll: Option<bool>,
    #[serde(rename = "noevports", skip_serializing_if = "Option::is_none")]
    pub noevports: Option<bool>,
    #[serde(rename = "nogetaddrinfo", skip_serializing_if = "Option::is_none")]
    pub nogetaddrinfo: Option<bool>,
    #[serde(rename = "nokqueue", skip_serializing_if = "Option::is_none")]
    pub nokqueue: Option<bool>,
    #[serde(rename = "nopoll", skip_serializing_if = "Option::is_none")]
    pub nopoll: Option<bool>,
    #[serde(rename = "noreuseport", skip_serializing_if = "Option::is_none")]
    pub noreuseport: Option<bool>,
    #[serde(rename = "nosplice", skip_serializing_if = "Option::is_none")]
    pub nosplice: Option<bool>,
    #[serde(rename = "numa_cpu_mapping", skip_serializing_if = "Option::is_none")]
    pub numa_cpu_mapping: Option<NumaCpuMapping>,
    #[serde(rename = "pidfile", skip_serializing_if = "Option::is_none")]
    pub pidfile: Option<String>,
    #[serde(rename = "pp2_never_send_local", skip_serializing_if = "Option::is_none")]
    pub pp2_never_send_local: Option<bool>,
    #[serde(rename = "prealloc-fd", skip_serializing_if = "Option::is_none")]
    pub prealloc_fd: Option<bool>,
    #[serde(rename = "presetenv", skip_serializing_if = "Option::is_none")]
    pub presetenv: Option<Vec<crate::models::GlobalPresetenvInner>>,
    #[serde(rename = "profiling_tasks", skip_serializing_if = "Option::is_none")]
    pub profiling_tasks: Option<ProfilingTasks>,
    #[serde(rename = "quiet", skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
    #[serde(rename = "resetenv", skip_serializing_if = "Option::is_none")]
    pub resetenv: Option<String>,
    #[serde(rename = "runtime_apis", skip_serializing_if = "Option::is_none")]
    pub runtime_apis: Option<Vec<crate::models::GlobalRuntimeApisInner>>,
    #[serde(rename = "server_state_base", skip_serializing_if = "Option::is_none")]
    pub server_state_base: Option<String>,
    #[serde(rename = "server_state_file", skip_serializing_if = "Option::is_none")]
    pub server_state_file: Option<String>,
    #[serde(rename = "set_dumpable", skip_serializing_if = "Option::is_none")]
    pub set_dumpable: Option<bool>,
    #[serde(rename = "set_var", skip_serializing_if = "Option::is_none")]
    pub set_var: Option<Vec<crate::models::GlobalSetVarInner>>,
    #[serde(rename = "set_var_fmt", skip_serializing_if = "Option::is_none")]
    pub set_var_fmt: Option<Vec<crate::models::GlobalSetVarFmtInner>>,
    #[serde(rename = "setcap", skip_serializing_if = "Option::is_none")]
    pub setcap: Option<String>,
    #[serde(rename = "setenv", skip_serializing_if = "Option::is_none")]
    pub setenv: Option<Vec<crate::models::GlobalSetenvInner>>,
    #[serde(rename = "spread_checks", skip_serializing_if = "Option::is_none")]
    pub spread_checks: Option<i32>,
    #[serde(rename = "ssl_default_bind_ciphers", skip_serializing_if = "Option::is_none")]
    pub ssl_default_bind_ciphers: Option<String>,
    #[serde(rename = "ssl_default_bind_ciphersuites", skip_serializing_if = "Option::is_none")]
    pub ssl_default_bind_ciphersuites: Option<String>,
    #[serde(rename = "ssl_default_bind_client_sigalgs", skip_serializing_if = "Option::is_none")]
    pub ssl_default_bind_client_sigalgs: Option<String>,
    #[serde(rename = "ssl_default_bind_curves", skip_serializing_if = "Option::is_none")]
    pub ssl_default_bind_curves: Option<String>,
    #[serde(rename = "ssl_default_bind_options", skip_serializing_if = "Option::is_none")]
    pub ssl_default_bind_options: Option<String>,
    #[serde(rename = "ssl_default_bind_sigalgs", skip_serializing_if = "Option::is_none")]
    pub ssl_default_bind_sigalgs: Option<String>,
    #[serde(rename = "ssl_default_server_ciphers", skip_serializing_if = "Option::is_none")]
    pub ssl_default_server_ciphers: Option<String>,
    #[serde(rename = "ssl_default_server_ciphersuites", skip_serializing_if = "Option::is_none")]
    pub ssl_default_server_ciphersuites: Option<String>,
    #[serde(rename = "ssl_default_server_client_sigalgs", skip_serializing_if = "Option::is_none")]
    pub ssl_default_server_client_sigalgs: Option<String>,
    #[serde(rename = "ssl_default_server_curves", skip_serializing_if = "Option::is_none")]
    pub ssl_default_server_curves: Option<String>,
    #[serde(rename = "ssl_default_server_options", skip_serializing_if = "Option::is_none")]
    pub ssl_default_server_options: Option<String>,
    #[serde(rename = "ssl_default_server_sigalgs", skip_serializing_if = "Option::is_none")]
    pub ssl_default_server_sigalgs: Option<String>,
    #[serde(rename = "ssl_dh_param_file", skip_serializing_if = "Option::is_none")]
    pub ssl_dh_param_file: Option<String>,
    #[serde(rename = "ssl_engines", skip_serializing_if = "Option::is_none")]
    pub ssl_engines: Option<Vec<crate::models::GlobalSslEnginesInner>>,
    #[serde(rename = "ssl_load_extra_files", skip_serializing_if = "Option::is_none")]
    pub ssl_load_extra_files: Option<String>,
    #[serde(rename = "ssl_mode_async", skip_serializing_if = "Option::is_none")]
    pub ssl_mode_async: Option<SslModeAsync>,
    #[serde(rename = "ssl_propquery", skip_serializing_if = "Option::is_none")]
    pub ssl_propquery: Option<String>,
    #[serde(rename = "ssl_provider", skip_serializing_if = "Option::is_none")]
    pub ssl_provider: Option<String>,
    #[serde(rename = "ssl_provider_path", skip_serializing_if = "Option::is_none")]
    pub ssl_provider_path: Option<String>,
    #[serde(rename = "ssl_server_verify", skip_serializing_if = "Option::is_none")]
    pub ssl_server_verify: Option<SslServerVerify>,
    #[serde(rename = "ssl_skip_self_issued_ca", skip_serializing_if = "Option::is_none")]
    pub ssl_skip_self_issued_ca: Option<bool>,
    #[serde(rename = "stats_maxconn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stats_maxconn: Option<Option<i32>>,
    #[serde(rename = "stats_timeout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stats_timeout: Option<Option<i32>>,
    #[serde(rename = "strict_limits", skip_serializing_if = "Option::is_none")]
    pub strict_limits: Option<bool>,
    #[serde(rename = "thread_group_lines", skip_serializing_if = "Option::is_none")]
    pub thread_group_lines: Option<Vec<crate::models::GlobalThreadGroupLinesInner>>,
    #[serde(rename = "thread_groups", skip_serializing_if = "Option::is_none")]
    pub thread_groups: Option<i32>,
    #[serde(rename = "tune_options", skip_serializing_if = "Option::is_none")]
    pub tune_options: Option<Box<crate::models::GlobalTuneOptions>>,
    #[serde(rename = "tune_ssl_default_dh_param", skip_serializing_if = "Option::is_none")]
    pub tune_ssl_default_dh_param: Option<i32>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[serde(rename = "ulimit_n", skip_serializing_if = "Option::is_none")]
    pub ulimit_n: Option<i32>,
    #[serde(rename = "unsetenv", skip_serializing_if = "Option::is_none")]
    pub unsetenv: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "wurfl_options", skip_serializing_if = "Option::is_none")]
    pub wurfl_options: Option<Box<crate::models::GlobalWurflOptions>>,
    #[serde(rename = "zero_warning", skip_serializing_if = "Option::is_none")]
    pub zero_warning: Option<bool>,
}

impl Global {
    /// HAProxy global configuration
    pub fn new() -> Global {
        Global {
            anonkey: None,
            busy_polling: None,
            ca_base: None,
            chroot: None,
            close_spread_time: None,
            cluster_secret: None,
            cpu_maps: None,
            crt_base: None,
            daemon: None,
            default_path: None,
            description: None,
            device_atlas_options: None,
            expose_experimental_directives: None,
            external_check: None,
            fifty_one_degrees_options: None,
            gid: None,
            grace: None,
            group: None,
            h1_case_adjust: None,
            h1_case_adjust_file: None,
            h2_workaround_bogus_websocket_clients: None,
            hard_stop_after: None,
            httpclient_resolvers_disabled: None,
            httpclient_resolvers_id: None,
            httpclient_resolvers_prefer: None,
            httpclient_retries: None,
            httpclient_ssl_ca_file: None,
            httpclient_ssl_verify: None,
            httpclient_timeout_connect: None,
            insecure_fork_wanted: None,
            insecure_setuid_wanted: None,
            issuers_chain_path: None,
            limited_quic: None,
            load_server_state_from_file: None,
            localpeer: None,
            log_send_hostname: None,
            lua_load_per_thread: None,
            lua_loads: None,
            lua_prepend_path: None,
            master_worker: None,
            max_spread_checks: None,
            maxcompcpuusage: None,
            maxcomprate: None,
            maxconn: None,
            maxconnrate: None,
            maxpipes: None,
            maxsessrate: None,
            maxsslconn: None,
            maxsslrate: None,
            maxzlibmem: None,
            mworker_max_reloads: None,
            nbproc: None,
            nbthread: None,
            no_quic: None,
            node: None,
            noepoll: None,
            noevports: None,
            nogetaddrinfo: None,
            nokqueue: None,
            nopoll: None,
            noreuseport: None,
            nosplice: None,
            numa_cpu_mapping: None,
            pidfile: None,
            pp2_never_send_local: None,
            prealloc_fd: None,
            presetenv: None,
            profiling_tasks: None,
            quiet: None,
            resetenv: None,
            runtime_apis: None,
            server_state_base: None,
            server_state_file: None,
            set_dumpable: None,
            set_var: None,
            set_var_fmt: None,
            setcap: None,
            setenv: None,
            spread_checks: None,
            ssl_default_bind_ciphers: None,
            ssl_default_bind_ciphersuites: None,
            ssl_default_bind_client_sigalgs: None,
            ssl_default_bind_curves: None,
            ssl_default_bind_options: None,
            ssl_default_bind_sigalgs: None,
            ssl_default_server_ciphers: None,
            ssl_default_server_ciphersuites: None,
            ssl_default_server_client_sigalgs: None,
            ssl_default_server_curves: None,
            ssl_default_server_options: None,
            ssl_default_server_sigalgs: None,
            ssl_dh_param_file: None,
            ssl_engines: None,
            ssl_load_extra_files: None,
            ssl_mode_async: None,
            ssl_propquery: None,
            ssl_provider: None,
            ssl_provider_path: None,
            ssl_server_verify: None,
            ssl_skip_self_issued_ca: None,
            stats_maxconn: None,
            stats_timeout: None,
            strict_limits: None,
            thread_group_lines: None,
            thread_groups: None,
            tune_options: None,
            tune_ssl_default_dh_param: None,
            uid: None,
            ulimit_n: None,
            unsetenv: None,
            user: None,
            wurfl_options: None,
            zero_warning: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Daemon {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for Daemon {
    fn default() -> Daemon {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpclientResolversDisabled {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for HttpclientResolversDisabled {
    fn default() -> HttpclientResolversDisabled {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpclientResolversPrefer {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}

impl Default for HttpclientResolversPrefer {
    fn default() -> HttpclientResolversPrefer {
        Self::Ipv4
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpclientSslVerify {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "required")]
    Required,
}

impl Default for HttpclientSslVerify {
    fn default() -> HttpclientSslVerify {
        Self::Empty
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoadServerStateFromFile {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "none")]
    None,
}

impl Default for LoadServerStateFromFile {
    fn default() -> LoadServerStateFromFile {
        Self::Global
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NumaCpuMapping {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for NumaCpuMapping {
    fn default() -> NumaCpuMapping {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProfilingTasks {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

impl Default for ProfilingTasks {
    fn default() -> ProfilingTasks {
        Self::Auto
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SslModeAsync {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for SslModeAsync {
    fn default() -> SslModeAsync {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SslServerVerify {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "required")]
    Required,
}

impl Default for SslServerVerify {
    fn default() -> SslServerVerify {
        Self::None
    }
}

