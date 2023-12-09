use derive_builder::Builder;

#[derive(Default, Builder, Debug)]
pub struct Device {
    entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub enum Entity {
    //Sensor(Sensor),
}

fn builder_test() {
    let builder = DeviceBuilder::default();
}
