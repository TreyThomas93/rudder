mod args;
mod dart;

use args::{RudderArgs, RudderCommand};
use clap::Parser;
use rudder::{create_project_structure, add_dependencies, add_feature};

fn main() {
    let args: RudderArgs = RudderArgs::parse();

    match args.command {
        RudderCommand::Create => {
            println!("Creating project structure...");
            create_project_structure();
            println!("Project structure created successfully!");
            add_dependencies();
        }
        RudderCommand::Add(add_command) => {
            

            // Add a sub feature to existing parent feature.
            if let Some (sub_feature) = add_command.sub_feature {
                for feature in add_command.features.split(",") {
                    println!("Adding sub feature {} to {}...", sub_feature, feature);
                    add_feature(feature.trim(), Some(sub_feature.as_str()));
                    println!("Sub feature {} added successfully!", sub_feature);
                }
                return;
            }

            // Add a feature.
            for feature in add_command.features.split(",") {
                println!("Adding {} feature...", feature);
                add_feature(feature.trim(), None);
                println!("{} feature added successfully!\n", feature);
            }
        }
    }

}

