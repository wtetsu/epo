pub fn usage(epoch: i64) -> String {
  let epoch1 = epoch + 86400;
  let epoch2 = epoch + 86400 * 2;
  format!(
    "Usage: epo [options...]
  -p Print as plain text.
  -h Display help.

Examples:
  epo {epoch}

  epo {epoch} {epoch1} {epoch2}
  epo now now+86400 \"now+86400*2\"

  epo 0
  epo 0 -10 -5 +0 +5 +10

  epo now honolulu new_york london karachi sydney
  epo \"[0,1,2,3,4,5,6].map(a=>now+86400*a)\" honolulu new_york london karachi sydney
  epo \"[...Array(365).keys()].map(a=>now+86400*a)\" denver phoenix los_angeles 
  "
  )
}
