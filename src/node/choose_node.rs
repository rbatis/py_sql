use crate::ast::RbatisAST;
use crate::node::node_type::NodeType;
use rexpr::runtime::RExprRuntime;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ChooseNode {
    pub when_nodes: Option<Vec<NodeType>>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl ChooseNode {
    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        //let express = express["choose".len()..].trim();
        let mut node = ChooseNode {
            when_nodes: None,
            otherwise_node: None,
        };
        for x in childs {
            match x {
                NodeType::NWhen(_) => {
                    if node.when_nodes.is_none() {
                        node.when_nodes = Some(vec![]);
                    }
                    node.when_nodes.as_mut().unwrap().push(x);
                }
                NodeType::NOtherwise(_) => {
                    node.otherwise_node = Some(Box::new(x));
                }
                _ => {
                    return Err(crate::error::Error::from("[rbatis] parser node fail,choose node' child must be when and otherwise nodes!".to_string()));
                }
            }
        }
        return Ok(node);
    }
}

impl RbatisAST for ChooseNode {
    fn name() -> &'static str {
        "choose"
    }

    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        if self.when_nodes.is_none() == false {
            let mut when_index = 0;
            for item in self.when_nodes.as_ref().unwrap() {
                let v = item.eval(convert, env, engine, arg_array, arg_sql)?;
                if v.as_bool().unwrap_or(false) == true {
                    return Result::Ok(serde_json::Value::Null);
                }
                when_index += 1;
            }
        }
        match &self.otherwise_node {
            Some(other) => {
                return other.eval(convert, env, engine, arg_array, arg_sql);
            }
            _ => {}
        }
        return Result::Ok(serde_json::Value::Null);
    }
}
