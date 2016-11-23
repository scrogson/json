#[macro_use]
extern crate nom;

use nom::{digit, alphanumeric};

use std::str::{self, FromStr};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}


named!(unsigned_float <f64>, map_res!(
    map_res!(
        recognize!(
            alt_complete!(
                delimited!(digit, tag!("."), opt!(complete!(digit))) |
                delimited!(opt!(digit), tag!("."), digit)            |
                digit
            )
        ),
        str::from_utf8
    ),
    FromStr::from_str
));

named!(float<f64>, map!(
    pair!(
        opt!(alt!(tag!("+") | tag!("-"))),
        unsigned_float
    ),
    |(sign, value): (Option<&[u8]>, f64)| {
        sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f64) } else { None }).unwrap_or(1f64) * value
    }
));

named!(string<&str>,
    delimited!(
        tag!("\""),
        map_res!(escaped!(call!(alphanumeric), '\\', is_a!("\"n\\")), str::from_utf8),
        tag!("\"")
    )
);


named!(array< Vec<JsonValue> >,
    ws!(
        delimited!(
            tag!("["),
            separated_list!(tag!(","), value),
            tag!("]")
        )
    )
);

named!(key_value<(&str,JsonValue)>,
    ws!(
        separated_pair!(
            string,
            tag!(":"),
            value
        )
    )
);


named!(hash< HashMap<String,JsonValue> >,
   ws!(
       map!(
           delimited!(
               tag!("{"),
               separated_list!(tag!(","), key_value),
               tag!("}")
           ),
           |tuple_vec| {
               let mut h: HashMap<String, JsonValue> = HashMap::new();
               for (k, v) in tuple_vec {
                   h.insert(String::from(k), v);
               }
               h
           }
       )
    )
);

named!(value<JsonValue>,
    ws!(
        alt!(
            hash   => { |v| JsonValue::Object(v) }               |
            array  => { |v| JsonValue::Array(v) }                |
            string => { |v| JsonValue::String(String::from(v)) } |
            float  => { |v| JsonValue::Number(v) }
        )
    )
);

#[test]
fn it_works() {
    let test = b"{ \"a\"\t: 42, \"b\": \"x\"}";

    println!("{:?}", value(&test[..]));
    //assert!(false);"
}
