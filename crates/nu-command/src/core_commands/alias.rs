use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, Example, PipelineData, Signature, SyntaxShape};

#[derive(Clone)]
pub struct Alias;

impl Command for Alias {
    fn name(&self) -> &str {
        "alias"
    }

    fn usage(&self) -> &str {
        "Alias a command (with optional flags) to a new name"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("alias")
            .required("name", SyntaxShape::String, "name of the alias")
            .required(
                "initial_value",
                SyntaxShape::Keyword(b"=".to_vec(), Box::new(SyntaxShape::Expression)),
                "equals sign followed by value",
            )
            .category(Category::Core)
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(PipelineData::new(call.head))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Alias ll to ls -l",
            example: "alias ll = ls -l",
            result: None,
        }]
    }
}
