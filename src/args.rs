use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RudderArgs{
    #[clap(subcommand)]
    pub command: RudderCommand,
}

#[derive(Subcommand, Debug)]
pub enum RudderCommand {
    /// Create a Flutter project folder structure.
    Create,

    /// Add a feature/sub feature to the Flutter project.
    Add(AddCommand),
}


#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct AddCommand {
    /// Name of the feature to add. >> rudder add [feature_name].
    #[clap(short, long)]
    pub feature: String,

    /// Name of the sub feature to add. >> rudder add [feature_name] [sub_feature_name]
    #[clap(short, long)]
    pub sub_feature: Option<String>,
}

