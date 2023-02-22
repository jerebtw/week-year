use chrono::{NaiveDate, Weekday};
use wasm_bindgen::prelude::*;

/// Returns true if the given year has a 53rd week.
#[wasm_bindgen(js_name = "has53Weeks")]
pub fn has_53_weeks(year: i32 /*** Test */) -> bool {
  NaiveDate::from_isoywd_opt(year, 53, Weekday::Sun).is_some()
}

/// Returns a string in the format "dd.mm.yyyy-dd.mm.yyyy" for the given year and week.
/// The week is assumed to be a Monday-Friday week.
/// Returns an error if the year/week is invalid.
#[wasm_bindgen(js_name = "getDateStr")]
pub fn get_date_str(year: i32, week: u32) -> Result<String, String> {
  let mon =
    NaiveDate::from_isoywd_opt(year, week, Weekday::Mon).ok_or_else(|| "Invalid year/week")?;
  let fri =
    NaiveDate::from_isoywd_opt(year, week, Weekday::Fri).ok_or_else(|| "Invalid year/week")?;
  return Ok(format!(
    "{}-{}",
    mon.format("%d.%m.%Y"),
    fri.format("%d.%m.%Y")
  ));
}
