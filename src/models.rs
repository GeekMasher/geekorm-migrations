use geekorm::prelude::*;

/// Users
#[derive(Table, Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Users {
    #[geekorm(primary_key, auto_increment)]
    pub id: PrimaryKey<i32>,

    #[geekorm(unique)]
    pub username: String,

    pub email: Option<String>,

    #[geekorm(new = "UserType::User")]
    pub user_type: UserType,

    #[geekorm(password, hash_algorithm = "sha512")]
    pub password: String,

    #[geekorm(new = "chrono::Utc::now()")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[geekorm(new = "chrono::Utc::now()")]
    pub last_login: chrono::DateTime<chrono::Utc>,
}

pub const USERS: [&str; 7] = [
    "geekmasher",
    "alice",
    "bob",
    "charlie",
    "dave",
    "eve",
    "frank",
];

#[derive(Data, Debug, Clone, Default)]
pub enum UserType {
    Admin,
    #[default]
    User,
}

/// Sessions
#[derive(Table, Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sessions {
    #[geekorm(primary_key, auto_increment)]
    pub id: PrimaryKey<i32>,

    #[geekorm(rand, rand_length = 42, rand_prefix = "token")]
    pub token: String,
}

/// Posts
#[derive(Table, Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Posts {
    #[geekorm(primary_key, auto_increment)]
    pub id: PrimaryKeyInteger,
    pub title: String,

    #[geekorm(foreign_key = "Users.id")]
    pub user: ForeignKey<i32, Users>,

    #[geekorm(new = "chrono::Utc::now()")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[geekorm(new = "chrono::Utc::now()")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
