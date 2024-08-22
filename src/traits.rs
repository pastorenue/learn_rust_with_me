use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct UserAge {
    pub value: i32,
}

impl From<i32> for UserAge {
    fn from(value: i32) -> Self {
        UserAge { value }
    }
}

pub(crate) fn try_from(x: i32) -> Option<UserAge> {
    let age = UserAge::from(x);

    Some(age)
}

pub(crate) fn try_into(x: i32) -> Option<UserAge> {
    let age: UserAge = x.into();

    Some(age)
}

impl Display for UserAge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

