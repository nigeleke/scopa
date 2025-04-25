mod id;
mod name;
mod team;

pub type Teams = Vec<Team>;

pub use id::Id as TeamId;
pub use name::Name as TeamName;
pub use team::Team;
