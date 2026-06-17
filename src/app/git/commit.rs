use git2::{IndexAddOption, Repository, Signature};
use super::super::error::{NekoError, NekoResult};
use super::push::push;

pub struct CommitConfig {
    pub username:   String,
    pub email:      String,
    pub message:    String,
    pub gpg_key_id: Option<String>,
}

pub fn stage_commit_push(
    repo_path:          &str,
    cfg:                &CommitConfig,
    request_passphrase: impl Fn(String) -> Option<String>,
) -> NekoResult<()> {
    let branch = {
        let repo = Repository::open(repo_path)
            .map_err(|e| NekoError::GitOpen(e.message().to_string()))?;

        stage_all(&repo)?;

        let tree_id = repo.index()
            .and_then(|mut i| i.write_tree())
            .map_err(|e| NekoError::GitWriteTree(e.message().to_string()))?;

        let tree = repo.find_tree(tree_id)
            .map_err(|e| NekoError::GitFindTree(e.message().to_string()))?;

        let sig = Signature::now(&cfg.username, &cfg.email)
            .map_err(|e| NekoError::GitSignature(e.message().to_string()))?;

        let parent  = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let parents: Vec<&git2::Commit> = parent.iter().collect();

        let branch = head_branch(&repo)?;

        match &cfg.gpg_key_id {
            Some(key_id) => {
                signed_commit(&repo, &sig, &cfg.message, &tree, &parents, key_id, &branch)?;
            }
            None => {
                repo.commit(Some("HEAD"), &sig, &sig, &cfg.message, &tree, &parents)
                    .map_err(|e| NekoError::GitCommit(e.message().to_string()))?;
            }
        }

        branch
    };

    push(repo_path, &branch, request_passphrase)
}

fn stage_all(repo: &Repository) -> NekoResult<()> {
    let mut index = repo.index()
        .map_err(|e| NekoError::GitStage(e.message().to_string()))?;

    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| NekoError::GitStage(e.message().to_string()))?;

    index.write()
        .map_err(|e| NekoError::GitStage(e.message().to_string()))
}

fn head_branch(repo: &Repository) -> NekoResult<String> {
    repo.head()
        .map_err(|_| NekoError::GitBranchUnknown)?
        .shorthand()
        .map(str::to_string)
        .ok_or(NekoError::GitBranchUnknown)
}

fn signed_commit(
    repo:    &Repository,
    sig:     &Signature,
    message: &str,
    tree:    &git2::Tree,
    parents: &[&git2::Commit],
    key_id:  &str,
    branch:  &str,
) -> NekoResult<()> {
    let buf = repo.commit_create_buffer(sig, sig, message, tree, parents)
        .map_err(|e| NekoError::GitCommitBuffer(e.message().to_string()))?;

    let commit_str = std::str::from_utf8(&buf)
        .map_err(|_| NekoError::GitCommitBufferEncoding)?;

    let gpg_sig_raw = crate::app::gpg::sign(commit_str, key_id)?;
    let gpg_sig     = gpg_sig_raw.trim();

    let oid = repo.commit_signed(commit_str, gpg_sig, Some("gpgsig"))
        .map_err(|e| NekoError::GitSignedCommit(e.message().to_string()))?;

    let refname = format!("refs/heads/{}", branch);

    repo.reference(&refname, oid, true, "nekosource: signed commit")
        .map_err(|e| NekoError::GitRefUpdate(e.message().to_string()))?;

    repo.set_head(&refname)
        .map_err(|e| NekoError::GitHeadSet(e.message().to_string()))
}