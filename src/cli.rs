use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Option pricer",
    about = "Binomial, Black-Sholes and Monte-Carlo option pricers"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}