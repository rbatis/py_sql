use crate::ast::RbatisAST;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use serde_json::{json, Value};
#[derive(Clone, Debug)]
pub struct ForEachNode {
    pub childs: Vec<NodeType>,
    pub collection: String,
    pub index: String,
    pub item: String,
    pub get_collection_func: Node,
}

impl ForEachNode {
    pub fn from(
        runtime: &RExprRuntime,
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        if !express.contains("in ") {
            return Err(crate::error::Error::from(
                "[rbatis] parser express fail:".to_string() + source,
            ));
        }
        let express = express[Self::name().len()..].trim();
        let in_index = express.find("in ").unwrap();
        let col = express[in_index + "in ".len()..].trim();
        let mut item = express[..in_index].trim();
        let mut index = "";
        if item.contains(",") {
            let items: Vec<&str> = item.split(",").collect();
            if items.len() != 2 {
                return Err(crate::error::Error::from(format!(
                    "[rbatis][py] parse fail 'for ,' must be 'for arg1,arg2 in ...',value:'{}'",
                    source
                )));
            }
            index = items[0];
            item = items[1];
        }
        return Ok(ForEachNode {
            childs: childs,
            collection: col.to_string(),
            index: index.to_string(),
            item: item.to_string(),
            get_collection_func: runtime.parse(col)?,
        });
    }
}

impl RbatisAST for ForEachNode {
    fn name() -> &'static str {
        "for"
    }
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        let collection_value = self.get_collection_func.eval(env)?;
        if collection_value.is_null() {
            return Result::Ok(serde_json::Value::Null);
        }
        if collection_value.is_array() {
            let collection = collection_value.as_array().unwrap();
            let mut index = -1;
            for item in collection {
                index = index + 1;
                env[&self.item] = item.clone();
                env[&self.index] = serde_json::Value::Number(serde_json::Number::from(index));
                do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql)?;
                match env {
                    serde_json::Value::Object(obj) => {
                        obj.remove(&self.item);
                        obj.remove(&self.index);
                    }
                    _ => {}
                }
            }
            return Result::Ok(serde_json::Value::Null);
        } else if collection_value.is_object() {
            let collection = collection_value.as_object().unwrap();
            let mut index = -1;
            for (key, item) in collection {
                index = index + 1;
                env[&self.item] = item.clone();
                env[&self.index] = serde_json::Value::String(key.to_string());
                do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql)?;
                match env {
                    serde_json::Value::Object(obj) => {
                        obj.remove(&self.item);
                        obj.remove(&self.index);
                    }
                    _ => {}
                }
            }
            return Result::Ok(serde_json::Value::Null);
        } else {
            return Result::Err(crate::error::Error::from(
                "[rbatis] collection name:".to_owned()
                    + self.collection.as_str()
                    + " is not a array or object/map value!",
            ));
        }
    }
}
