use std::collections::HashSet;

use futures::StreamExt;
use mongodb::{Collection, Database};
use reqwest::Client;
use serde::Serialize;

use crate::mobile_network_core_event::{EventSubscriber, MobileNetworkCoreEvent};

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
        let events = self.get_events(&database).await;
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
                })
                .map(|event| event.clone())
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
            .filter(|r| r.is_ok())
            //We know that this should be fine
            .map(|r| r.clone().unwrap())
            .collect()
    }
}
