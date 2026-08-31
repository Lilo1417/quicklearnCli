use crate::user_actions::{self, UserAction};
pub fn handle_action(ua: UserAction) -> Result<usize, std::io::Error> {
    match ua {
        UserAction::Help => UserAction::list_actions(),
        UserAction::Quit => return Ok(0),
        UserAction::AddLernset => return add_lernset(),

    }
    Ok(0)
}

fn add_lernset() -> Result<usize, std::io::Error> {
    
    Ok(0)
}
