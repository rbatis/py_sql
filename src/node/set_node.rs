use crate::ast::RbatisAST;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use rexpr::runtime::RExprRuntime;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl SetNode {
    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        //let trim_x = express[Self::name().len()..].trim();
        return Ok(SetNode { childs });
    }
}

impl RbatisAST for SetNode {
    fn name() -> &'static str {
        "set"
    }
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql);
    }
}
