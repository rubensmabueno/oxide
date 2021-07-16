use std::collections::HashMap;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).unwrap()
        .merge(config::Environment::with_prefix("OXIDE_")).unwrap();

    TimeBasedPartitioner

    // Print out our settings (as a HashMap)
    println!("{:?}",
             settings.try_into::<HashMap<String, String>>().unwrap());
}