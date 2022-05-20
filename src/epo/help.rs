const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> String {
  format!("epo {}", VERSION)
}

pub fn usage(epoch: i64) -> String {
  let version = version();
  let epoch1 = epoch + 86400;
  let epoch2 = epoch + 86400 * 2;
  format!(
    "{version}
Usage: epo [options...]
  -p Print as plain text.
  -h Display help.

Examples:
  # Basics: Specify epochs
  epo {epoch}
  epo {epoch} {epoch1} {epoch2}
  epo now now+86400 \"now+86400*2\"
  
  # Specify epochs and timezones
  epo 0 greenwich
  epo 0 -1000 -0500 +0000 +0500 +0010
  epo 0 -10 -5 +0 +5 +10
  epo now honolulu new_york london karachi sydney

  # Use JavaScript
  epo \"[0,1,2,3,4,5,6].map(a=>now+86400*a)\" honolulu new_york london karachi sydney
  epo \"[...Array(365).keys()].map(a=>now+86400*a)\" denver phoenix los_angeles 

  # Specify ISO 8601 date formats
  epo 1970-01-01 greenwich
  epo 2022-06-15T12:30:45 los_angeles new_york
  epo 2022-04-01 2022-05-01T12:30 2022-06-15T12:30:45 tokyo hawaii gmt
  "
  )
}
