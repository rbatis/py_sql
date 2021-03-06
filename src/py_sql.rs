use dashmap::DashMap;
use std::collections::HashMap;

use rexpr::runtime::RExprRuntime;

use serde_json::Value;

use crate::ast::RbatisAST;
use crate::error::Error;
use crate::node::bind_node::BindNode;
use crate::node::choose_node::ChooseNode;
use crate::node::foreach_node::ForEachNode;
use crate::node::if_node::IfNode;
use crate::node::node::do_child_nodes;
use crate::node::node_type::NodeType;
use crate::node::otherwise_node::OtherwiseNode;
use crate::node::print_node::PrintNode;
use crate::node::proxy_node::NodeFactory;
use crate::node::set_node::SetNode;
use crate::node::string_node::StringNode;
use crate::node::trim_node::TrimNode;
use crate::node::when_node::WhenNode;
use crate::node::where_node::WhereNode;

/// Py lang,make sure Send+Sync
#[derive(Debug)]
pub struct PyRuntime {
    pub cache: DashMap<String, Vec<NodeType>>,
    pub generate: Vec<Box<dyn NodeFactory>>,
}

impl PyRuntime {
    pub fn new(generate: Vec<Box<dyn NodeFactory>>) -> Self {
        Self {
            cache: Default::default(),
            generate: generate,
        }
    }
    ///eval with cache
    pub fn eval(
        &self,
        driver_type: &dyn crate::StringConvert,
        py_sql: &str,
        env: &mut Value,
        engine: &RExprRuntime,
    ) -> Result<(String, Vec<serde_json::Value>), Error> {
        if !env.is_object() {
            return Result::Err(Error::from(
                "[rbatis] py_sql Requires that the parameter be an json object!",
            ));
        }
        let mut sql = String::new();
        let mut arg_array = vec![];
        let cache_value = self.cache.get(py_sql);
        match cache_value {
            Some(v) => {
                do_child_nodes(driver_type, &v, env, engine, &mut arg_array, &mut sql)?;
            }
            _ => {
                let nodes = Self::parse(engine, py_sql, &self.generate)?;
                do_child_nodes(driver_type, &nodes, env, engine, &mut arg_array, &mut sql)?;
                self.cache.insert(py_sql.to_string(), nodes);
            }
        }
        sql = sql.trim().to_string();
        return Ok((sql, arg_array));
    }

    pub fn add_gen<T>(&mut self, arg: T)
    where
        T: NodeFactory + 'static,
    {
        self.generate.push(Box::new(arg));
    }

    /// parser py string data
    pub fn parse(
        runtime: &RExprRuntime,
        arg: &str,
        generates: &Vec<Box<dyn NodeFactory>>,
    ) -> Result<Vec<NodeType>, crate::error::Error> {
        let line_space_map = PyRuntime::create_line_space_map(arg);
        let mut main_node = vec![];
        let ls = arg.lines();
        let mut space = -1;
        let mut line = -1;
        let mut skip = -1;
        for x in ls {
            line += 1;
            if x.is_empty() || (skip != -1 && line <= skip) {
                continue;
            }
            let count_index = *line_space_map.get(&line).unwrap();
            if space == -1 {
                space = count_index;
            }
            let (child_str, do_skip) =
                PyRuntime::find_child_str(line, count_index, arg, &line_space_map);
            if do_skip != -1 && do_skip >= skip {
                skip = do_skip;
            }
            let parserd;
            if !child_str.is_empty() {
                parserd = PyRuntime::parse(runtime, child_str.as_str(), generates)?;
            } else {
                parserd = vec![];
            }
            PyRuntime::parse_node(
                runtime,
                generates,
                &mut main_node,
                x,
                *line_space_map.get(&line).unwrap() as usize,
                parserd,
            )?;
        }
        return Ok(main_node);
    }

