pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Cont,
    Back,
    Break(String),
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            "c" | "cont" | "continue" => Some(DebuggerCommand::Cont),
            "bt" | "back" | "backtrace" => Some(DebuggerCommand::Back),
            "b" | "break" => {
                if tokens.len() == 1 {
                    Some(DebuggerCommand::Break("".to_string()))
                } else {
                    Some(DebuggerCommand::Break(tokens[1].to_string()))
                }
            }

            // Default case:
            _ => None,
        }
    }
}
