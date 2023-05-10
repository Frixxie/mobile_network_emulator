use std::collections::HashSet;

use futures::StreamExt;
use mongodb::{Collection, Database};
use reqwest::Client;
use serde::{Serialize, Deserialize};

use mobile_network_core_event::{MobileNetworkCoreEvent, EventKind};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventSubscriber {
    notify_endpoint: String,
    kind: EventKind,
    user_ids: Vec<u32>,
}

impl EventSubscriber {
    pub fn new(notify_endpoint: Url, kind: EventKind, user_ids: Vec<u32>) -> Self {
        EventSubscriber {
            notify_endpoint: notify_endpoint.as_str().to_string(),
            kind,
            user_ids,
        }
    }

    pub fn get_event_type(&self) -> &EventKind {
        &self.kind
    }

    pub fn get_notify_endpoint(&self) -> Url {
        Url::parse(&self.notify_endpoint).unwrap()
    }

    pub fn get_user_ids(&self) -> Vec<&u32> {
        self.user_ids.iter().collect()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Subscriber {
    subscriber: EventSubscriber,
    recieved_events: HashSet<MobileNetworkCoreEvent>,
}

impl Subscriber {
    pub fn new(subscriber: EventSubscriber) -> Self {
        Self {
            subscriber,
            recieved_events: HashSet::new(),
        }
    }

    pub fn get_subscriber(&self) -> &EventSubscriber {
        &self.subscriber
    }
}

pub struct MobileNetworkExposure {
    event_subscribers: Vec<Subscriber>,
    http_client: Client,
}

impl MobileNetworkExposure {
    pub fn new() -> Self {
        Self {
            event_subscribers: Vec::new(),
            http_client: Client::new(),
        }
    }

    pub fn add_subscriber(&mut self, event_subscriber: EventSubscriber) {
        self.event_subscribers
            .push(Subscriber::new(event_subscriber));
    }

    pub fn get_subscribers(&self) -> Vec<&Subscriber> {
        self.event_subscribers.iter().collect()
    }

    pub async fn publish_events(&mut self, database: &Database) {
        let events = self.get_events(database).await;
        for subscriber in self.event_subscribers.iter_mut() {
            let res = events
                .iter()
                .filter(|event| {
                    event.get_event_type() == subscriber.subscriber.get_event_type()
                        && !subscriber.recieved_events.contains(event)
                        && subscriber
                            .subscriber
                            .get_user_ids()
                            .contains(&&event.get_user_id())
                }).cloned()
                .collect();
            self.http_client
                .post(subscriber.subscriber.get_notify_endpoint())
                .json::<Vec<MobileNetworkCoreEvent>>(&res)
                .send()
                .await
                .unwrap();
            for event in res {
                subscriber.recieved_events.insert(event);
            }
        }
    }

    pub async fn get_events(&self, database: &Database) -> Vec<MobileNetworkCoreEvent> {
        let collection: Collection<MobileNetworkCoreEvent> = database.collection("Events");
        collection
            .find(None, None)
            .await
            .unwrap()
            .collect::<Vec<Result<_, _>>>()
            .await
            .iter()
            .filter_map(|r| r.clone().ok())
            .collect()
    }
}
