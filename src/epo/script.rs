use super::date;
use boa_engine::object::JsObject;
use boa_engine::property::Attribute;
use boa_engine::property::PropertyKey;
use boa_engine::Context;
use boa_engine::JsValue;

pub fn eval(js_code: &str) -> Result<Vec<i64>, String> {
    let mut context = Context::default();

    context.register_global_property("now", date::current_epoch(), Attribute::all());

    let r = context.eval(js_code);

    if let Ok(val) = r {
        match &val {
            JsValue::Integer(i) => {
                return Ok(vec![*i as i64]);
            }
            JsValue::Rational(f) => {
                return Ok(vec![*f as i64]);
            }
            JsValue::Object(o) => {
                let values = to_int_values(o, &mut context)?;
                return Ok(values);
            }
            _ => {}
        }
    }

    Err(format!("Invalid JavaScript: {}", js_code))
}

fn to_int_values(obj: &JsObject, context: &mut Context) -> Result<Vec<i64>, String> {
    let mut values: Vec<i64> = Vec::new();

    let length_value = obj_get(obj, "length", context)?;

    if let JsValue::Integer(length) = length_value {
        for i in 0..length {
            let val = obj_get(obj, i, context)?;
            if let JsValue::Integer(i) = val {
                values.push(i as i64);
            } else if let JsValue::Rational(f) = val {
                values.push(f as i64);
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
