use crate::interpreter::GraphInterpreter;

pub fn test_f() -> (){
    print!("Test")
}

enum NotUsed {

}

struct Attributes {

}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphStageLogicError {

}

pub trait GraphStageLogic<'a> {
    fn set_id(&self, id: usize);
    fn set_interpreter<'b>(&self, interpreter: &GraphInterpreter<'a>);
    fn before_pre_start(&self) -> Result<(), GraphStageLogicError>;
    fn pre_start(&self) -> Result<(), GraphStageLogicError>;
    fn fail_stage(&self, error: GraphStageLogicError);
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
