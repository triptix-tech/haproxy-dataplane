/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// SpoeAgent : SPOE agent configuration



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpoeAgent {
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<Async>,
    #[serde(rename = "continue-on-error", skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<ContinueOnError>,
    #[serde(rename = "dontlog-normal", skip_serializing_if = "Option::is_none")]
    pub dontlog_normal: Option<DontlogNormal>,
    #[serde(rename = "engine-name", skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "force-set-var", skip_serializing_if = "Option::is_none")]
    pub force_set_var: Option<ForceSetVar>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,
    #[serde(rename = "hello_timeout", skip_serializing_if = "Option::is_none")]
    pub hello_timeout: Option<i32>,
    #[serde(rename = "idle_timeout", skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
    /// HAProxy log target array (corresponds to log directives)
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<crate::models::LogTarget>>,
    #[serde(rename = "max-frame-size", skip_serializing_if = "Option::is_none")]
    pub max_frame_size: Option<i32>,
    #[serde(rename = "max-waiting-frames", skip_serializing_if = "Option::is_none")]
    pub max_waiting_frames: Option<i32>,
    #[serde(rename = "maxconnrate", skip_serializing_if = "Option::is_none")]
    pub maxconnrate: Option<i32>,
    #[serde(rename = "maxerrrate", skip_serializing_if = "Option::is_none")]
    pub maxerrrate: Option<i32>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "option_set-on-error", skip_serializing_if = "Option::is_none")]
    pub option_set_on_error: Option<String>,
    #[serde(rename = "option_set-process-time", skip_serializing_if = "Option::is_none")]
    pub option_set_process_time: Option<String>,
    #[serde(rename = "option_set-total-time", skip_serializing_if = "Option::is_none")]
    pub option_set_total_time: Option<String>,
    #[serde(rename = "option_var-prefix", skip_serializing_if = "Option::is_none")]
    pub option_var_prefix: Option<String>,
    #[serde(rename = "pipelining", skip_serializing_if = "Option::is_none")]
    pub pipelining: Option<Pipelining>,
    #[serde(rename = "processing_timeout", skip_serializing_if = "Option::is_none")]
    pub processing_timeout: Option<i32>,
    #[serde(rename = "register-var-names", skip_serializing_if = "Option::is_none")]
    pub register_var_names: Option<String>,
    #[serde(rename = "send-frag-payload", skip_serializing_if = "Option::is_none")]
    pub send_frag_payload: Option<SendFragPayload>,
    #[serde(rename = "use-backend", skip_serializing_if = "Option::is_none")]
    pub use_backend: Option<String>,
}

impl SpoeAgent {
    /// SPOE agent configuration
    pub fn new(name: String) -> SpoeAgent {
        SpoeAgent {
            r#async: None,
            continue_on_error: None,
            dontlog_normal: None,
            engine_name: None,
            force_set_var: None,
            groups: None,
            hello_timeout: None,
            idle_timeout: None,
            log: None,
            max_frame_size: None,
            max_waiting_frames: None,
            maxconnrate: None,
            maxerrrate: None,
            messages: None,
            name,
            option_set_on_error: None,
            option_set_process_time: None,
            option_set_total_time: None,
            option_var_prefix: None,
            pipelining: None,
            processing_timeout: None,
            register_var_names: None,
            send_frag_payload: None,
            use_backend: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Async {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for Async {
    fn default() -> Async {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContinueOnError {
    #[serde(rename = "enabled")]
    Enabled,
}

impl Default for ContinueOnError {
    fn default() -> ContinueOnError {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DontlogNormal {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for DontlogNormal {
    fn default() -> DontlogNormal {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ForceSetVar {
    #[serde(rename = "enabled")]
    Enabled,
}

impl Default for ForceSetVar {
    fn default() -> ForceSetVar {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Pipelining {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for Pipelining {
    fn default() -> Pipelining {
        Self::Enabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SendFragPayload {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for SendFragPayload {
    fn default() -> SendFragPayload {
        Self::Enabled
    }
}

