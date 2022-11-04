use crate::state::{Ballot, Poll};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    // We do not use Option<Addr> due to validation; we want to check what the user is passing us is a valid address
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: String,
        vote: String,
    },
    DeletePoll {
        poll_id: String,
    },
    RevokeVote {
        poll_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll { poll_id: String },
    Vote { poll_id: String, address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}
