use crate::ast::RbatisAST;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
    pub test_func: Node,
}

impl IfNode {
    pub fn from(
        runtime: &RExprRuntime,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        let express = express[Self::name().len()..].trim();
        return Ok(IfNode {
            childs: childs,
            test: express.to_string(),
            test_func: runtime.parse(express)?,
        });
    }
}

impl RbatisAST for IfNode {
    fn name() -> &'static str {
        "if"
    }
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        let result = self.test_func.eval(env)?;
        if !result.is_boolean() {
            return Result::Err(crate::error::Error::from(
                "[rbatis] express:'".to_owned()
                    + self.test.as_str()
                    + "' is not return bool value!",
            ));
        }
        if result.as_bool().unwrap_or(false) {
            return do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql);
        }
        return Result::Ok(serde_json::Value::Null);
    }
}
