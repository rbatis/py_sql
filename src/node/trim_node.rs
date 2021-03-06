use serde_json::Value;

use crate::ast::RbatisAST;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use rexpr::runtime::RExprRuntime;

#[derive(Clone, Debug)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub trim: String,
}

impl TrimNode {
    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        let express = express[Self::name().len()..].trim();
        if express.starts_with("'") && express.ends_with("'") {
            let express = express[1..express.len() - 1].trim();
            return Ok(TrimNode {
                childs: childs,
                trim: express.to_string(),
            });
        } else {
            return Err(crate::error::Error::from(format!("[rbatis] express trim value must be string value, for example:  trim 'value',error express: {}", source)));
        }
    }
}

impl RbatisAST for TrimNode {
    fn name() -> &'static str {
        "trim"
    }
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        let mut child_sql = String::new();
        do_child_nodes(
            convert,
            &self.childs,
            env,
            engine,
            arg_array,
            &mut child_sql,
        )?;
        let mut result = child_sql.as_str().trim();
        if !self.trim.is_empty() {
            let splits: Vec<&str> = self.trim.split("|").collect();
            for item in splits {
                result = result.trim_start_matches(item);
                result = result.trim_end_matches(item);
            }
        }
        arg_sql.push_str(result);
        return Result::Ok(serde_json::Value::Null);
    }
}
