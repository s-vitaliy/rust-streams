pub fn test_f() -> (){
    print!("Test")
}

enum NotUsed {

}

struct Attributes {

}

pub trait GraphStageLogic {

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
