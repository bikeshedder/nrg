use std::sync::Arc;

use crate::actuators::Actuator;

struct HeatingUnit {
    pub request_heating: Arc<dyn Actuator>,
    pub request_cooling: Arc<dyn Actuator>,
}
