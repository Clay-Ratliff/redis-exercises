extern crate redis;

use redis::Commands;
use redis::Connection;
use structopt::StructOpt;

// struct to hold command line arguments
#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long, default_value = "redis://localhost:12000")]
    insert_target: String,
    #[structopt(short, long, default_value = "redis://localhost:12000")]
    read_target: String,
}

fn main() {

    let opts = Opts::from_args();

    println!("Opts has: {:#?}", opts);

    println!("Creating insert connection to Redis on host {}", opts.insert_target);
    let mut connection = create_connection(opts.insert_target);

    match add_ordinals(&mut connection) {
        Err(err) => {
            println!("Could not insert ordinals into database");
            println!("  {}:  {}", err.category(), err);
        }
        Ok(()) => {}
    }

    println!("Creating read connection to Redis on host {}", opts.read_target);
    let mut connection = create_connection(opts.read_target);

    match read_ordinals(&mut connection) {
        Err(err) => {
            println!("Could not insert ordinals into database");
            println!("  {}:  {}", err.category(), err);
        }
        Ok(()) => {}
    }
}

fn create_connection(url: String) -> Connection {
    // Creates a client, which is then used to create a connection to redis
    let client = redis::Client::open(url);
    let connection = client.unwrap().get_connection();
    return connection.unwrap();
}

fn add_ordinals(conn:&mut Connection) -> redis::RedisResult<()> {
    // The values are stored as a sorted set called "ordinals", with a string value of the ordinal,
    // and a score equal to the numeric value of the ordinal being inserted
    for index in 1..=100 {
        // we don't care about the return value
        let _: () = conn.zadd("ordinals", index.to_string(), index)?;
    }
    Ok(())
}

fn read_ordinals(conn:&mut Connection) -> redis::RedisResult<()> {
    // we read the sorted set, asking for them in reverse ordering
    let ordinals: Vec<String> = conn.zrevrange("ordinals", 0, -1)?;
    println!("Length of ordinals: {}", ordinals.len());
    println!("ordinals: {:#?}", ordinals);
    Ok(())
}
