#[macro_use]
extern crate py_sql;

use py_sql::py_sql::PyRuntime;
use py_sql::{RExprRuntime, StringConvert};

pub struct DriverType {}

impl StringConvert for DriverType {
    fn convert(&self, index: usize) -> String {
        "?".to_string()
    }
}

fn main() {
    let expr_runtime = RExprRuntime::new();
    let runtime = PyRuntime::new(vec![]);
    let (sql, args) = runtime
        .eval(
            &DriverType {},
            "select * from table where
                     if 1 == 1:
                        1 = 1",
            &mut serde_json::json!({}),
            &expr_runtime,
        )
        .unwrap();
    println!("sql:{}", sql);
    println!("args:{:?}", args);
}


#[expr("@.s.string()+'f'")]
pub fn add(arg: &serde_json::Value) -> py_sql::error::Result<serde_json::Value> {}


#[test]
fn test_rexpr() {
    let arg = serde_json::json!({
        "s":"sss"
    });
    println!("{}", add(&arg).unwrap());
}
