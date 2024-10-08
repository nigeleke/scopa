use crate::types::{TeamCount, TeamId};

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("team {0} exists")]
    TeamExists(TeamId),

    #[error("team {0} not found")]
    TeamNotFound(TeamId),
    
    #[error("scopa cannot be played with {0} teams. It must be played with 2, 3, 4 or 6 teams only; 2 and 3 teams can be a single player or pair of players")]
    InvalidNumberOfTeams(TeamCount),

    // #[error("teams cannot change after the game has started")]
    // TeamsCannotBeChanged,

    // #[error("the game is not started")]
    // GameNotStarted,

    // #[error("the game is already complete")]
    // GameAlreadyComplete,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_can_be_displayed() {
        let id = TeamId::new();

        let errors = vec![
            Error::TeamExists(id),
            Error::TeamNotFound(id),
            Error::InvalidNumberOfTeams(0.into()),
            // Error::GameNotStarted,
            // Error::GameAlreadyComplete,
        ];

        let expected = vec![
            format!("team {} exists", id),
            format!("team {} not found", id),
            "scopa cannot be played with 0 teams. It must be played with 2, 3, 4 or 6 teams only; 2 and 3 teams can be a single player or pair of players".into(),
            // "the game is not started",
            // "the game is already complete"
        ];

        assert_eq!(errors.len(), expected.len());
        assert_eq!(errors.len(), std::mem::variant_count::<Error>()); 

        for (i, error) in errors.into_iter().enumerate() {
            assert_eq!(error.to_string(), expected[i])
        }
    }
}