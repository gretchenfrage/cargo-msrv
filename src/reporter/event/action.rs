use crate::toolchain::OwnedToolchainSpec;
use crate::ReleaseSource;
use rust_releases::semver;
use std::path::{Path, PathBuf};

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Action {
    name: &'static str,
    #[serde(skip)]
    status: ActionStatus,
    details: ActionDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<ScopePosition>,
}

impl Action {
    fn new(action: ActionDetails) -> Self {
        Self {
            name: (&action).into(),
            status: (&action).into(),
            details: action,
            scope: None,
        }
    }

    pub(in crate::reporter) fn clone_with_scope_position(&self, position: ScopePosition) -> Self {
        let mut cloned = self.clone();
        cloned.scope = Some(position);
        cloned
    }

    pub fn status(&self) -> ActionStatus {
        self.status
    }

    pub fn details(&self) -> &ActionDetails {
        &self.details
    }

    pub fn must_report(&self) -> bool {
        matches!(self.scope, Some(ScopePosition::Start) | None)
    }
}

/// Specialized `new` methods which provide a shortcut to create actions.
///
/// Without these short cuts, you would have to create an action like so (assuming `new` would
/// be public):
///
/// ```no_run
/// Action::new(ActionDetails::FetchingIndex { source });
/// ```
impl Action {
    pub fn fetching_index(source: ReleaseSource) -> Self {
        Self::new(ActionDetails::FetchingIndex { source })
    }

    pub fn setup_toolchain(toolchain: OwnedToolchainSpec) -> Self {
        Self::new(ActionDetails::SetupToolchain { toolchain })
    }

    pub fn check_toolchain(toolchain: OwnedToolchainSpec) -> Self {
        Self::new(ActionDetails::CheckToolchain { toolchain })
    }

    pub fn run_toolchain_check(version: semver::Version) -> Self {
        Self::new(ActionDetails::RunToolchainCheck { version })
    }

    pub fn run_toolchain_check_pass(version: semver::Version) -> Self {
        Self::new(ActionDetails::RunToolchainCheckPass { version })
    }

    pub fn run_toolchain_check_fail(version: semver::Version, error_msg: String) -> Self {
        Self::new(ActionDetails::RunToolchainCheckFail {
            version,
            error_message: error_msg,
        })
    }
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionDetails {
    SetupToolchain {
        toolchain: OwnedToolchainSpec,
    },
    CheckToolchain {
        toolchain: OwnedToolchainSpec,
    },
    RunToolchainCheck {
        version: semver::Version,
    },
    RunToolchainCheckPass {
        version: semver::Version,
    },
    RunToolchainCheckFail {
        version: semver::Version,
        error_message: String, // TODO: possibly we had a flag which disabled printing the error msg
    },
}

impl<'reference> From<&'reference ActionDetails> for ActionStatus {
    fn from(action_details: &'reference ActionDetails) -> Self {
        match action_details {
            ActionDetails::FetchingIndex { .. } => Self::Fetching,
            ActionDetails::SetupToolchain { .. } => Self::Setup, // consider: split in check-if-present & install?
            ActionDetails::CheckToolchain { .. } => Self::Check,
            ActionDetails::RunToolchainCheck { .. } => Self::Running,
            ActionDetails::RunToolchainCheckPass { .. } => Self::Passed,
            ActionDetails::RunToolchainCheckFail { .. } => Self::Failed,
        }
    }
}

impl<'reference> From<&'reference ActionDetails> for &'static str {
    fn from(action_details: &'reference ActionDetails) -> Self {
        match action_details {
            ActionDetails::FetchingIndex { .. } => "fetching_index",
            ActionDetails::SetupToolchain { .. } => "setup_toolchain",
            ActionDetails::CheckToolchain { .. } => "check",
            ActionDetails::RunToolchainCheck { .. } => "run_check",
            ActionDetails::RunToolchainCheckPass { .. } => "check_passed",
            ActionDetails::RunToolchainCheckFail { .. } => "check_failed",
        }
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Fetching,
    Check,
    Setup,
    Running,

    Passed,
    Failed,
}

impl ActionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fetching => "Fetching",

            Self::Check => "Check",
            Self::Setup => "Setup",
            Self::Running => "Running",

            Self::Passed => "[Pass]",
            Self::Failed => "[Fail]",
        }
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopePosition {
    Start,
    End,
}
