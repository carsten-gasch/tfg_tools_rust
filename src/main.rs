mod catkin;

use catkin::item::Header;
use catkin::item::Item;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, short, default_value_t = false, help = "Debug")]
    debug: bool,
    #[arg(
        long,
        short,
        default_value_t = 50,
        help = "Umfuhren pro erzeugter Datei"
    )]
    split: u16,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Yard(YardArgs),
}

#[derive(Debug, Args)]
struct YardArgs {
    #[command(subcommand)]
    command: YardCommands,
}

#[derive(Debug, Subcommand)]
enum YardCommands {
    Hapag {
        #[arg(long, short, help = "Kalenderwoche um eindeutige Referenz zu erzeugen")]
        week: u8,
        #[arg(
            long,
            short,
            help = "Datum für die Umfuhr [tt.mm.jjjj oder yyyy-mm-dd]"
        )]
        date: String,
        #[arg(long, short, help = "Anzahl der zu erzeugenden Umfuhren")]
        amount: u16,
    },
    Evergreen {
        #[arg(long)]
        week: u8,
        #[arg(long)]
        date: String,
        #[arg(long)]
        amount: u16,
        #[arg(long)]
        turnout: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Yard(shipper) => match &shipper.command {
            YardCommands::Hapag { week, date, amount } => {
                println!(
                    "creating {} relocations for HAPAG on {} with reference 'HLEGCBMW-KW{}'",
                    amount, date, week
                );
                let mut items: Vec<Item> = Vec::new();
                for i in 1..*amount {
                    let mut item = Item::new();
                }
            }
            YardCommands::Evergreen {
                week,
                date,
                amount,
                turnout,
            } => {
                println!(
                    "creating {} relocations for EVERGREEN on {} with reference '{}-KW{}'",
                    amount, date, turnout, week
                );
            }
        },
    }
}
