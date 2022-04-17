use chrono::{FixedOffset, TimeZone, Utc};

pub fn to_date_str(epoch_sec: i64, offset: i32) -> String {
  let dt = Utc
    .timestamp(epoch_sec, 0)
    .with_timezone(&FixedOffset::east(offset * 3600));

  return dt.format("%Y-%m-%dT%H:%M:%S%z").to_string();
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn internal() {
    assert_eq!("1970-01-01T00:00:00+0000", to_date_str(0, 0));
    assert_eq!("1970-01-01T09:00:00+0900", to_date_str(0, 9));
    assert_eq!("2022-04-17T21:09:49+0900", to_date_str(1650197389, 9));
  }
}
