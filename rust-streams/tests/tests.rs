
mod tests {
    use rust_streams::{interpreter::{GraphInterpreter, GraphInterpreterImpl}, materializer::MaterializerImpl, stage::GraphStageImpl};


    #[test]
    fn it_works() {
        let mut int = GraphInterpreterImpl{
            sub_fusing_materializer: None,
            logics: vec![],
            materializer: &MaterializerImpl{}
        };

        int.init(None);
        int.init(None)
    }

}