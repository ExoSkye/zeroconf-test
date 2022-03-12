use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroconf::{MdnsService, ServiceRegistration, ServiceType, TxtRecord};
use zeroconf::prelude::*;

#[derive(Default, Debug)]
pub struct Context {
    service_name: String
}

fn on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    context: Option<Arc<dyn Any>>,
) {
    let service = result.unwrap();

    println!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<Context>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();

    println!("Context: {:?}", context);
}

fn main() {
    let mut service = MdnsService::new(ServiceType::new("pepe", "udp").unwrap(), 44180);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    txt_record.insert("id", "eeeee").unwrap();

    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));
    service.set_txt_record(txt_record);

    let event_loop = service.register().unwrap();

    loop {
        event_loop.poll(Duration::from_secs(0)).unwrap();
    }
}
