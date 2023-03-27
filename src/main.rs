use serde_json::json;
use clap::{Parser, Subcommand};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct WLED {
    ip: String,
    name: String,
}

#[derive(Parser, Debug)]
#[command(name = "WLEDC")]
#[command(author = "Paul Finkbeiner")]
#[command(version = "1.0")]
#[command(about = "WLED Controller CLI Tool", long_about = None)]
struct Args {
    /// Lists persisted wled instances
    #[arg(short, long)]
    ls: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// adds a new wled instance with ip and custom name
    Add {
        /// ip address of the wled instance
        #[arg(short, long)]
        ip: String,
        /// custom name of the wled instance
        #[arg(short, long)]
        name: String,
    },
    /// removes a led instance with ether ip or name
    Remove {
        /// ip address of the wled instance
        #[arg(short, long)]
        ip: Option<String>,
        /// custom name of the wled instance
        #[arg(short, long)]
        name: Option<String>,
    },
    /// enable wled instance
    Enable {
        /// ip address of the wled instance
        #[arg(short, long)]
        ip: Option<String>,
        /// custom name of the wled instance
        #[arg(short, long)]
        name: Option<String>,
    },
    /// disables wled instance
    Disable {
        /// ip address of the wled instance
        #[arg(short, long)]
        ip: Option<String>,
        /// custom name of the wled instance
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    let conn = Connection::open("wled.db")?;
    // create db of it doen´t exists
    let _ = create_db_if_not_exist(&conn);

    // handle ls
    if let true = args.ls {
        println!("list wled instances");
        let _ = list_wled_instances(&conn);
        return Ok(());
    }

    // handle subcommands
    if let Some(commands) = args.command {
        match commands {
            Commands::Add { ip, name } => {
                let wled = WLED { ip, name };
                add_wled_instances(&conn, &wled)
            }
            Commands::Remove { ip, name } => delete_wled_instances(&conn, ip, name),
            Commands::Enable { ip, name } => enable_wled_instance(&conn, ip, name),
            Commands::Disable { ip, name } => disable_wled_instance(&conn, ip, name),
        }
    } else {
        println!("No command provided");
        return Ok(());
    }
}

fn create_db_if_not_exist(conn: &Connection) -> Result<(), rusqlite::Error> {
    let query = "CREATE TABLE IF NOT EXISTS WLED (ip STRING, name STRING)";
    conn.execute(query, ())?;
    Ok(())
}

fn list_wled_instances(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM WLED")?;
    let wled_instances = stmt.query_map((), |row| {
        Ok(WLED {
            ip: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    for wled_instance in wled_instances {
        if let Ok(i) = wled_instance {
            println!("{:?}", i);
        } else {
            println!("Error fetching WLED instance");
        }
    }
    Ok(())
}

fn add_wled_instances(conn: &Connection, wled: &WLED) -> Result<()> {
    match conn.execute(
        "INSERT INTO WLED (ip, name) VALUES (?1, ?2)",
        (&wled.ip, &wled.name),
    ) {
        Ok(_) => Ok(()),
        Err(_) => todo!(),
    }
}

fn delete_wled_instances(
    conn: &Connection,
    ip: Option<String>,
    name: Option<String>,
) -> Result<()> {
    match ip {
        Some(ip) => {
            if ip == "*" {
                let _ = drop_wled_instances(&conn);
            } else {
                let result = conn.execute("DELETE FROM WLED WHERE ip LIKE ?1", params![&ip])?;
                println!("{}", result);
            }
            Ok(())
        }
        None => match name {
            Some(name) => {
                if name == "*" {
                    let _ = drop_wled_instances(&conn);
                } else {
                    let result =
                        conn.execute("DELETE FROM WLED WHERE name LIKE ?1", params![&name])?;
                    println!("{}", result);
                }
                Ok(())
            }
            None => {
                println!("No params provided!");
                return Ok(());
            }
        },
    }
}

fn drop_wled_instances(conn: &Connection) {
    let _ = conn.execute("DROP TABLE WLED", ());
}

fn enable_wled_instance(conn: &Connection, ip: Option<String>, name: Option<String>) -> Result<()> {
    // todo get instances directly through sql statement
    let mut stmt = conn.prepare("SELECT * FROM WLED")?;
    let wled_instances = stmt.query_map((), |row| {
        Ok(WLED {
            ip: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let rest_client = reqwest::blocking::Client::new();
    for wled_instance in wled_instances {
        if let Ok(wled) = wled_instance {
            if let Some(ip) = &ip {
                if &wled.ip == ip {
                    let _ = rest_client.post(format!("http://{}/json", &wled.ip)).json(&json!({"on": true})).send();
                }
            }else if let Some(name) = &name {
                if &wled.name == name {
                    let _ = rest_client.post(format!("http://{}/json", &wled.ip)).json(&json!({"on": true})).send();
                }
            }
        }
    }
    Ok(())
}

fn disable_wled_instance(conn: &Connection, ip: Option<String>, name: Option<String>) -> Result<()> {
        // todo get instances directly through sql statement
        let mut stmt = conn.prepare("SELECT * FROM WLED")?;
        let wled_instances = stmt.query_map((), |row| {
            Ok(WLED {
                ip: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        let rest_client = reqwest::blocking::Client::new();
        for wled_instance in wled_instances {
            if let Ok(wled) = wled_instance {
                if let Some(ip) = &ip {
                    if &wled.ip == ip {
                        let _ = rest_client.post(format!("http://{}/json", &wled.ip)).json(&json!({"on": false})).send();
                    }
                }else if let Some(name) = &name {
                    if &wled.name == name {
                        let _ = rest_client.post(format!("http://{}/json", &wled.ip)).json(&json!({"on": false})).send();
                    }
                }
            }
        }
        Ok(())
}
