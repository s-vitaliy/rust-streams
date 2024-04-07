
mod tests {
    use rust_streams::{interpreter::{GraphInterpreter, GraphInterpreterImpl}, materializer::MaterializerImpl, stage::{GraphStageLogic, GraphStageLogicError}};
    use mockall::*;

    mock! {

        pub GraphStageLogic {}

        impl<'a> GraphStageLogic<'a> for GraphStageLogic {
            fn set_id(&self, id: usize);
            fn set_interpreter<'b>(&self, interpreter: &dyn GraphInterpreter<'b>);
            fn before_pre_start(&self) -> Result<(), GraphStageLogicError>;
            fn pre_start(&self) -> Result<(), GraphStageLogicError>;
            fn fail_stage(&self, error: GraphStageLogicError);
        }

    }


    #[test]
    fn test_run_initialization() {
        let mut mock = MockGraphStageLogic::new();
        mock.expect_before_pre_start().times(1).return_const(Ok(()));
        mock.expect_pre_start().times(1).return_const(Ok(()));
        mock.expect_set_id().return_const(());
        mock.expect_set_interpreter().return_const(());

        let mut int = GraphInterpreterImpl{
            sub_fusing_materializer: None,
            logics: vec![Box::new(mock)],
            materializer: &MaterializerImpl{}
        };

        int.init(None);
    }

    #[test]
    fn test_fail_before_pre_start() {
        let mut mock = MockGraphStageLogic::new();
        mock.expect_before_pre_start().times(1).return_const(Err(GraphStageLogicError{}));
        mock.expect_set_id().return_const(());
        mock.expect_set_interpreter().return_const(());
        mock.expect_fail_stage().with(predicate::eq(GraphStageLogicError{})).return_const(());

        let mut int = GraphInterpreterImpl{
            sub_fusing_materializer: None,
            logics: vec![Box::new(mock)],
            materializer: &MaterializerImpl{}
        };

        int.init(None);
    }

    #[test]
    fn test_fail_pre_start() {
        let mut mock = MockGraphStageLogic::new();
        mock.expect_before_pre_start().times(1).return_const(Ok(()));
        mock.expect_pre_start().times(1).return_const(Err(GraphStageLogicError{}));
        mock.expect_set_id().return_const(());
        mock.expect_set_interpreter().return_const(());
        mock.expect_fail_stage().with(predicate::eq(GraphStageLogicError{})).return_const(());

        let mut int = GraphInterpreterImpl{
            sub_fusing_materializer: None,
            logics: vec![Box::new(mock)],
            materializer: &MaterializerImpl{}
        };

        int.init(None);
    }

}