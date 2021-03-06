use crate::ast::RbatisAST;
use crate::error::Error;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use rexpr;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct PrintNode {
    pub express: String,
    pub childs: Vec<NodeType>,
    pub express_func: Node,
}

impl PrintNode {
    pub fn from(
        runtime: &RExprRuntime,
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        let source = source.trim();
        if express.starts_with(Self::name()) {
            let express = express[Self::name().len()..].trim();
            return Ok(PrintNode {
                express: express.to_string(),
                childs: childs,
                express_func: runtime.parse(express)?,
            });
        } else {
            return Err(Error::from(
                "[rbaits] PrintNode must start with 'print arg:' or 'print childs:'",
            ));
        }
    }
}

impl RbatisAST for PrintNode {
    fn name() -> &'static str {
        "print"
    }
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql)?;
        if !env.is_object() {
            return Err(Error::from("[rbatis] print node arg must be json object! you can use empty json for example: {}"));
        }
        if self.express.contains("sql") {
            env["sql"] = json!(arg_sql);
        }
        if self.express.contains("arg_array") {
            env["arg_array"] = json!(arg_array);
        }
        let r = self.express_func.eval(env)?;
        println!("{}: {}", self.express, r);
        return Ok(serde_json::Value::Null);
    }
}
