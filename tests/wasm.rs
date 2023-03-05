use wasm_bindgen_test::*;
use week_year::{get_date_str, has_53_weeks};

#[wasm_bindgen_test]
fn test_has_53_weeks() {
  assert_eq!(true, has_53_weeks(2020));
  assert_eq!(false, has_53_weeks(2021));
  assert_eq!(false, has_53_weeks(2022));
  assert_eq!(false, has_53_weeks(2023));
  assert_eq!(false, has_53_weeks(2024));
  assert_eq!(false, has_53_weeks(2025));
  assert_eq!(true, has_53_weeks(2026));
  assert_eq!(false, has_53_weeks(2027));
}

#[wasm_bindgen_test]
fn test_get_date_str() {
  assert_eq!(
    "30.12.2019-03.01.2020".to_string(),
    get_date_str(2020, 1, None).unwrap()
  );
  assert_eq!(
    "21.12.2020-25.12.2020".to_string(),
    get_date_str(2020, 52, None).unwrap()
  );
  assert_eq!(
    "28.12.2020-01.01.2021".to_string(),
    get_date_str(2020, 53, None).unwrap()
  );
  assert_eq!(
    "04.01.2021-08.01.2021".to_string(),
    get_date_str(2021, 1, None).unwrap()
  );
  assert_eq!(
    "Invalid year/week".to_string(),
    get_date_str(2021, 53, None).unwrap_err()
  );
}

#[wasm_bindgen_test]
fn test_get_date_str_custom_format() {
  let custom_format = Some("[Year]-[Month]-[Day]".to_string());
  assert_eq!(
    "2019-12-30-2020-01-03".to_string(),
    get_date_str(2020, 1, custom_format.clone()).unwrap()
  );
  assert_eq!(
    "2020-12-21-2020-12-25".to_string(),
    get_date_str(2020, 52, custom_format.clone()).unwrap()
  );
  assert_eq!(
    "2020-12-28-2021-01-01".to_string(),
    get_date_str(2020, 53, custom_format.clone()).unwrap()
  );
  assert_eq!(
    "2021-01-04-2021-01-08".to_string(),
    get_date_str(2021, 1, custom_format.clone()).unwrap()
  );
  assert_eq!(
    "Invalid year/week".to_string(),
    get_date_str(2021, 53, custom_format).unwrap_err()
  );
}
