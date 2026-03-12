use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GitHubEvent<'a> {
    #[serde(borrow)]
    pub id: &'a str,

    #[serde(borrow)]
    pub repo: Repo<'a>,

    #[serde(rename = "type")]
    pub kind: &'a str,

    #[serde(borrow)]
    pub actor: Actor<'a>,

    #[serde(borrow)]
    pub created_at: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Repo<'a> {
    #[serde(borrow)]
    pub name: Option<&'a str>,
}

#[derive(Serialize, Deserialize)]
pub struct Actor<'a> {
    #[serde(borrow)]
    pub login: Option<&'a str>,
}
