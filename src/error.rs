//! Error types for eoka

use thiserror::Error;

/// Result type for eoka operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for eoka
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to launch Chrome
    #[error("Failed to launch Chrome: {0}")]
    Launch(String),

    /// Transport error
    #[error("Transport error: {context}")]
    Transport {
        context: String,
        #[source]
        source: Option<std::io::Error>,
    },

    /// CDP protocol error
    #[error("CDP error in {method}: {message} (code {code})")]
    Cdp {
        method: String,
        code: i64,
        message: String,
    },

    /// CDP error without method context (for simple cases)
    #[error("CDP error: {0}")]
    CdpSimple(String),

    /// Navigation error
    #[error("Navigation error: {0}")]
    Navigation(String),

    /// Element not found in DOM
    #[error("Element not found: {0}")]
    ElementNotFound(String),

    /// Element exists in DOM but is not visible/rendered
    #[error("Element not visible: '{selector}' exists in DOM but is not rendered (hidden, display:none, or off-screen)")]
    ElementNotVisible { selector: String },

    /// Element exists but cannot be interacted with
    #[error("Element not interactive: '{selector}' is {reason}")]
    ElementNotInteractive { selector: String, reason: String },

    /// Frame/iframe not found
    #[error("Frame not found: {0}")]
    FrameNotFound(String),

    /// Timeout
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Decode error (e.g., base64)
    #[error("Decode error: {0}")]
    Decode(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Chrome not found
    #[error("Chrome not found")]
    ChromeNotFound,

    /// Binary patching error
    #[error("Patching error in {operation}: {message}")]
    Patching { operation: String, message: String },

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Retry exhausted
    #[error("Retry exhausted after {attempts} attempts: {last_error}")]
    RetryExhausted { attempts: u32, last_error: String },
}

impl Error {
    /// Create a transport error with context
    pub fn transport(context: impl Into<String>) -> Self {
        Self::Transport {
            context: context.into(),
            source: None,
        }
    }

    /// Create a transport error with IO source
    pub fn transport_io(context: impl Into<String>, source: std::io::Error) -> Self {
        Self::Transport {
            context: context.into(),
            source: Some(source),
        }
    }

    /// Create a CDP error with full context
    pub fn cdp(method: impl Into<String>, code: i64, message: impl Into<String>) -> Self {
        Self::Cdp {
            method: method.into(),
            code,
            message: message.into(),
        }
    }

    /// Create a patching error
    pub fn patching(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Patching {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Create an element not visible error
    pub fn not_visible(selector: impl Into<String>) -> Self {
        Self::ElementNotVisible {
            selector: selector.into(),
        }
    }

    /// Create an element not interactive error
    pub fn not_interactive(selector: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ElementNotInteractive {
            selector: selector.into(),
            reason: reason.into(),
        }
    }

    /// Check if this is a "box model" error (element not visible)
    pub fn is_not_visible(&self) -> bool {
        match self {
            Error::Cdp { message, .. } => message.contains("box model"),
            Error::ElementNotVisible { .. } => true,
            _ => false,
        }
    }

    /// Convert CDP box model errors to friendlier ElementNotVisible
    pub fn clarify(self, selector: &str) -> Self {
        match &self {
            Error::Cdp { message, .. } if message.contains("box model") => {
                Error::not_visible(selector)
            }
            _ => self,
        }
    }
}
