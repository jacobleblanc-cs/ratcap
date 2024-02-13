use zbus::{Connection, Result, proxy};

// Not sure what the exact service and paths should be,
// basing this off of the zbus example for now
#[proxy(
    interface = "org.freedesktop.ratbag1",
    default_service = "org.freedesktop.ratbag",
    default_path = "/usr/bin/ratbagd"
)]
// The below code is pulled directly from zbus example
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `proxy` macro creates `MyGreeterProxy` based on `Notifications` trait.
    let proxy = RatbagProxy::new(&connection).await?;
    let reply = proxy.say_hello("Maria").await?;
    println!("{reply}");

    Ok(())
}
