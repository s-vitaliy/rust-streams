use crate::interpreter::GraphInterpreter;

pub fn test_f() -> (){
    print!("Test")
}

enum NotUsed {

}

struct Attributes {

}

#[derive(Debug)]
pub struct GraphStageLogicError {

}

pub trait GraphStageLogic {
    fn set_id(&self, id: usize);
    fn set_interpreter(&self, interpreter: &dyn GraphInterpreter);
    fn before_pre_start(&self) -> Result<(), GraphStageLogicError>;
    fn pre_start(&self) -> Result<(), GraphStageLogicError>;
}

pub trait LogicAndMaterializedValue<TMaterialized> {
    fn get_logic(&self) -> dyn GraphStageLogic;
    fn get_materializer(&self) -> TMaterialized;
}

pub trait GraphStageWithMaterializedValue<'a, TShape, TMaterialized> {
    fn create_logic_and_materialized_value(&self) -> &'a dyn LogicAndMaterializedValue<TMaterialized>;
}

pub trait GraphStage {
}

pub struct GraphStageImpl<'a> {
    pub a: &'a str
}
