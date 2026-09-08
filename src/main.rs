
use clap::{Parser, Subcommand};
use forge::store::FileStore;

#[derive(Parser)]
#[command(name = "forge")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Endpoint {
        #[command(subcommand)]
        command: EndpointCommand,
    },
    Event {
        #[command(subcommand)]
        command: EventCommand,
    },
    Delivery {
        #[command(subcommand)]
        command: DeliveryCommand,
    },
}

#[derive(Subcommand)]
enum EndpointCommand {
    Add { url: String },
    List,
}

#[derive(Subcommand)]
enum EventCommand {
    Send {
        #[arg(long, short)]
        endpoint: String,
        payload: String,
    },
}

#[derive(Subcommand)]
enum DeliveryCommand {
    List,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut store = FileStore::open()?;

    match cli.command {
        Command::Endpoint { command } => match command {
            EndpointCommand::Add { url } => {
                let ep = store.add_endpoint(url)?;
                println!("{}\t{}", ep.id, ep.url);
            }
            EndpointCommand::List => {
                for ep in store.list_endpoints() {
                    println!("{}\t{}", ep.id, ep.url);
                }
            }
        },
        Command::Event { command } => match command {
            EventCommand::Send { endpoint, payload } => {
                let payload = serde_json::from_str(&payload)?;
                let (event, delivery) = store.send_event(&endpoint, payload)?;
                println!("event\t{}", event.id);
                println!("delivery\t{}\t{:?}", delivery.id, delivery.status);
            }
        },
        Command::Delivery { command } => match command {
            DeliveryCommand::List => {
                for d in store.list_deliveries() {
                    println!("{}\t{}\t{:?}", d.id, d.event_id, d.status);
                }
            }
        },
    }

    Ok(())
}