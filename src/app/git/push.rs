use std::process::Command;
use super::credentials::{build_ssh_command, prepare_ssh_key};
use super::super::error::{NekoError, NekoResult};

pub fn push(
    repo_path:          &str,
    branch:             &str,
    request_passphrase: impl Fn(String) -> Option<String>,
) -> NekoResult<()> {
    let remote_url = remote_url(repo_path);

    let key_guard = if remote_url.as_deref().map(is_ssh_url).unwrap_or(false) {
        prepare_ssh_key(&request_passphrase)?
    } else {
        None
    };

    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_path)
        .arg("push")
        .arg("origin")
        .arg(branch)
        .env("GIT_TERMINAL_PROMPT", "0");

    if let Some(ref key) = key_guard {
        cmd.env("GIT_SSH_COMMAND", build_ssh_command(key.path()));
    }

    let output = cmd
        .output()
        .map_err(|e| NekoError::GitPush(e.to_string()))?;

    drop(key_guard);

    if output.status.success() {
        Ok(())
    } else {
        let msg = String::from_utf8_lossy(&output.stderr)
            .trim()
            .to_string();
        Err(NekoError::GitPush(msg))
    }
}

fn remote_url(repo_path: &str) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn is_ssh_url(url: &str) -> bool {
    url.starts_with("ssh://") || (!url.starts_with("http") && url.contains('@'))
}