use crate::ast::RbatisAST;
use crate::node::bind_node::BindNode;
use crate::node::choose_node::ChooseNode;
use crate::node::foreach_node::ForEachNode;
use crate::node::if_node::IfNode;
use crate::node::node_type::NodeType::NWhen;
use crate::node::otherwise_node::OtherwiseNode;
use crate::node::set_node::SetNode;
use crate::node::string_node::StringNode;
use crate::node::trim_node::TrimNode;
use crate::node::when_node::WhenNode;
use crate::node::where_node::WhereNode;
use rexpr::runtime::RExprRuntime;
use serde_json::{json, Value};
use std::collections::HashMap;

use super::node_type::NodeType;

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
