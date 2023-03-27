use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub email: String,
}

impl From<DiscordUser> for OAuthUser {
    fn from(value: DiscordUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
            avatar: value.avatar,
            email: value.email,
        }
    }
}

impl From<GoogleUser> for OAuthUser {
    fn from(value: GoogleUser) -> Self {
        Self {
            id: value.sub,
            username: value.name,
            avatar: value.picture,
            email: value.email,
        }
    }
}
