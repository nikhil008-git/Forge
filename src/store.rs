//storage/database layer of our lil app!!!!

// We rnn creaetied State, and save() is what stores the current State into forge.json as JSON.
use std::fs;
use std::path::PathBuf;

user serde::{Deserialize, Serialize};
use serde_json::Value;

// imported the structs from the models.rs file
use crate::models::{Delivery, Endpoint, Event};
#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    endpoints: Vec<Endpoint>,
    events: Vec<Event>,
    deliveries: Vec<Delivery>,
}
pub struct FileStore {
    path: PathBuf,
    state: State,
}


impl FileStore {
    pub fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let path = PathBuf::from("forge.json"); //Our storage file will be called forge.json
        let state = if path.exists() {
            serde_json::from_str(&fs::read_to_string(&path)?)?
        } else {
            State::default()
        };
        Ok(Self { path, state })
    }
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&self.path, serde_json::to_string_pretty(&self.state)?)?;
        Ok(())
    }
    pub fn add_endpoint(&mut self, url: String) -> Result<Endpoint, Box<dyn std::error::Error>> {
        let endpoint = Endpoint::new(url);
        self.state.endpoints.push(endpoint.clone());
        self.save()?;
        Ok(endpoint)
    }
    pub fn list_endpoints(&self) -> &[Endpoint] {
        &self.state.endpoints
    }
    pub fn send_event(
        &mut self,
        endpoint_id: &str,
        payload: Value,
    ) -> Result<(Event, Delivery), Box<dyn std::error::Error>> {
        let exists = self.state.endpoints.iter().any(|e| e.id == endpoint_id);
        if !exists {
            return Err(format!("endpoint not found: {endpoint_id}").into());
        }
        let event = Event::new(endpoint_id, payload);
        let delivery = Delivery::pending(&event.id, endpoint_id);
        self.state.events.push(event.clone());
        self.state.deliveries.push(delivery.clone());
        self.save()?;
        Ok((event, delivery))
    }
    pub fn list_deliveries(&self) -> &[Delivery] {
        &self.state.deliveries
    }
}
