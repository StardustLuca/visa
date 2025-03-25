use crate::resource_manager::{AccessMode, AsResourceManager, ResourceManager, Scope};
use std::{thread::sleep, time::Duration};

#[test]
fn get_instruments_with_expression() {
    let resource_manager = ResourceManager::new().unwrap();

    let resources = resource_manager
        .get_resources_with_expression("?*INSTR")
        .unwrap();
    println!("{:?}", resources);

    for resource in &resources {
        let instrument =
            resource_manager.open(resource, AccessMode::EXCLUSIVE_LOCK, Duration::from_secs(0));

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
        let instrument = resource_manager.open(
            &resource,
            AccessMode::EXCLUSIVE_LOCK,
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
            "RS PRO",
            "IDM-8341",
            "827B070G2",
            AccessMode::EXCLUSIVE_LOCK,
            Scope::Local,
            Duration::from_secs(0),
        )
        .unwrap();

    let identification = instrument.query_identification().unwrap();

    println!("{:?}", identification);
}

#[test]
fn session_stress_test() {
    tracing_subscriber::fmt().with_env_filter("trace").init();
    let resource_manager = ResourceManager::new().unwrap();

    loop {
        let mut instrument = resource_manager
            .open(
                "resource",
                AccessMode::EXCLUSIVE_LOCK,
                Duration::from_secs(0),
            )
            .unwrap();

        let identification = instrument.query_identification().unwrap();

        println!("{:?}", identification);
        sleep(Duration::from_secs(1));
    }
}
