use matrix_sdk::{ruma::RoomIdOrAliasId, Client, ClientConfig, SyncSettings};
use url::Url;

use matrix_sdk::ruma::events::{room::message::MessageEventContent, AnyMessageEventContent};
use matrix_sdk_common::uuid::Uuid;

mod settings;
use settings::Settings;

use config::Config;

async fn login_and_sync(settings: &Config) -> Result<Client, matrix_sdk::Error> {
    let homeserver_url = settings.get::<String>("server.homeserver").unwrap();
    let username = settings.get::<String>("user.username").unwrap();
    let password = settings.get::<String>("user.password").unwrap();

    let mut cache = dirs::cache_dir().expect("no home directory found");
    cache.push("faucet_slobber");

    let client_config = ClientConfig::new().store_path(cache);

    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client
        .login(&username, &password, None, Some("faucet slobber bot"))
        .await?;

    println!("logged in as {}", username);

    // An initial sync to set up state and so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store
    client.sync_once(SyncSettings::default()).await.unwrap();

    Ok(client)
}

async fn join_room_and_beg(client: &Client, settings: &Config) {
    let channel = settings.get::<String>("server.channel").unwrap();
    let server = settings.get::<String>("server.server").unwrap();
    let walletaddr = settings.get::<String>("wallet.address").unwrap();

    let roomaliasid: RoomIdOrAliasId = RoomIdOrAliasId::try_from(channel).unwrap();

    let my_room_id = client
        .join_room_by_id_or_alias(&roomaliasid, &[server.try_into().unwrap()])
        .await
        .unwrap()
        .room_id;

    let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(format!(
        "!drip {}",
        walletaddr
    )));

    let txn_id = Uuid::new_v4();
    client
        .room_send(&my_room_id, content, Some(txn_id))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), matrix_sdk::Error> {
    let settings = Settings::new().unwrap();

    let client = login_and_sync(&settings).await?;
    let _x = join_room_and_beg(&client, &settings).await;
    Ok(())
}
