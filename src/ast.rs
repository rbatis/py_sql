use rexpr::runtime::RExprRuntime;
use serde_json::Value;
use std::fmt::Debug;

/// Abstract syntax tree node
pub trait RbatisAST: Send + Sync + Debug {
    fn name() -> &'static str
    where
        Self: Sized;
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_result: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error>;
}
