use clap::Parser;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct WLED {
    ip: String,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    let conn = Connection::open("wled.db")?;
    conn.close();
    Ok(())
}
