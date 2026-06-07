use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use strum_macros::Display;

/// Source classification for client-supplied context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdditionalContextKind {
    Untrusted,
    Application,
}

/// Client-supplied context keyed by an opaque source identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalContextEntry {
    pub value: String,
    pub kind: AdditionalContextKind,
}

/// User's decision in response to an ExecApprovalRequest.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Display, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    /// User has approved this command and the agent should execute it.
    Approved,

    /// User has denied this command and the agent should not execute it, but
    /// it should continue the session and try something else.
    #[default]
    Denied,

    /// Automatic approval review timed out before reaching a decision.
    TimedOut,

    /// User has denied this command and the agent should not do anything until
    /// the user's next command.
    Abort,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct ByteRange {
    /// Start byte offset (inclusive) within the UTF-8 text buffer.
    pub start: usize,
    /// End byte offset (exclusive) within the UTF-8 text buffer.
    pub end: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct TextElement {
    /// Byte range in the parent `text` buffer that this element occupies.
    pub byte_range: ByteRange,
    /// Optional human-readable placeholder for the element, displayed in the UI.
    placeholder: Option<String>,
}

/// User input
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserInput {
    Text {
        text: String,
        /// UI-defined spans within `text` that should be treated as special elements.
        /// These are byte ranges into the UTF-8 `text` buffer and are used to render
        /// or persist rich input markers (e.g., image placeholders) across history
        /// and resume without mutating the literal text.
        #[serde(default)]
        text_elements: Vec<TextElement>,
    },

    /// Skill selected by the user (name + path to SKILL.md).
    Skill {
        name: String,
        path: std::path::PathBuf,
    },

    /// Explicit structured mention selected by the user.
    Mention { name: String, at_type: String },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct RequestUserInputAnswer {
    pub answers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct RequestUserInputResponse {
    pub answers: BTreeMap<String, RequestUserInputAnswer>,
}

/// Submission operation
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Op {
    /// Abort current task without terminating background terminal processes.
    /// This server sends [`EventMsg::TurnAborted`] in response.
    Interrupt,

    /// User input, optionally with thread-settings overrides applied first.
    UserInput {
        /// User input items, see `InputItem`
        items: Vec<UserInput>,
        /// Optional JSON Schema used to constrain the final assistant message for this turn.
        final_output_json_schema: Option<Value>,
        /// Client-supplied context fragments keyed by an opaque source identifier.
        additional_context: BTreeMap<String, AdditionalContextEntry>,
    },

    /// Approve a command execution
    ExecApproval {
        /// The id of the submission we are approving
        id: String,
        /// Turn id associated with the approval event, when available.
        turn_id: Option<String>,
        /// The user's decision in response to the request.
        decision: ReviewDecision,
    },

    /// Resolve a request_user_input tool call.
    UserInputAnswer {
        /// Turn id for the in-flight request.
        id: String,
        /// User-provided answers.
        response: RequestUserInputResponse,
    },

    /// Request to shut down codex instance.
    Shutdown,
}
