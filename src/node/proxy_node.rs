use std::ops::Deref;
use std::ptr::NonNull;

use serde_json::Value;
use std::fmt::{Debug, Display};
use crate::error::Error;
use crate::ast::RbatisAST;
use crate::node::node_type::NodeType;
use rexpr::runtime::RExprRuntime;
use std::sync::Arc;

///CustomNode Generate,you can custom py lang parse
pub trait NodeFactory: Send + Sync + Debug {
    ///generate return an Option<CustomNode>,if return None,parser will be skip this build
    fn try_new(
        &self,
        express: &str,
        child_nodes: Vec<NodeType>,
    ) -> Result<Option<ProxyNode>, Error>;
}

#[derive(Clone, Debug)]
pub struct ProxyNode {
    pub childs: Vec<NodeType>,
    ptr: Arc<Box<dyn RbatisAST>>,
}

impl ProxyNode {
    pub fn from<T>(body: T, childs: Vec<NodeType>) -> Self
    where
        T: RbatisAST + 'static,
    {
        Self {
            childs,
            ptr: Arc::new(Box::new(body)),
        }
    }
}

impl RbatisAST for ProxyNode {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "proxy"
    }

    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        self.ptr
            .deref()
            .eval(convert, env, engine, arg_array, arg_sql)
    }
}
