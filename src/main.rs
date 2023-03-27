use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

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
    println!("{:?}", args);

    let conn = Connection::open("wled.db")?;
    // create db of it doen´t exists
    let creation_result = create_db_if_not_exist(&conn);

    // handle ls
    if let true = args.ls {
        println!("list wled instances");
        list_wled_instances(&conn);
        return Ok(());
    }

    // handle subcommands
    if let Some(commands) = args.command {
        match commands {
            Commands::Add { ip, name } => {
                let wled = WLED {ip, name};
                let ret = add_wled_instances(&conn, &wled);
                println!("return: {:#?}", ret);
                return Ok(());
            },
            Commands::Remove { ip, name } => delete_wled_instances(&conn, ip, name),
            Commands::Enable { ip, name } => todo!(),
            Commands::Disable { ip, name } => todo!(),
        }
    }else{
        println!("No command provided");
        return Ok(());
    }
}

fn create_db_if_not_exist(conn: &Connection) -> Result<(), rusqlite::Error>{
    let query = "CREATE TABLE IF NOT EXISTS WLED (ip STRING, name STRING)";
    conn.execute(query, ())?;
    Ok(())
}

fn list_wled_instances(conn: &Connection) -> Result<(), rusqlite::Error>{
    let mut stmt = conn.prepare("SELECT * FROM WLED")?;
    let wled_instances = stmt.query_map((), |row| {
        Ok(WLED {
            name: row.get(0)?,
            ip: row.get(1)?,
        })
    })?;
    for wled_instance in wled_instances{
        if let Ok(i) = wled_instance {
            println!("{:?}", i);
        }else{
            println!("Error fetching WLED instance");
        }
    }
    Ok(())
}

fn add_wled_instances(conn: &Connection, wled: &WLED) -> Result<usize, rusqlite::Error>{
    conn.execute("INSERT INTO WLED (ip, name) VALUES (?1, ?2)", &[&wled.ip, &wled.name])
}

fn delete_wled_instances(conn: &Connection, ip: Option<String>, name: Option<String>) -> Result<()>{
    match ip {
        Some(ip) => {
            conn.execute("DELETE FROM WLED WHERE ip LIKE '?1'", &[&ip])?;
            Ok(())
        },
        None => match name {
            Some(name) => {
                conn.execute("DELETE FROM WLED WHERE name LIKE '?1'", &[&name])?;
                Ok(())
            },
            None => {
                println!("No params provided!");
                return Ok(())
            },
        },
    }
}


