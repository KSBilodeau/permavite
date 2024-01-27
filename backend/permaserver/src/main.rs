use redb::{Database, Error, ReadableTable, TableDefinition};
use rouille::{Response, router};

const _USER_TABLE: TableDefinition<&str, Vec<&str>> = TableDefinition::new("user_data");
const LINK_TABLE: TableDefinition<&str, &str> = TableDefinition::new("link_data");

#[cfg(not(target_os = "wasi"))]
fn main() -> Result<(), Error>{
    // let users = Database::create("/Users/ksbilodeau/userdb")?;
    // let write = users.begin_write()?;
    // {
    //     let mut table = write.open_table(USER_TABLE)?;
    //     table.insert("ksbilodeau", &vec!["gay", "meep"])?;
    //     table.insert("moosh", &vec!["demonic"])?;
    // }
    // write.commit()?;
    //
    // let links = Database::create("/Users/ksbilodeau/linkdb")?;
    // let write = links.begin_write()?;
    // {
    //     let mut table = write.open_table(LINK_TABLE)?;
    //     table.insert("gay", "https://discord.gg/H7mpzzXE")?;
    //     table.insert("meep", "https://discord.gg/uyWEmbY")?;
    //     table.insert("demonic", "https://discord.gg/tNDTnJd")?;
    // }
    // write.commit()?;

    let _ = Database::create("~/userdb")?;
    let linkdb = Database::create("~/linkdb")?;

    rouille::start_server("127.0.0.1:7878", move |request| {
        println!("{}", request.url());
        let result = router!(request,
            (GET) (/api/) => {
                "LATEST PERMAVITE API: PAPI V1".into()
            },
            (GET) (/invite/{id: String }) => {
                let read = linkdb.begin_read().unwrap();
                let table = read.open_table(LINK_TABLE).unwrap();

                let Some(link) = table.get(id.as_str()).unwrap() else {
                   return Response::empty_404();
                };

                format!("INVITE LINK: {}", link.unwrap().value())
            },
            (POST) (/api/v1/register_plugin) => {
                "REGISTER PLUGIN".into()
            },
            _ => "UNKNOWN".into()
        );

        Response::text(result)
    })
}