mod browser;
mod config;
mod holiday;

use anyhow::Result;
use browser::{Chrome, Driver};
use clap::{Parser, Subcommand, ValueEnum};
use config::{config_file, holiday_csv_dir, initialize_config_file, write};
use env_logger::init;
use holiday::{download, holiday_check};
use std::fmt::Display;
use thirtyfour::prelude::*;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Download {
        #[clap(short = 'u', long = "url")]
        url: Option<String>,
    },
    Execute {
        #[clap(short = 't', long = "type")]
        type_: Action,
        #[clap(short = 'H', long = "holiday")]
        holiday: Option<bool>,
        #[clap(short = 'd', long = "display")]
        display: Option<bool>,
    },
    ShowConfig,
    SetConfig {
        #[clap(short = 'u', long = "url")]
        url: Option<String>,
        #[clap(short = 'i', long = "id")]
        user_id: Option<String>,
        #[clap(short = 'p', long = "password")]
        password: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Action {
    Attendance,
    Leaving,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Attendance => "出勤",
                Action::Leaving => "退勤",
            }
        )
    }
}

pub async fn execute(
    url: &str,
    id: &str,
    password: &str,
    action: Action,
    holiday: bool,
    headless: bool,
) -> Result<()> {
    if holiday && holiday_check(&holiday_csv_dir()?) {
        return Ok(());
    }
    let hadler = Chrome::buid(headless).await?;
    // 対象のウェブページにアクセス
    hadler.driver.goto(url).await?;
    let name_input = hadler.driver.find(By::Id("login-name")).await?;
    name_input.send_keys(id).await?;
    let password_input = hadler.driver.find(By::Id("login-password")).await?;
    password_input.send_keys(password).await?;
    let login_btn = hadler.driver.find(By::Id("login-btn")).await?;
    login_btn.click().await?;
    loop {
        if !hadler.driver.find(By::Id("login-index")).await.is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    match action {
        Action::Attendance => {
            let attendance_btn = hadler.driver.find(By::Id("btn03")).await?;
            attendance_btn.click().await?;
        }
        Action::Leaving => {
            let leaving_btn = hadler.driver.find(By::Id("btn04")).await?;
            leaving_btn.click().await?;
        }
    }
    hadler.clear().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    initialize_config_file()?;
    let mut config = config_file()?;
    let cli = Cli::parse();
    match cli.subcommand {
        SubCommands::Download { url } => {
            let url = url
                .unwrap_or("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv".to_string());
            println!("休日CSVを{}からダウンロードしています。", url);
            download(&url, &holiday_csv_dir()?).await?;
            println!("ダウンロードが完了しました。");
        }
        SubCommands::Execute {
            type_,
            holiday,
            display,
        } => {
            println!("{}します。", type_);
            let url = config.url.unwrap();
            let user_id = config.user_id.unwrap();
            let password = config.password.unwrap();
            execute(
                &url,
                &user_id,
                &password,
                type_.clone(),
                holiday.unwrap_or(true),
                display.unwrap_or(true),
            )
            .await?;
            println!("{}が完了しました。", type_);
        }
        SubCommands::ShowConfig => {
            println!("{}", config);
        }
        SubCommands::SetConfig {
            url,
            user_id,
            password,
        } => {
            if url.is_some() {
                println!("URLを設定します。");
                config.url = url;
            }
            if user_id.is_some() {
                println!("UserIdを設定します。");
                config.user_id = user_id;
            }
            if password.is_some() {
                println!("Passwordを設定します。");
                config.password = password;
            }
            write(&config)?;
            println!("Configの設定が完了しました。");
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use std::env::var;

    #[tokio::test]
    async fn execute_test() -> Result<()> {
        dotenv().ok();
        let url = var("URL").unwrap();
        let user_id = var("USER_ID").unwrap();
        let password = var("PASSWORD").unwrap();
        let res = execute(&url, &user_id, &password, Action::Leaving, true, false).await;
        println!("{:?}", res);
        assert!(res.is_ok());
        panic!()
    }
}
