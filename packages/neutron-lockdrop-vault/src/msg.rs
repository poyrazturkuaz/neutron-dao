use cwd_interface::Admin;
use cwd_macros::{info_query, voting_query, voting_vault, voting_vault_query};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct InstantiateMsg {
    /// Name contains the vault name which is used to ease the vault's recognition.
    pub name: String,
    /// Description contains information that characterizes the vault.
    pub description: String,
    /// The lockdrop contract behind the vault.
    pub lockdrop_contract: String,
    /// Owner can update all configs including changing the owner. This will generally be a DAO.
    pub owner: Option<Admin>,
    /// Manager can update configs except changing the owner and the lockdrop contract.
    /// This will generally be an operations multisig for a DAO.
    pub manager: Option<String>,
}

#[voting_vault]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        lockdrop_contract: String,
        manager: Option<String>,
        name: String,
        description: String,
    },
}

#[voting_query]
#[voting_vault_query]
#[info_query]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}
