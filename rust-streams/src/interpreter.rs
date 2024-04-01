use std::any::Any;

use crate::{materializer::{Materializer}, shape::shape::{Inlet, Outlet}, stage::{GraphStageLogicError, GraphStageLogic, GraphStageWithMaterializedValue}};

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

pub trait GraphInterpreter<'interpreter> {
    fn init(&mut self, materializer: Option<&'interpreter dyn Materializer>) -> ();
    fn execute(&self, event_limit: i32) -> bool;
    fn is_suspended(&self) -> bool;
    fn fail_stage(&mut self, error: GraphStageLogicError);
}

pub struct GraphInterpreterImpl<'interpreter> {
    pub sub_fusing_materializer: Option<&'interpreter dyn Materializer>,
    pub logics: Vec<Box<dyn GraphStageLogic>>,
    pub materializer: &'interpreter dyn Materializer,
}

impl<'a> GraphInterpreter<'a> for GraphInterpreterImpl<'a> {

    fn init(&mut self, materializer: Option<&'a dyn Materializer>) -> () {
        self.sub_fusing_materializer = materializer.or(Some(self.materializer));

        for i in 0..self.logics.len(){
            let logic = &self.logics[i];
            logic.set_id(i);
            logic.set_interpreter(self);
            if let Err(error) = logic.before_pre_start().and_then(|_| logic.pre_start()) {
                self.fail_stage(error);
            }
        }
    }

    fn fail_stage(&mut self, error: GraphStageLogicError){
        print!("Stage failed with: {:?}", error)
    }

    fn execute(&self, event_limit: i32) -> bool {
        todo!()
    }

    fn is_suspended(&self) -> bool {
        todo!()
    }
}