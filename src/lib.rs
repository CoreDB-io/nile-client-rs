use serde::{Deserialize, Serialize};
use std::error::Error;

use log::error;

#[derive(Serialize, Debug)]
pub struct InstanceUpdate {
    pub op: String,
    pub path: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct EntityInstance {
    pub id: String,
    pub created: String,
    pub updated: String,
    pub seq: i32,

    #[serde(rename = "type")]
    pub type_: String,
    pub properties: serde_json::Value, // Properties are the entity spec
    pub org: String,
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    token: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub enum EventType {
    CREATE,
    UPDATE,
    DELETE,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Event {
    pub timestamp: String,
    pub id: i32,
    pub event_type: EventType,
    pub before: Option<serde_json::Value>,
    pub after: serde_json::Value,
    pub org: String,
}

#[derive(Debug)]
pub struct NileClient {
    base_url: String,
    auth_path: String,
    _token: String,
}

// https://www.thenile.dev/rest-api#tag/entities/operation/getOpenAPI
impl Default for NileClient {
    fn default() -> NileClient {
        NileClient {
            base_url: "https://prod.thenile.dev".to_owned(),
            auth_path: "/auth/login".to_owned(),
            _token: "".to_owned(),
        }
    }
}

impl NileClient {
    pub async fn authenticate(
        &mut self,
        email: String,
        password: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        // TODO: add token authentication
        let body = &serde_json::json!({
            "email": email.to_owned(),
            "password": password.to_owned(),
        });
        // TODO: handle errors. non-200 must be loud
        let auth = client
            .post(format!(
                "{base}{auth}",
                base = self.base_url,
                auth = self.auth_path
            ))
            .json(&body)
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;

        self._token = auth.token;
        Ok(())
    }

    pub fn token_auth(&mut self, token: String) {
        self._token = token;
    }

    // poll for the events in a workspace/entity
    // TODO: how can we handle `seq`?
    pub async fn get_events(
        &self,
        workspace: &str,
        entity_name: &str,
        seq: i64,
        limit: i32,
    ) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let uri = format!(
            "{base_url}/workspaces/{workspace}/events/{entity_name}?seq={seq}&limit={limit}",
            base_url = self.base_url,
            workspace = workspace,
            entity_name = entity_name,
            seq = seq,
            limit = limit
        );
        let client = reqwest::Client::new();
        let resp = client
            .get(uri)
            .header("Authorization", "Bearer ".to_owned() + &self._token)
            .send()
            .await?;
        let events = resp.json::<Vec<Event>>().await?;
        Ok(events)
    }

    // retrieve existing instances of the entity
    pub async fn get_instances(
        &self,
        workspace: &str,
        entity_name: &str,
    ) -> Result<Vec<EntityInstance>, Box<dyn std::error::Error>> {
        let uri = format!(
            "{base_url}/workspaces/{workspace}/instances/{entity_name}",
            base_url = self.base_url,
            workspace = workspace,
            entity_name = entity_name,
        );

        let client = reqwest::Client::new();
        let resp = client
            .get(uri)
            .header("Authorization", "Bearer ".to_owned() + &self._token)
            .send()
            .await?;
        let instances = resp.json::<Vec<EntityInstance>>().await?;
        Ok(instances)
    }

    // update attributes on an existing entity
    // only supports the ReplaceOperation
    // https://www.thenile.dev/rest-api#tag/entities/operation/patchInstance
    pub async fn patch_instance(
        &self,
        workspace: &str,
        org: &str,
        entity_name: &str,
        instance_id: &str,
        updates: Vec<InstanceUpdate>,
    ) -> Result<EntityInstance, Box<dyn Error>> {
        let uri = format!(
            "{base_url}/workspaces/{workspace}/orgs/{org}/instances/{entity_name}/{id}",
            base_url = self.base_url,
            workspace = workspace,
            entity_name = entity_name,
            id = instance_id
        );

        let client = reqwest::Client::new();
        let resp = client
            .patch(uri)
            .json(&updates)
            .timeout(std::time::Duration::from_secs(5))
            .header("Authorization", "Bearer ".to_owned() + &self._token)
            .send()
            .await?;

        if resp.status().is_success() {
            let resp_obj = resp.json::<EntityInstance>().await?;
            Ok(resp_obj)
        } else {
            let errmsg = format!("Error: {}, {}", resp.status().as_u16(), resp.text().await?);
            error!("{errmsg}");
            Err(errmsg.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event() {
        let event = Event {
            timestamp: "2020-10-01T00:00:00Z".to_owned(),
            id: 1,
            event_type: EventType::CREATE,
            before: None,
            after: serde_json::json!({}),
            org: "coredb".to_owned(),
        };
        assert_eq!(event.timestamp, "2020-10-01T00:00:00Z".to_owned());
        assert_eq!(event.id, 1);
        assert_eq!(event.event_type, EventType::CREATE);
        assert_eq!(event.before, None);
        assert_eq!(event.after, serde_json::json!({}));
        assert_eq!(event.org, "coredb".to_owned());
    }
}
