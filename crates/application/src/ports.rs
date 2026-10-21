use std::rc::Rc;

use domain::ports::{Authorization, Dialogs, History, Outbox};

/// The ports a [`Session`](crate::Session) is served through, one adapter
/// behind each.
#[derive(Clone)]
pub struct Ports {
    pub authorization: Rc<dyn Authorization>,
    pub dialogs: Rc<dyn Dialogs>,
    pub history: Rc<dyn History>,
    pub outbox: Rc<dyn Outbox>,
}

/// One adapter that serves every port, as a Telegram client does.
impl<Adapter> From<Rc<Adapter>> for Ports
where
    Adapter: Authorization + Dialogs + History + Outbox + 'static,
{
    fn from(adapter: Rc<Adapter>) -> Self {
        Self {
            authorization: adapter.clone(),
            dialogs: adapter.clone(),
            history: adapter.clone(),
            outbox: adapter,
        }
    }
}
