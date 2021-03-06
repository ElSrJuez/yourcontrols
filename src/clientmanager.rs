use std::{collections::HashMap};

#[derive(Default)]
pub struct Client {
    pub observer_mode: bool,
    pub is_server: bool
}

pub struct ClientManager {
    clients: HashMap<String, Client>,
    current_control: Option<String>
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            current_control: None
        }
    }

    pub fn client_has_control(&self, name: &str) -> bool {
        if let Some(client_name) = self.current_control.as_ref() {
            return name == client_name
        }
        return false;
    }

    pub fn set_client_control(&mut self, name: String) -> Option<String> {
        if self.clients.contains_key(&name) {
            let previous_name = self.current_control.take();

            self.current_control = Some(name);

            return previous_name;
        }
        return None
    }

    pub fn get_client_in_control(&self) -> Option<&String> {
        self.current_control.as_ref()
    }

    pub fn set_no_control(&mut self) {
        self.current_control = None;
    }

    pub fn add_client(&mut self, name: String) {
        self.clients.insert(name.to_string(), Default::default());
    }

    pub fn remove_client(&mut self, name: &str) {
        self.clients.remove(name);
    }

    pub fn is_observer(&self, name: &str) -> bool {
        if let Some(client) = self.clients.get(name) {
            return client.observer_mode
        }
        return false
    }

    pub fn set_observer(&mut self, name: &str, is_observer: bool) {
        if let Some(client) = self.clients.get_mut(name) {
            client.observer_mode = is_observer;
        }
    }

    pub fn set_server(&mut self, name: &str, is_server: bool) {
        if let Some(client) = self.clients.get_mut(name) {
            client.is_server = is_server;
        }
    }

    pub fn client_is_server(&self, name: &str) -> bool {
        if let Some(client) = self.clients.get(name) {
            return client.is_server
        }
        return false;
    }

    pub fn reset(&mut self) {
        self.clients.clear();
        self.current_control = None;
    }
}