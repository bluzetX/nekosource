use std::fmt;
use super::language::Locale;


#[derive(Debug)]
pub enum NekoError {
    SshPassphraseCancelled,

    GitDestExists(String),
    GitOpen(String),
    GitClone(String),
    GitStage(String),
    GitWriteTree(String),
    GitFindTree(String),
    GitSignature(String),
    GitCommit(String),
    GitCommitBuffer(String),
    GitCommitBufferEncoding,
    GitSignedCommit(String),
    GitRefUpdate(String),
    GitHeadSet(String),
    GitPush(String),
    GitBranchUnknown,

    GitConfigNotFound(String),
    GitConfigOpen(String),
    GitConfigWriteName(String),
    GitConfigWriteEmail(String),

    GpgLaunch(String),
    GpgStdinWrite(String),
    GpgWait(String),
    GpgFailure(String),
    GpgOutputEncoding(String),
}

pub type NekoResult<T> = Result<T, NekoError>;

impl NekoError {
    pub fn localize(&self, locale: &Locale) -> String {
        match self {
            Self::SshPassphraseCancelled =>
                locale.t("err-ssh-cancelled"),

            Self::GitDestExists(path) =>
                locale.t_args("err-git-dest-exists",    &[("path",   path)]),
            Self::GitOpen(d) =>
                locale.t_args("err-git-open",           &[("detail", d)]),
            Self::GitClone(d) =>
                locale.t_args("err-git-clone",          &[("detail", d)]),
            Self::GitStage(d) =>
                locale.t_args("err-git-stage",          &[("detail", d)]),
            Self::GitWriteTree(d) =>
                locale.t_args("err-git-write-tree",     &[("detail", d)]),
            Self::GitFindTree(d) =>
                locale.t_args("err-git-find-tree",      &[("detail", d)]),
            Self::GitSignature(d) =>
                locale.t_args("err-git-signature",      &[("detail", d)]),
            Self::GitCommit(d) =>
                locale.t_args("err-git-commit",         &[("detail", d)]),
            Self::GitCommitBuffer(d) =>
                locale.t_args("err-git-commit-buffer",  &[("detail", d)]),
            Self::GitCommitBufferEncoding =>
                locale.t("err-git-commit-encoding"),
            Self::GitSignedCommit(d) =>
                locale.t_args("err-git-signed-commit",  &[("detail", d)]),
            Self::GitRefUpdate(d) =>
                locale.t_args("err-git-ref-update",     &[("detail", d)]),
            Self::GitHeadSet(d) =>
                locale.t_args("err-git-head-set",       &[("detail", d)]),
            Self::GitPush(d) =>
                locale.t_args("err-git-push",           &[("detail", d)]),
            Self::GitBranchUnknown =>
                locale.t("err-git-branch-unknown"),

            Self::GitConfigNotFound(d) =>
                locale.t_args("err-git-config-not-found",   &[("detail", d)]),
            Self::GitConfigOpen(d) =>
                locale.t_args("err-git-config-open",         &[("detail", d)]),
            Self::GitConfigWriteName(d) =>
                locale.t_args("err-git-config-write-name",   &[("detail", d)]),
            Self::GitConfigWriteEmail(d) =>
                locale.t_args("err-git-config-write-email",  &[("detail", d)]),

            Self::GpgLaunch(d) =>
                locale.t_args("err-gpg-launch",    &[("detail", d)]),
            Self::GpgStdinWrite(d) =>
                locale.t_args("err-gpg-stdin",     &[("detail", d)]),
            Self::GpgWait(d) =>
                locale.t_args("err-gpg-wait",      &[("detail", d)]),
            Self::GpgFailure(d) =>
                locale.t_args("err-gpg-failure",   &[("detail", d)]),
            Self::GpgOutputEncoding(d) =>
                locale.t_args("err-gpg-encoding",  &[("detail", d)]),
        }
    }
}

impl fmt::Display for NekoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SshPassphraseCancelled =>
                write!(f, "SSH passphrase cancelled"),
            Self::GitDestExists(p) =>
                write!(f, "Destination already exists: '{p}'"),
            Self::GitOpen(d) =>
                write!(f, "Failed to open repository: {d}"),
            Self::GitClone(d) =>
                write!(f, "Clone failed: {d}"),
            Self::GitStage(d) =>
                write!(f, "Failed to stage changes: {d}"),
            Self::GitWriteTree(d) =>
                write!(f, "Failed to write index tree: {d}"),
            Self::GitFindTree(d) =>
                write!(f, "Failed to find tree object: {d}"),
            Self::GitSignature(d) =>
                write!(f, "Failed to create signature: {d}"),
            Self::GitCommit(d) =>
                write!(f, "Failed to create commit: {d}"),
            Self::GitCommitBuffer(d) =>
                write!(f, "Failed to create commit buffer: {d}"),
            Self::GitCommitBufferEncoding =>
                write!(f, "Commit buffer contains invalid UTF-8"),
            Self::GitSignedCommit(d) =>
                write!(f, "Failed to create signed commit: {d}"),
            Self::GitRefUpdate(d) =>
                write!(f, "Failed to update branch reference: {d}"),
            Self::GitHeadSet(d) =>
                write!(f, "Failed to move HEAD: {d}"),
            Self::GitPush(d) =>
                write!(f, "Push failed: {d}"),
            Self::GitBranchUnknown =>
                write!(f, "Cannot determine current branch name"),
            Self::GitConfigNotFound(d) =>
                write!(f, "Global git config not found: {d}"),
            Self::GitConfigOpen(d) =>
                write!(f, "Failed to open git config: {d}"),
            Self::GitConfigWriteName(d) =>
                write!(f, "Failed to write user.name: {d}"),
            Self::GitConfigWriteEmail(d) =>
                write!(f, "Failed to write user.email: {d}"),
            Self::GpgLaunch(d) =>
                write!(f, "Failed to launch gpg: {d}"),
            Self::GpgStdinWrite(d) =>
                write!(f, "Failed to write GPG stdin: {d}"),
            Self::GpgWait(d) =>
                write!(f, "GPG process wait failed: {d}"),
            Self::GpgFailure(d) =>
                write!(f, "GPG exited with error:\n{d}"),
            Self::GpgOutputEncoding(d) =>
                write!(f, "GPG output is not valid UTF-8: {d}"),
        }
    }
}

impl std::error::Error for NekoError {}