use crate::domain::{SubscriberName, SubscriberEmail};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
