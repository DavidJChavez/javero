mod list;
use argh::FromArgs;
use list::ListCommand;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(help_triggers("help"))]
/// JDK Version manager
struct Javero {
    #[argh(subcommand)]
    sub_command: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    List(ListCommand),
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let javero: Javero = argh::from_env();

    match javero.sub_command {
        SubCommand::List(scmd) => {
            scmd.run().await?;
        },
    }

    Ok(())
}
