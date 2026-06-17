use chrono::{DateTime, Duration, Local, TimeZone};
use git2::Repository;

pub struct CommitInfo {
    pub short_hash: String,
    pub title:      String,
    pub author:     String,
    pub time_ago:   String,
    pub datetime:   String,
}

pub fn load_branch(repo_path: &str) -> String {
    let Ok(repo) = Repository::open(repo_path) else { return "?".into() };
    repo.head()
        .ok()
        .and_then(|h| h.shorthand().map(str::to_string))
        .unwrap_or_else(|| "?".into())
}

pub fn load_commits(repo_path: &str, count: usize) -> Vec<CommitInfo> {
    let repo = match Repository::open(repo_path) {
        Ok(r)  => r,
        Err(_) => return vec![],
    };

    let mut walk = match repo.revwalk() {
        Ok(w)  => w,
        Err(_) => return vec![],
    };

    if walk.push_head().is_err() { return vec![]; }
    walk.set_sorting(git2::Sort::TIME).ok();

    let now = Local::now();

    walk.take(count)
        .filter_map(|oid| {
            let oid    = oid.ok()?;
            let commit = repo.find_commit(oid).ok()?;

            let git_time  = commit.time();
            let offset    = chrono::FixedOffset::east_opt(git_time.offset_minutes() * 60)?;
            let committed: DateTime<chrono::FixedOffset> =
                offset.timestamp_opt(git_time.seconds(), 0).single()?;
            let committed_local = committed.with_timezone(&Local);

            Some(CommitInfo {
                short_hash: format!("{:.7}", oid),
                title:      commit.summary().unwrap_or("").to_string(),
                author:     commit.author().name().unwrap_or("?").to_string(),
                time_ago:   fmt_time_ago(now - committed_local),
                datetime:   committed_local.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
        })
        .collect()
}

fn fmt_time_ago(delta: Duration) -> String {
    let secs  = delta.num_seconds().max(0);
    let mins  = delta.num_minutes().max(0);
    let hours = delta.num_hours().max(0);
    let days  = delta.num_days().max(0);
    let weeks = days / 7;

    match secs {
        s if s < 60        => "just now".into(),
        _ if mins  < 60    => format!("{}m ago", mins),
        _ if hours < 24    => format!("{}h ago", hours),
        _ if days  < 7     => format!("{}d ago", days),
        _ if weeks < 52    => format!("{}w ago", weeks),
        _                  => format!("{}y ago", days / 365),
    }
}
