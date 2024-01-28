use redb::{Database, ReadableTable, TableDefinition};
use rouille::{router, Response};
use std::fs::File;
use std::path::Path;

const _USER_TABLE: TableDefinition<&str, Vec<&str>> = TableDefinition::new("user_data");
const LINK_TABLE: TableDefinition<&str, (&str, u64)> = TableDefinition::new("link_data");
const _ACCESS_TABLE: TableDefinition<&str, i64> = TableDefinition::new("access_data");
const _PLUGIN_TABLE: TableDefinition<&str, (&str, &str, &str, Vec<u8>)> =
    TableDefinition::new("plugin_data");

fn main() -> anyhow::Result<()> {
    // Create all the database files if they don't exist yet

    if Path::new(&*shellexpand::tilde("~/userdb")).exists() {
        File::create(&*shellexpand::tilde("~/userdb"))?;
    }

    if Path::new(&*shellexpand::tilde("~/linkdb")).exists() {
        File::create(&*shellexpand::tilde("~/linkdb"))?;
    }

    if Path::new(&*shellexpand::tilde("~/accessdb")).exists() {
        File::create(&*shellexpand::tilde("~/accessdb"))?;
    }

    if Path::new(&*shellexpand::tilde("~/plugindb")).exists() {
        File::create(&*shellexpand::tilde("~/plugindb"))?;
    }

    // Create all the database objects to prep for future transactions

    let _userdb = Database::create(&*shellexpand::tilde("~/userdb"))?;
    let linkdb = Database::create(&*shellexpand::tilde("~/linkdb"))?;
    let _accessdb = Database::create(&*shellexpand::tilde("~/accessdb"))?;
    let _plugindb = Database::create(&*shellexpand::tilde("~/plugindb"))?;

    // Start the server and start routing non-nginx managed pathways

    rouille::start_server("127.0.0.1:7878", move |request| {
        router!(request,
            (GET) (/invite/{id: String}) => {
                // Prepare read transaction on the link database
                let read = linkdb.begin_read().unwrap();
                // Open the link table for reading using the LINK_TABLE format
                let table = read.open_table(LINK_TABLE).unwrap();

                if let Some(link) = table.get(id.as_str()).unwrap() {
                    let (link_code, channel_id) = link.value();

                    let link_state = ureq::get(format!("https://discord.com/api/v10/invites/{}", link_code).as_str())
                        .call()
                        .unwrap();

                    if link_state.status() == 404 {
                        let new_link = ureq::post(format!("https://discord.com/api/v10/channels/{}/invites", channel_id).as_str())
                            .set("Authorization", env!("BOT_TOKEN"))
                            .send_json(
                                ureq::json!({
                                    "max_age": 0,
                                    "target_type": 2,
                                    "target_application_id": 1200665926098497576,
                                }))
                            .unwrap();

                        todo!()
                    }

                    Response::redirect_303(link.value())
                }

                Response::redirect_303("https://permavite.com/404.html")
            },
            _ => Response::text("Unknown")
        )
    });
}
