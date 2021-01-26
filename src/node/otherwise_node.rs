use serde_json::{Value};

use crate::error::Error;
use crate::ast::RbatisAST;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use rexpr::runtime::RExprRuntime;

#[derive(Clone, Debug)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl OtherwiseNode {
    pub fn def_name() -> &'static str {
        "_"
    }

    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        let source = source.trim();
        if source.starts_with(Self::def_name()) {
            return Ok(OtherwiseNode { childs });
        } else if source.starts_with(Self::name()) {
            return Ok(OtherwiseNode { childs });
        }
        return Err(Error::from(
            "[rbaits] OtherwiseNode must start with '_:' or 'otherwise:'",
        ));
    }
}

impl RbatisAST for OtherwiseNode {
    fn name() -> &'static str {
        "otherwise"
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
