use crate::node::node_type::NodeType;
use serde_json::Value;
use rexpr::runtime::RExprRuntime;
use crate::ast::RbatisAST;

//执行子所有节点
pub(crate) fn do_child_nodes(
    convert: &dyn crate::StringConvert,
    child_nodes: &Vec<NodeType>,
    env: &mut Value,
    engine: &RExprRuntime,
    arg_array: &mut Vec<Value>,
    arg_sql: &mut String,
) -> Result<serde_json::Value, crate::error::Error> {
    for item in child_nodes {
        item.eval(convert, env, engine, arg_array, arg_sql)?;
    }
    return Result::Ok(serde_json::Value::Null);
}
