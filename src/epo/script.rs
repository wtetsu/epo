use super::date;
use boa_engine::object::JsObject;
use boa_engine::property::{Attribute, PropertyKey};
use boa_engine::{Context, JsValue};

const MAX_EPOCH: i64 = 8210298412799 - 86400;
const MIN_EPOCH: i64 = -MAX_EPOCH;

const DEFINE_FUNCTIONS: &str = "
const range = (start, end) => {
  let s, e;
  if (end === undefined) {
    s = 0;
    e = start;
  } else {
    s = start;
    e = end;
  }
  const r = [];
  const inc = s < e ? 1 : -1;
  while (s !== e) {
    r.push(s);
    s += inc;
  }
  return r;
};
";

pub fn eval(js_code: &str) -> Result<Vec<i64>, String> {
    let mut context = Context::default();

    context.register_global_property("now", date::current_epoch(), Attribute::all());
    let r = context.eval(format!("{}{}", DEFINE_FUNCTIONS, js_code).as_str());

    match r {
        Ok(js_value) => match &js_value {
            JsValue::Integer(i) => {
                return Ok(vec![*i as i64]);
            }
            JsValue::Rational(f) => {
                let i = *f as i64;
                if i > MAX_EPOCH {
                    return Err(format!("epoch value is too large: {}", f));
                }
                if i < MIN_EPOCH {
                    return Err(format!("epoch value is too small: {}", f));
                }
                return Ok(vec![i]);
            }
            JsValue::Object(o) => {
                let values = to_epoch_values(o, &mut context)?;
                return Ok(values);
            }
            _ => {}
        },
        Err(err) => {
            eprintln!("Uncaught {}", err.display());
        }
    }

    Err(format!("Invalid JavaScript code: {}", js_code))
}

fn to_epoch_values(obj: &JsObject, context: &mut Context) -> Result<Vec<i64>, String> {
    let mut values: Vec<i64> = Vec::new();

    let length_value = obj_get(obj, "length", context)?;

    if let JsValue::Integer(length) = length_value {
        for i in 0..length {
            let val = obj_get(obj, i, context)?;
            if let JsValue::Integer(i) = val {
                values.push(i as i64);
            } else if let JsValue::Rational(f) = val {
                let i = f as i64;
                if i > MAX_EPOCH {
                    return Err(format!("epoch value is too large: {}", f));
                }
                if i < MIN_EPOCH {
                    return Err(format!("epoch value is too small: {}", f));
                }
                values.push(i);
            } else {
                return Err("value is not integer".to_string());
            }
        }
    } else {
        return Err("length is not integer".to_string());
    }
    Ok(values)
}

fn obj_get<K>(obj: &JsObject, key: K, context: &mut Context) -> Result<JsValue, String>
where
    K: Into<PropertyKey>,
{
    let r = obj.get(key, context);
    match r {
        Ok(v) => Ok(v),
        Err(_) => Err("No such property".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_number() {
        assert_eq!(vec!(0), eval("0").unwrap());
        assert_eq!(vec!(1651256673), eval("1651256673").unwrap());
        assert_eq!(vec!(8210298326399), eval("8210298326399").unwrap());
    }
    #[test]
    fn test_numbers() {
        assert_eq!(vec!(0, 1, 2), eval("[0,1,2]").unwrap());
        assert_eq!(
            vec!(-8210298326398, -8210298326399),
            eval("[-8210298326398,-8210298326399]").unwrap()
        );
        assert_eq!(vec!(8210298326398, 8210298326399), eval("[8210298326398,8210298326399]").unwrap());

        assert_eq!(vec!(0, 1, 2), eval("[0.9,1.1,2.9]").unwrap());
    }

    #[test]
    fn test_scripts() {
        assert_eq!(vec!(997002999), eval("999*999*999").unwrap());

        assert_eq!(
            vec!(1651256673, 1651256674, 1651256675),
            eval("[0,1,2].map(a=>1651256673+a)").unwrap()
        );

        assert_eq!(
            vec!(1651256673, 1651256674, 1651256675, 1651256676, 1651256677),
            eval("[...Array(5).keys()].map(a=>1651256673+a)").unwrap()
        );
    }

    #[test]
    fn test_range() {
        assert_eq!(
            vec!(1651256673, 1651256674, 1651256675),
            eval("range(0, 3).map(a=>1651256673+a)").unwrap()
        );
        assert_eq!(
            vec!(1651256673, 1651256674, 1651256675),
            eval("range(3).map(a=>1651256673+a)").unwrap()
        );
        assert_eq!(
            vec!(1651256673, 1651256672, 1651256671),
            eval("range(0, -3).map(a=>1651256673+a)").unwrap()
        );
        assert_eq!(
            vec!(1651256673, 1651256672, 1651256671),
            eval("range(-3).map(a=>1651256673+a)").unwrap()
        );

        assert_eq!(0_usize, eval("range(100, 100).map(a=>1651256673+a)").unwrap().len());
        assert_eq!(0_usize, eval("range(-100, -100).map(a=>1651256673+a)").unwrap().len());
    }

    #[test]
    fn test_errors() {
        assert!(eval("8210298326400").is_err());
        assert!(eval("-8210298326400").is_err());
    }
}
