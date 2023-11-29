use anyhow::Result;
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Utc, Weekday};
use reqwest;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(serde::Deserialize, Debug)]
struct HolidayRecord {
    date: String,
}

pub async fn download(url: &str, out_path: &str) -> Result<()> {
    let temp_path = "temp_file.csv";
    let path = Path::new(out_path);
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    let content_str = String::from_utf8_lossy(&content);
    let mut temp_file = File::create(temp_path)?;
    temp_file.write_all(&content_str.as_bytes())?;
    let temp_file = File::open(temp_path)?;
    let reader = BufReader::new(temp_file);
    let mut file = File::create(path)?;

    for (index, line) in reader.lines().enumerate() {
        if index != 0 {
            // 最初の行を除く
            writeln!(file, "{}", line?)?;
        }
    }
    remove_file(temp_path)?;
    Ok(())
}

fn load(path: &str) -> Result<Vec<NaiveDate>> {
    let file = File::open(path)?;
    let mut rows: Vec<NaiveDate> = vec![];
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    rdr.records().next();
    for result in rdr.deserialize() {
        let record: HolidayRecord = result?;
        let holiday = NaiveDate::parse_from_str(&record.date, "%Y/%m/%d")?;
        rows.push(holiday)
    }
    Ok(rows)
}

pub fn holiday_check(path: &str) -> bool {
    let holidays = load(path).unwrap_or(vec![]);
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
    let today: DateTime<Utc> = Utc::now();
    let today = today.with_timezone(&jst_offset);
    let date: NaiveDate = today.date_naive();
    let weekday = today.weekday();
    holidays.contains(&date) || weekday == Weekday::Sat || weekday == Weekday::Sun
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::holiday_csv_dir;

    #[tokio::test]
    async fn download_test() -> Result<()> {
        let res = download(
            "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv",
            &holiday_csv_dir()?,
        )
        .await;
        println!("{:?}", res);
        assert!(res.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn load_test() -> Result<()> {
        let res = load(&holiday_csv_dir()?);
        assert!(res.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn holiday_check_test() -> Result<()> {
        let res = holiday_check(&holiday_csv_dir()?);
        assert_eq!(res, false);
        Ok(())
    }
}
