use crate::core::runtime::RuntimeManager;
use actix_web::web::Data;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub type HandleMap = HashMap<Uuid, Arc<RwLock<RuntimeManager>>>;
pub type AppData = Data<State>;

pub struct State {
    pub handles: RwLock<HandleMap>,
}

impl State {
    pub fn new() -> State {
        State {
            handles: RwLock::new(HashMap::new()),
        }
    }
}