    fn parse_trim_node(
        runtime: &RExprRuntime,
        factorys: &Vec<Box<dyn NodeFactory>>,
        trim_express: &str,
        source_str: &str,
        childs: Vec<NodeType>,
    ) -> Result<NodeType, crate::error::Error> {
        if trim_express.starts_with(IfNode::name()) {
            return Ok(NodeType::NIf(IfNode::from(runtime, trim_express, childs)?));
        } else if trim_express.starts_with(ForEachNode::name()) {
            return Ok(NodeType::NForEach(ForEachNode::from(
                runtime,
                source_str,
                &trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(TrimNode::name()) {
            return Ok(NodeType::NTrim(TrimNode::from(
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(ChooseNode::name()) {
            return Ok(NodeType::NChoose(ChooseNode::from(
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(OtherwiseNode::def_name())
            || trim_express.starts_with(OtherwiseNode::name())
        {
            return Ok(NodeType::NOtherwise(OtherwiseNode::from(
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(WhenNode::name()) {
            return Ok(NodeType::NWhen(WhenNode::from(
                runtime,
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(BindNode::def_name())
            || trim_express.starts_with(BindNode::name())
        {
            return Ok(NodeType::NBind(BindNode::from(
                runtime,
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(SetNode::name()) {
            return Ok(NodeType::NSet(SetNode::from(
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(WhereNode::name()) {
            return Ok(NodeType::NWhere(WhereNode::from(
                source_str,
                trim_express,
                childs,
            )?));
        } else if trim_express.starts_with(PrintNode::name()) {
            return Ok(NodeType::NPrint(PrintNode::from(
                runtime,
                source_str,
                trim_express,
                childs,
            )?));
        } else {
            for f in factorys {
                let gen = f.try_new(trim_express, childs.clone())?;
                if gen.is_some() {
                    return Ok(NodeType::NCustom(gen.unwrap()));
                }
            }
            // unkonw tag
            return Err(crate::error::Error::from(
                "[rbatis] unknow tag: ".to_string() + source_str,
            ));
        }
    }

    fn parse_node(
        runtime: &RExprRuntime,
        generates: &Vec<Box<dyn NodeFactory>>,
        main_node: &mut Vec<NodeType>,
        x: &str,
        space: usize,
        mut childs: Vec<NodeType>,
    ) -> Result<(), crate::error::Error> {
        let mut trim_x = x.trim();
        if trim_x.starts_with("//") {
            return Ok(());
        }
        if trim_x.ends_with(":") {
            trim_x = trim_x[0..trim_x.len() - 1].trim();
            if trim_x.contains(": ") {
                let vecs: Vec<&str> = trim_x.split(": ").collect();
                if vecs.len() > 1 {
                    let len = vecs.len();
                    for index in 0..len {
                        let index = len - 1 - index;
                        let item = vecs[index];
                        childs = vec![Self::parse_trim_node(runtime, generates, item, x, childs)?];
                        if index == 0 {
                            for x in &childs {
                                main_node.push(x.clone());
                            }
                            return Ok(());
                        }
                    }
                }
            }
            let node = Self::parse_trim_node(runtime, generates, trim_x, x, childs)?;
            main_node.push(node);
            return Ok(());
        } else {
            //string,replace space to only one
            let mut data = x.to_owned();
            if space <= 1 {
                data = x.to_string();
            } else {
                data = x[(space - 1)..].to_string();
            }
            main_node.push(NodeType::NString(StringNode::new(runtime, &data)?));
            for x in childs {
                main_node.push(x);
            }
            return Ok(());
        }
    }

    fn count_space(arg: &str) -> i32 {
        let cs = arg.chars();
        let mut index = 0;
        for x in cs {
            match x {
                ' ' => {
                    index += 1;
                }
                _ => {
                    break;
                }
            }
        }
        return index;
    }

    ///find_child_str
    fn find_child_str(
        line_index: i32,
        space_index: i32,
        arg: &str,
        m: &HashMap<i32, i32>,
    ) -> (String, i32) {
        let mut result = String::new();
        let mut skip_line = -1;
        let mut line = -1;
        let lines = arg.lines();
        for x in lines {
            line += 1;
            if line > line_index {
                let cached_space = *m.get(&line).unwrap();
                if cached_space > space_index {
                    result = result + x + "\n";
                    skip_line = line;
                } else {
                    break;
                }
            }
        }
        return (result, skip_line);
    }

    ///Map<line,space>
    fn create_line_space_map(arg: &str) -> HashMap<i32, i32> {
        let mut m = HashMap::new();
        let lines = arg.lines();
        let mut line = -1;
        for x in lines {
            line += 1;
            let space = PyRuntime::count_space(x);
            //dothing
            m.insert(line, space);
        }
        return m;
    }
}
