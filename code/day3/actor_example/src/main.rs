mod world_state;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    world_state::run().await?;

    world_state::set_value("foo".to_string(), "bar".to_string()).await?;
    if let Some(value) = world_state::get_value("foo".to_string()).await? {
        println!("Got foo: {}", value);
    } else {
        println!("Key 'foo' not found");
    }

    world_state::set_value("hello".to_string(), "world".to_string()).await?;
    if let Some(value) = world_state::get_value("hello".to_string()).await? {
        println!("Got hello: {}", value);
    }

    world_state::delete_value("foo".to_string()).await?;
    match world_state::get_value("foo".to_string()).await? {
        Some(value) => println!("Got foo after delete: {}", value),
        None => println!("Key 'foo' correctly not found after delete"),
    }

    Ok(())    
}
