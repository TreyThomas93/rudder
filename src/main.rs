
/* 
Basic folder structure for Flutter projects:
- lib
    - src
        - features
            - home
                - application
                    > home_service.dart
                - data
                    > remote_repository.dart
                    > local_repository.dart
                - domain
                    > home_model.dart
                    > home_union.dart
                - presentation
                    > home_screen.dart
                    - controllers
                        > home_controller.dart
                    - sub_screens
                        ...
        - services
            > *_service.dart
        - utils
            > theme.dart
            > routes.dart
            ...

This is created when the following command is run: rudder create
Must be in root of project folder to work correctly.

Commands:
rudder create - creates the basic folder structure for the project
rudder add feature:[feature_name] - creates the folder structure for a feature

Plan:
- Add ability to create sub_features. (e.g. home -> home/sub_features/roster/sub_features/documents/...)
- Add routes.dart to utils folder on create.
- Update repo names to remote/local.
- Add union file to domain folder.
- Automatically add dart code to dart files.
 */

mod args;

use args::{RudderArgs, RudderCommand};
use clap::Parser;
use std::fs::create_dir;
use std::fs::File;
use std::path::Path;


fn main() {
    let args: RudderArgs = RudderArgs::parse();

    match args.command {
        RudderCommand::Create => {
            println!("Creating project structure...");

            create_project_structure();
        }
        RudderCommand::Add(add_command) => {
            

            if let Some (sub_feature) = add_command.sub_feature {
                println!("Adding sub feature {} to {}...", sub_feature, add_command.feature);
                add_feature(add_command.feature.as_str(), Some(sub_feature.as_str()));
                return;
            }

            println!("Adding {} feature...", add_command.feature);

            add_feature(add_command.feature.as_str(), None);
        }
    }
}

fn add_feature(feature_name: &str, sub_feature_name: Option<&str>) {
    let layers = ["application", "data", "domain", "presentation"];

    let base_path = match sub_feature_name {
        Some(sub_feature_name) => format!("lib\\src\\features\\{}\\sub_features\\{}", feature_name, sub_feature_name),
        None => format!("lib\\src\\features\\{}", feature_name),
    };

    // create [feature_name] folder
    create_folder(&base_path);

     // Create sub_features folder if doesnt exist
    if !Path::new(format!("lib\\src\\features\\{}\\sub_features", feature_name).as_str()).exists() {
        create_folder(format!("lib\\src\\features\\{}\\sub_features", feature_name).as_str());
    }

    let feature_name: &str = match sub_feature_name {
        Some(sub_feature_name) => sub_feature_name,
        None => feature_name,
    }; 

    // create folders for layers
    for &layer in layers.iter() {
        let path = format!("{base_path}\\{}", layer);

        create_folder(&path);

        match layer {
            "application" => {
                create_file(&path, &format!("{}_service.dart", feature_name));
            }
            "data" => {
                create_file(&path, &format!("{}_local_repository.dart", feature_name));
                create_file(&path, &format!("{}_remote_repository.dart", feature_name));
            }
            "domain" => {
                create_file(&path, &format!("{}_models.dart", feature_name));
                create_file(&path, &format!("{}_unions.dart", feature_name));
                create_file(&path, &format!("{}_exceptions.dart", feature_name));
            }
            "presentation" => {
                create_file(&path, format!("{}_screen.dart", feature_name).as_str());

                create_folder(format!("{}\\controllers", &path).as_str());

                create_file(&path, &format!("controllers\\{}_controller.dart", feature_name));
            }
            _ => (),
        }

        
    }
}

fn create_project_structure() {
    if Path::new("lib/src").exists() {
        println!("**lib/src folder already exists. Please delete the folder and try again.**");
        return;
    }

    let root = "lib";

    // create src folder
    create_folder(format!("{}\\src", &root).as_str());

    // create features folder
    create_folder(format!("{}\\src\\features", &root).as_str());

    // create utils folder
    create_folder(format!("{}\\src\\utils", &root).as_str());

    // create theme.dart file for utils folder
    create_file(format!("{}\\src\\utils", &root).as_str(), "theme.dart");

    // create routes.dart file for utils folder
    create_file(format!("{}\\src\\utils", &root).as_str(), "routes.dart");

    // create services folder
    create_folder(format!("{}\\src\\services", &root).as_str());

    // create logger_service.dart file
    create_file(
        format!("{}\\src\\services", &root).as_str(),
        "logger_service.dart",
    );

    // add home feature
    add_feature("home", None);

    println!("Project structure created successfully!");
}

fn create_folder(path: &str) {
    create_dir(&path).expect(format!("Error creating folder: {}", &path).as_str());
}

fn create_file<'a>(path: &'a str, name: &'a str) {
    File::create(format!("{}\\{}", &path, &name))
        .expect(format!("Error creating file: {}/{}", &path, &name).as_str());
}
