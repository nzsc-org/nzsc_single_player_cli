pub trait CommandLineApp {
    fn initial_prompt(&self) -> String;
    fn next(&mut self, response: String) -> Prompt;
}

pub struct Prompt {
    pub text: String,
    pub is_final: bool
}
