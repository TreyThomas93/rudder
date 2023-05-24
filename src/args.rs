use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(name="Rudder", author="Trey Thomas", version="1.0.0", about="A program to generate project folder structures and features for Flutter apps.", long_about = None)]
#[command(
    help_template = "Name: {name}\nAuthor: {author}\nAbout: {about}\nVersion: {version}\n\n{usage-heading}\n{usage}\n\n{all-args}\n{tab}"
)]
pub struct RudderArgs{

    /// Subcommand to execute.
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

