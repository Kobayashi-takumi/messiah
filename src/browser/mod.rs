use anyhow::Result;
use std::{
    path::PathBuf,
    process::{Child, Command},
};
use thirtyfour::prelude::*;

fn get_driver(name: &str) -> Result<PathBuf> {
    let path = which::which(name)?;
    Ok(path)
}

#[async_trait::async_trait]
pub trait Driver {
    fn run_driver() -> Result<Child>;
    async fn build_webdriver(headless: bool) -> Result<WebDriver>;
    async fn buid(headless: bool) -> Result<Handler>;
}

#[derive(Clone)]
pub struct Chrome;
#[async_trait::async_trait]
impl Driver for Chrome {
    fn run_driver() -> Result<Child> {
        let path = get_driver("chromedriver")?;
        let mut cmd = Command::new(&path);
        let child = cmd.spawn()?;
        Ok(child)
    }
    async fn build_webdriver(headless: bool) -> Result<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        if headless {
            caps.add_chrome_arg("--headless")?;
        }
        let driver = WebDriver::new(format!("http://localhost:{}", "9515").as_str(), caps).await?;
        Ok(driver)
    }
    async fn buid(headless: bool) -> Result<Handler> {
        let process = Self::run_driver()?;
        let driver = Self::build_webdriver(headless).await?;
        Ok(Handler { driver, process })
    }
}
unsafe impl Send for Chrome {}
unsafe impl Sync for Chrome {}

// #[derive(Clone)]
// struct Safari {
//     port: String,
// }
// #[async_trait::async_trait]
// impl Driver for Safari {
//     fn run(&self) -> Result<Child> {
//         let path = get_driver("safaridriver")?;
//         let mut cmd = Command::new(path);
//         cmd.arg("--port").arg(&self.port);
//         let child = cmd.spawn()?;
//         Ok(child)
//     }
//     async fn build(&self) -> Result<WebDriver> {
//         let caps = DesiredCapabilities::safari();
//         let driver = WebDriver::new(
//             format!("http://localhost:{}", self.port.as_str()).as_str(),
//             caps,
//         )
//         .await?;
//         Ok(driver)
//     }
// }
// unsafe impl Send for Safari {}
// unsafe impl Sync for Safari {}

pub struct Handler {
    pub driver: WebDriver,
    pub process: Child,
}

impl Handler {
    pub async fn clear(mut self) -> Result<()> {
        self.process.kill()?;
        self.driver.quit().await?;
        Ok(())
    }
}
