use crate::resource_manager::{AccessMode, AsResourceManager, ResourceManager, Scope};
use std::time::Duration;

#[test]
fn get_instruments_with_expression() {
    let resource_manager = ResourceManager::new().unwrap();

    let resources = resource_manager
        .get_resources_with_expression("?*INSTR")
        .unwrap();
    println!("{:?}", resources);

    for resource in &resources {
        let instrument = resource_manager.open_with_expression(
            resource,
            AccessMode::NO_LOCK,
            Duration::from_secs(0),
        );

        println!("{}", resource);
        match instrument {
            Ok(mut instrument) => {
                println!("opened");
                let identification = instrument.query_identification().unwrap();

                println!("{:?}", identification);
            }
            Err(e) => println!("{}", e),
        }
    }
}

#[test]
fn get_instruments_with_scope() {
    let resource_manager = ResourceManager::new().unwrap();

    let resources = resource_manager
        .get_resources_with_scope(Scope::Local)
        .unwrap();
    println!("{:?}", resources);

    for resource in resources {
        let instrument = resource_manager.open_with_expression(
            &resource,
            AccessMode::NO_LOCK,
            Duration::from_secs(0),
        );

        println!("{}", resource);
        match instrument {
            Ok(mut instrument) => {
                println!("opened");
                let identification = instrument.query_identification().unwrap();

                println!("{:?}", identification);
            }
            Err(e) => println!("{}", e),
        }
    }
}

#[test]
fn get_instrument_with_identification() {
    let resource_manager = ResourceManager::new().unwrap();

    let mut instrument = resource_manager
        .open_with_identification(
            "manufacturer",
            "model",
            "serial_number",
            AccessMode::NO_LOCK,
            Scope::Local,
            Duration::from_secs(0),
        )
        .unwrap();

    let identification = instrument.query_identification().unwrap();

    println!("{:?}", identification);
}
