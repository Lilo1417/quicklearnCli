use rustyline::completion::{Completer, Pair};
use rustyline::{Context, Helper, Highlighter, Hinter, Result as RLResult, Validator};

pub struct CommandCompleter {
    pub commands: Vec<&'static str>,
}

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> RLResult<(usize, Vec<Pair>)> {
        let line = &line[..pos];
        let start = line.rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word = &line[start..];

        let matches = self
            .commands
            .iter()
            .filter(|c| c.starts_with(word))
            .map(|c| Pair { display: c.to_string(), replacement: c.to_string() })
            .collect();

        Ok((start, matches))
    }
}

#[derive(Helper, rustyline::Completer, Hinter, Highlighter, Validator)]
pub struct ReplHelper {
    #[rustyline(Completer)]
    pub completer: CommandCompleter,
}
