use std::any::Any;

use crate::{shape::shape::{Inlet, Outlet}, stage::{GraphStageLogic, GraphStageWithMaterializedValue}};

struct Connection {

}

struct GraphAssemblyImpl<'a> {
    inlets: Vec<&'a dyn Inlet<&'a dyn Any>>,
    outlets: Vec<&'a dyn Outlet<&'a dyn Any>>,
    stages: Vec<&'a dyn GraphStageWithMaterializedValue<'a, &'a dyn Any, &'a dyn Any>>
}

trait GraphAssembly {
    fn materialize(&self) -> (Vec<& Connection>, Vec<& dyn GraphStageLogic>);
}

impl GraphAssembly for GraphAssemblyImpl<'_> {
    fn materialize(&self) -> (Vec<& Connection>, Vec<& dyn GraphStageLogic>){
        return (vec![], vec![])
    }

}
