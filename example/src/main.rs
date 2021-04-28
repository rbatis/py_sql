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

#[cfg(test)]
mod test{
    use py_sql::RExprRuntime;
    use py_sql::py_sql::PyRuntime;
    use crate::DriverType;


    #[test]
    fn test_bench(){
        let expr_runtime = RExprRuntime::new();
        let runtime = PyRuntime::new(vec![]);
        let (sql, args) = runtime
            .eval(
                &DriverType {},
                "select * from biz_activity
                     if name != null:
                        and name like pattern
                     trim 'and':
                     and id!=null and",
                &mut serde_json::json!({}),
                &expr_runtime,
            )
            .unwrap();
        println!("sql:{}", sql);
        println!("args:{:?}", args);

        let mut arg=serde_json::json!({
             "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}],
        "g":true,
        "name":"ss",
        "startTime":"2019",
        "endTime":"2020"
        });

        bench!(1000000,{
            runtime
            .eval(
                &DriverType {},
                "select * from biz_activity
                     if name != null:
                        and name like pattern
                     trim 'and':
                     and id!=null and",
                &mut arg,
                &expr_runtime,
            )
            .unwrap();
        });
    }
}