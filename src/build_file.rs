use std::{
    collections::HashMap,
    env,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    process::Command,
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuildFile {
    #[serde(default)]
    build: HashMap<String, Build>,
}

#[derive(Deserialize, Debug)]
struct Build {
    cmd: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    envs: HashMap<String, String>,
}

pub fn load(path: String) -> Result<BuildFile> {
    let text = read_to_string(&path).map_err(|e| Error::new(e.kind(), format!("{path}, {e}")))?;
    let file = toml::from_str(&text).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(file)
}

pub fn build_file_compile(path: String, project_root: String) -> Result<(String, String, i32)> {
    let file = load(path)?;

    let Some(r) = file.build.get("release") else {
        Some("no [build.release] found".to_string());
        return Ok(("".to_string(), "no [build.release] found".to_string(), 1));
    };

    let mut envs: HashMap<String, String> = env::vars().collect();
    envs.extend(r.envs.clone());

    let out = Command::new(&r.cmd)
        .args(&r.args)
        .arg(project_root)
        .envs(&envs)
        .output()?;

    let stdout_str = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr_str = String::from_utf8_lossy(&out.stderr).to_string();
    let status_num = out.status.code().unwrap_or(1);

    Ok((stdout_str, stderr_str, status_num))
}
