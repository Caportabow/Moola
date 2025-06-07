use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use std::io::Cursor;
use chrono::{NaiveDate, Datelike};
use scraper::{Html, Selector};
use regex::Regex;
use anyhow::Result;

// ========== Data Structures ==========
#[derive(Debug, Serialize)]
pub struct YearlyRates {
    pub best: f32,
    pub average: f32,
    pub worst: f32,
}

#[derive(Debug, Serialize)]
pub struct MonthlyRates {
    pub best: f32,
    pub average: f32,
    pub worst: f32,
}

#[derive(Debug, Serialize)]
pub struct HistoricalRates {
    pub yearly: Option<YearlyRates>,
    pub monthly: Option<MonthlyRates>,
}

#[derive(Debug, Serialize)]
pub struct CurrentRate {
    pub rate: f64,
    pub from: String,
    pub to: String,
    pub date: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    #[serde(rename = "Rate")]
    rate: Option<f64>,

    #[serde(rename = "UpdatedDateTimeUTC")]
    date: Option<String>,

    #[serde(rename = "ErrorMessage")]
    error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub name: String,
    pub code: String,
}

// ========== Historical Rates ==========
#[tauri::command]
async fn parse_exchange_rates(
    from: String,
    to: String,
    year: String,
    month: String,
) -> Result<HistoricalRates, String> {
    let url = format!(
        "https://www.exchange-rates.org/exchange-rate-history/{}-{}-{}",
        from.to_lowercase(),
        to.to_lowercase(),
        year
    );

    let html = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    // --- ПАРСИМ HTML ДО await CSV ---
    let (yearly, monthly_from_html) = {
        let document = Html::parse_document(&html);
        let tr_selector = Selector::parse("table.history-rate-summary tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let regex = Regex::new(r"([\d.]+)\s+[A-Z]{3}").unwrap();

        let mut yearly = YearlyRates {
            best: 0.0,
            average: 0.0,
            worst: 0.0,
        };

        for row in document.select(&tr_selector) {
            let tds: Vec<_> = row.select(&td_selector).collect();
            if tds.len() >= 2 {
                let label = tds[0].text().collect::<String>().to_lowercase();
                let value_text = tds[1].text().collect::<String>();
                if let Some(caps) = regex.captures(&value_text) {
                    if let Ok(value) = caps[1].parse::<f32>() {
                        if label.contains("highest") {
                            yearly.best = value;
                        } else if label.contains("lowest") {
                            yearly.worst = value;
                        } else if label.contains("average") {
                            yearly.average = value;
                        }
                    }
                }
            }
        }

        // Парсим месячные данные
        let footer_selector = Selector::parse("td.month-footer").unwrap();
        let nowrap_selector = Selector::parse("span.nowrap").unwrap();
        let mut monthly = None;

        for footer in document.select(&footer_selector) {
            let inner_html = footer.inner_html();
            if inner_html.contains(&month) {
                let fragment = Html::parse_fragment(&inner_html);

                let values = fragment
                    .select(&nowrap_selector)
                    .filter_map(|el| {
                        let text = el.text().collect::<String>();
                        let after_eq = text.split('=').nth(1)?.trim();
                        let numeric_part: String = after_eq
                            .replace(",", "")
                            .chars()
                            .filter(|c| c.is_digit(10) || *c == '.' || *c == ',')
                            .collect();
                        numeric_part.parse::<f32>().ok()
                    })
                    .collect::<Vec<f32>>();

                if values.len() == 3 {
                    monthly = Some(MonthlyRates {
                        worst: values[0],
                        best: values[1],
                        average: values[2],
                    });
                    break;
                }
            }
        }

        (yearly, monthly)
    }; // <<< document и селекторы дропаются ЗДЕСЬ

    // --- CSV fallback ---
    let mut monthly = monthly_from_html;

    if monthly.is_none() {
        let year_int: i32 = year.parse().map_err(|_| "Invalid year format")?;
        let url = format!(
            "https://www.exchange-rates.org/HistoryExchangeRatesReportDownload.aspx?base_iso_code={}&iso_code={}",
            from.to_lowercase(),
            to.to_lowercase()
        );

        let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
        let cursor = Cursor::new(bytes);

        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(cursor);

        let mut rates = vec![];

        for result in rdr.records() {
            let record = result.map_err(|e| e.to_string())?;

            if record.len() < 4 {
                continue;
            }

            let date_str = record.get(0).ok_or("Missing date field")?;
            let rate_str = record.get(1).ok_or("Missing rate field")?;

            let date_part = date_str.split_whitespace().next().ok_or("Invalid date format")?;
            let parsed_date = NaiveDate::parse_from_str(date_part, "%m/%d/%Y")
                .map_err(|_| "Invalid date format")?;

            if parsed_date.year() == year_int && parsed_date.format("%B").to_string() == month {
                let rate: f32 = rate_str.parse().map_err(|_| "Failed to parse rate")?;
                rates.push(rate);
            }
        }

        if !rates.is_empty() {
            monthly = Some(MonthlyRates {
                worst: rates.iter().cloned().fold(f32::MAX, f32::min),
                best: rates.iter().cloned().fold(f32::MIN, f32::max),
                average: rates.iter().sum::<f32>() / rates.len() as f32,
            });
        }
    }

    Ok(HistoricalRates {
        yearly: Some(yearly),
        monthly,
    })
}

// ========== Current Rate ==========
#[tauri::command]
async fn get_current_exchange_rate(
    from: String,
    to: String,
) -> Result<CurrentRate, String> {
    let url = format!(
        "https://www.exchange-rates.org/api/v2/rates/lookup?isoTo={}&isoFrom={}&amount=1&pageCode=Home",
        to.to_uppercase(),
        from.to_uppercase()
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .json::<ApiResponse>()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(error) = response.error_message {
        return Err(error);
    }

    Ok(CurrentRate {
        rate: response.rate.unwrap_or(0.0),
        from: from.to_uppercase(),
        to: to.to_uppercase(),
        date: response.date.unwrap_or_else(|| "".into()),
    })
}

// ========== Currency Codes ==========
#[tauri::command]
async fn get_available_currency_codes() -> Result<Vec<CurrencyInfo>, String> {
    let html = reqwest::get("https://www.exchange-rates.org/")
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse("#currencyList option").unwrap();
    let regex = Regex::new(r"^(.*?)\s+\((\w+)\)$").unwrap();

    let mut currencies = Vec::new();

    for element in document.select(&selector) {
        if let Some(text) = element.text().next() {
            if let Some(caps) = regex.captures(text) {
                currencies.push(CurrencyInfo {
                    name: caps[1].trim().to_string(),
                    code: caps[2].to_uppercase(),
                });
            }
        }
    }

    Ok(currencies)
}

// ========== TAURI RUN ==========
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            parse_exchange_rates,
            get_current_exchange_rate,
            get_available_currency_codes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}