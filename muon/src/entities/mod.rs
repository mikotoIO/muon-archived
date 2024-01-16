use std::collections::HashMap;

use muon_core::Entity;
use muon_macros::Entity;
use scylla::{FromRow, FromUserType, SerializeCql};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, Entity)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub passhash: String,
}

#[derive(Debug, FromRow, Entity)]
#[entity(pkey = "token")]
pub struct Verification {
    pub token: String,
    pub category: Uuid,
    pub account_id: Uuid,
}

#[derive(Debug, FromRow, Entity)]
#[entity(pkey = "token")]
pub struct Session {
    pub token: String,
    pub account_id: String,

    pub expires_at: i64,
}

#[derive(Debug, FromRow, Entity)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub avatar: String,
    pub category: String,
}

// Space-related code
#[derive(Debug, FromRow, Entity)]
pub struct Space {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub variant: String,
    pub created_at: OffsetDateTime,
    pub roles: HashMap<Uuid, Role>,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            icon: None,
            owner_id: None,
            variant: String::new(),
            created_at: OffsetDateTime::now_utc(),
            roles: HashMap::new(),
        }
    }
}

#[derive(Debug, FromUserType, SerializeCql, Entity)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub hoisted: Option<bool>,
    pub color: Option<String>,
    pub permissions: Vec<String>,
    pub order: i32,
}

#[derive(Debug, FromRow, Entity)]
pub struct SpaceMember {
    pub space_id: Uuid,
    pub user_id: Uuid,
    pub roles: Vec<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, FromRow, Entity)]
pub struct Invite {
    pub id: String,
    pub space_id: Uuid,
    pub inviter_id: Option<Uuid>,

    pub created_at: OffsetDateTime,
}

#[derive(Debug, FromRow, Entity)]
#[entity(pkey = "space_id, id")]
pub struct Channel {
    pub space_id: Uuid,
    pub id: Uuid,

    pub variant: String,
    pub name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, FromRow, Entity)]
pub struct Message {
    pub channel_id: Uuid,
    pub id: i64, // Snowflake
    pub timestamp: OffsetDateTime,

    pub author_id: Uuid,
    pub content: String,
}

pub fn build_schemas() {
    let x = muon_core::build_database_definition![
        Account,
        Verification,
        Session,
        User,
        Space,
        Role,
        SpaceMember,
        Invite,
        Channel,
        Message
    ];
    println!("{}", x);
}
