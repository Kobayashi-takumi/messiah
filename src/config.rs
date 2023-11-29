use anyhow::{anyhow, Result};
use etcetera::{choose_base_strategy, BaseStrategy};
use once_cell::sync::OnceCell;
use std::fmt::Display;
use std::fs::{read_to_string, write as write_, File};
use std::path::{Path, PathBuf};
use toml::{from_str, to_string};

pub static CONFIG_FILE: OnceCell<PathBuf> = once_cell::sync::OnceCell::new();
static CONFIG_FILE_NAME: &str = "config.toml";
static HOLIDAY_FILE_NAME: &str = "syukujitsu.csv";

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub url: Option<String>,
    pub user_id: Option<String>,
    pub password: Option<String>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
##### Config ####
Url: {}
UserId: {}
Password: {}
#################
"#,
            self.url.clone().unwrap_or("Not set".to_string()),
            self.user_id.clone().unwrap_or("Not set".to_string()),
            self.password.clone().unwrap_or("Not set".to_string())
        )
    }
}

fn ensure_parent_dir(path: &Path) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).ok();
        }
    }
}

fn config_dir() -> Result<PathBuf> {
    let strategy = choose_base_strategy().expect("configのディレクトリが見つかりませんでした。");
    let mut path = strategy.config_dir();
    path.push("messiah");
    let path = path.join(CONFIG_FILE_NAME);
    Ok(path)
}

pub fn holiday_csv_dir() -> Result<String> {
    let strategy = choose_base_strategy().expect("configのディレクトリが見つかりませんでした。");
    let mut path = strategy.config_dir();
    path.push("messiah");
    let path = path.join(HOLIDAY_FILE_NAME);
    Ok(path
        .to_str()
        .ok_or(anyhow!("config fileのロードに失敗しました。"))?
        .to_string())
}

fn load_path() -> Result<PathBuf> {
    CONFIG_FILE
        .get()
        .map(|path| path.to_path_buf())
        .ok_or(anyhow!("config fileのロードに失敗しました。"))
}

pub fn config_file() -> Result<Config> {
    let path = load_path()?;
    if !path.exists() {
        File::create(path.clone())?;
    }
    let buf: String = read_to_string(path.clone())
        .map_err(|_e| anyhow!("config fileのロードに失敗しました。"))?;
    let config: Config =
        from_str(&buf).map_err(|_e| anyhow!("config fileのパースに失敗しました。"))?;
    Ok(config)
}

pub fn initialize_config_file() -> Result<()> {
    let config_file = config_dir()?;
    ensure_parent_dir(&config_file);
    match CONFIG_FILE.set(config_file) {
        Ok(_) => {}
        Err(_) => return Err(anyhow!("Failed to set a config file.")),
    };
    Ok(())
}

pub fn write(config: &Config) -> Result<()> {
    let path = load_path()?;
    let target =
        to_string(config).map_err(|_e| anyhow!("config fileのシリアライズに失敗しました。"))?;
    write_(path, target.as_bytes())
        .map_err(|_e| anyhow!("config fileの書き込みに失敗しました。"))?;
    Ok(())
}
