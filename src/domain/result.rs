use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("scopa must be played with 2, 3, 4 or 6 teams only; 2 and 3 teams can be a single player or pair of players")]
    InvalidNumberOfTeams,

    #[error("teams cannot change after the game has started")]
    TeamsCannotBeChanged,

    #[error("the game is not started")]
    GameNotStarted,

    #[error("the game is already complete")]
    GameAlreadyComplete,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_can_be_displayed() {
        let errors = vec![
            Error::InvalidNumberOfTeams,
            Error::TeamsCannotBeChanged,
            Error::GameNotStarted,
            Error::GameAlreadyComplete,
        ];

        let expected = vec![
            "scopa must be played with 2, 3, 4 or 6 teams only; 2 and 3 teams can be a single player or pair of players",
            "teams cannot change after the game has started",
            "the game is not started",
            "the game is already complete"
        ];

        assert_eq!(errors.len(), expected.len());
        assert_eq!(errors.len(), std::mem::variant_count::<Error>()); 

        for (i, error) in errors.into_iter().enumerate() {
            assert_eq!(error.to_string(), expected[i])
        }
    }
}