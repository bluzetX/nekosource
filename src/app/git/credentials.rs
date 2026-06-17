use ssh_key::{LineEnding, PrivateKey};
use std::{
    io::Write,
    path::{Path, PathBuf},
    os::windows::process::CommandExt
};
use super::super::error::{NekoError, NekoResult};

const MAX_PASSPHRASE_RETRIES: usize = 3;

pub fn find_ssh_keys() -> Vec<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    let ssh_dir = Path::new(&home).join(".ssh");
    ["id_ed25519", "id_rsa", "id_ecdsa", "id_dsa"]
        .iter()
        .map(|name| ssh_dir.join(name))
        .filter(|p| p.exists())
        .collect()
}

pub struct TempKey {
    path: PathBuf,
}

impl TempKey {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempKey {
    fn drop(&mut self) {
        if let Ok(meta) = std::fs::metadata(&self.path) {
            let zeros = vec![0u8; meta.len() as usize];
            let _ = std::fs::write(&self.path, &zeros);
        }
        let _ = std::fs::remove_file(&self.path);
    }
}

pub fn prepare_ssh_key(
    request_passphrase: &impl Fn(String) -> Option<String>,
) -> NekoResult<Option<TempKey>> {
    for key_path in find_ssh_keys() {
        let key_path_str = key_path.to_string_lossy().into_owned();

        let raw = match std::fs::read_to_string(&key_path) {
            Ok(s)  => s,
            Err(_) => continue,
        };

        let private_key = match PrivateKey::from_openssh(&raw) {
            Ok(k)  => k,
            Err(_) => continue,
        };

        let ready_key: PrivateKey = if private_key.is_encrypted() {
            let mut result = None;

            for _ in 0..MAX_PASSPHRASE_RETRIES {
                match request_passphrase(key_path_str.clone()) {
                    None     => return Err(NekoError::SshPassphraseCancelled),
                    Some(pp) => {
                        match private_key.decrypt(pp.as_bytes()) {
                            Ok(k)  => { result = Some(k); break; }
                            Err(_) => { /* wrong passphrase -> loop */ }
                        }
                    }
                }
            }

            match result {
                Some(k) => k,
                None    => continue,
            }
        } else {
            private_key
        };

        let pem = match ready_key.to_openssh(LineEnding::LF) {
            Ok(p)  => p,
            Err(_) => continue,
        };

        let tmp_path = std::env::temp_dir()
            .join(format!("nekosource_{}.key", std::process::id()));

        {
            let mut f = std::fs::File::create(&tmp_path)
                .map_err(|e| NekoError::GitClone(e.to_string()))?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                f.set_permissions(std::fs::Permissions::from_mode(0o600))
                    .map_err(|e| NekoError::GitClone(e.to_string()))?;
            }

            f.write_all(pem.as_bytes())
                .map_err(|e| NekoError::GitClone(e.to_string()))?;
        }

        #[cfg(windows)]
        restrict_key_permissions_windows(&tmp_path);

        return Ok(Some(TempKey { path: tmp_path }));
    }

    Ok(None)
}

pub fn build_ssh_command(key_path: &Path) -> String {
    let raw = key_path.to_str().unwrap_or("");

    #[cfg(windows)]
    {
        let fwd = raw.replace('\\', "/");
        format!(
            "ssh -i \"{}\" -o BatchMode=yes -o StrictHostKeyChecking=accept-new",
            fwd
        )
    }
    #[cfg(not(windows))]
    {
        format!(
            "ssh -i '{}' -o BatchMode=yes -o StrictHostKeyChecking=accept-new",
            raw
        )
    }
}

#[cfg(windows)]
fn restrict_key_permissions_windows(path: &Path) {
    let path_str = match path.to_str() {
        Some(s) => s,
        None    => return,
    };

    let _ = std::process::Command::new("icacls")
        .args([path_str, "/inheritance:r"])
        .creation_flags(0x08000000)
        .output();

    if let Ok(user) = std::env::var("USERNAME") {
        let grant = format!("{}:(F)", user);
        let _ = std::process::Command::new("icacls")
            .args([path_str, "/grant:r", &grant])
            .creation_flags(0x08000000)
            .output();
    }
}