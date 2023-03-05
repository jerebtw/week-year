use time::{
  format_description, Date,
  Weekday::{Friday, Monday},
};
use wasm_bindgen::prelude::*;

/// Returns true if the given year has a 53rd week.
#[wasm_bindgen(js_name = "has53Weeks")]
pub fn has_53_weeks(year: i32) -> bool {
  time::Date::from_iso_week_date(year, 53, Monday).is_ok()
}

/// Returns a string in the format "dd.mm.yyyy-dd.mm.yyyy" (Or a custom format if provided. https://time-rs.github.io/book/api/format-description.html) for the given year and week.
/// The week is assumed to be a Monday-Friday week.
/// Returns an error if the year/week is invalid.
#[wasm_bindgen(js_name = "getDateStr")]
pub fn get_date_str(year: i32, week: u8, format: Option<String>) -> Result<String, String> {
  let mon = Date::from_iso_week_date(year, week, Monday).map_err(|_| "Invalid year/week")?;
  let fri = Date::from_iso_week_date(year, week, Friday).map_err(|_| "Invalid year/week")?;

  let custom_format = format.unwrap_or_else(|| "[day].[month].[year]".to_string());
  let format = format_description::parse(&custom_format).unwrap();
  Ok(
    format!(
      "{}-{}",
      mon.format(&format).unwrap(),
      fri.format(&format).unwrap()
    )
    .trim()
    .to_string(),
  )
}
