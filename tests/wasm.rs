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
    get_date_str(2020, 1).unwrap()
  );
  assert_eq!(
    "21.12.2020-25.12.2020".to_string(),
    get_date_str(2020, 52).unwrap()
  );
  assert_eq!(
    "28.12.2020-01.01.2021".to_string(),
    get_date_str(2020, 53).unwrap()
  );
  assert_eq!(
    "04.01.2021-08.01.2021".to_string(),
    get_date_str(2021, 1).unwrap()
  );
  assert_eq!(
    "Invalid year/week".to_string(),
    get_date_str(2021, 53).unwrap_err()
  );
}
