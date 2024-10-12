#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Target(pub usize);

impl Default for Target {
    fn default() -> Self {
        Self(16)
    }
}

impl From<usize> for Target {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for Target {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Required".into())
        } else {
            value
            .parse::<usize>()
            .map(|p| p.into())
            .map_err(|e| e.to_string())
        }
    }
}

// impl Into<String> for Target {
//     fn into(self) -> String {
//         self.0.to_string()
//     }
// }

// impl std::ops::Add for Target {
//     type Output = Target;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0)
//     }
// }

// impl std::ops::AddAssign for Target {
//     fn add_assign(&mut self, rhs: Self) {
//         self.0 += rhs.0;
//     }
// }

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}