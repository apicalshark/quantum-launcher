use crate::auth::alt::AccountResponse;

use super::{AccountData, AccountType};
use ql_core::{info, pt, IntoJsonError, RequestError, CLIENT};

pub use super::alt::{Account, AccountResponseError, Error};
use serde::Serialize;
pub mod oauth;

const CLIENT_ID: &str = "1160";

#[derive(Serialize)]
struct Agent {
    name: &'static str,
    version: u8,
}
const AGENT: Agent = Agent {
    name: "Minecraft",
    version: 1,
};

pub async fn login_new(
    email: String,
    password: String,
    account_type: AccountType,
) -> Result<Account, Error> {
    // NOTE: It says email, but both username and email are accepted

    info!("Logging into {account_type}... ({email})");
    let mut value = serde_json::json!({
        "username": &email,
        "password": &password,
        "clientToken": account_type.get_client_id()
    });
    insert_agent_field(account_type, &mut value);

    let response = CLIENT
        .post(account_type.yggdrasil_authenticate())
        .json(&value)
        .send()
        .await?;

    let text = if response.status().is_success() {
        response.text().await?
    } else {
        return Err(RequestError::DownloadError {
            code: response.status(),
            url: response.url().clone(),
        }
        .into());
    };

    let account_response = match serde_json::from_str::<AccountResponse>(&text).json(text.clone()) {
        Ok(n) => n,
        Err(err) => {
            if let Ok(res_err) = serde_json::from_str::<AccountResponseError>(&text).json(text) {
                if res_err.error == "ForbiddenOperationException"
                    && res_err.errorMessage == "Account protected with two factor auth."
                {
                    return Ok(Account::NeedsOTP);
                }
            }
            return Err(err.into());
        }
    };

    let entry = account_type.get_keyring_entry(&email)?;
    entry.set_password(&account_response.accessToken)?;

    Ok(Account::Account(AccountData {
        access_token: Some(account_response.accessToken.clone()),
        uuid: account_response.selectedProfile.id,

        username: email,
        nice_username: account_response.selectedProfile.name,

        refresh_token: account_response.accessToken,
        needs_refresh: false,
        account_type,
    }))
}

fn insert_agent_field(account_type: AccountType, value: &mut serde_json::Value) {
    if account_type.yggdrasil_needs_agent_field() {
        if let (Some(value), Ok(insert)) = (value.as_object_mut(), serde_json::to_value(AGENT)) {
            value.insert("agent".to_owned(), insert);
        }
    }
}

pub async fn login_refresh(
    email: String,
    refresh_token: String,
    account_type: AccountType,
) -> Result<AccountData, Error> {
    // NOTE: It says email, but both username and email are accepted

    pt!("Refreshing {account_type} account...");
    let entry = account_type.get_keyring_entry(&email)?;

    let mut value = serde_json::json!({
        "accessToken": refresh_token,
        "clientToken": account_type.get_client_id()
    });
    insert_agent_field(account_type, &mut value);
    let response = CLIENT
        .post(account_type.yggdrasil_refresh())
        .json(&value)
        .send()
        .await?;

    let text = if response.status().is_success() {
        response.text().await?
    } else {
        return Err(RequestError::DownloadError {
            code: response.status(),
            url: response.url().clone(),
        }
        .into());
    };

    let account_response = serde_json::from_str::<AccountResponse>(&text).json(text.clone())?;
    entry.set_password(&account_response.accessToken)?;

    Ok(AccountData {
        access_token: Some(account_response.accessToken.clone()),
        uuid: account_response.selectedProfile.id,

        username: email,
        nice_username: account_response.selectedProfile.name,

        refresh_token: account_response.accessToken,
        needs_refresh: false,
        account_type,
    })
}
