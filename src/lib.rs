mod args;
mod dart;

use std::fs::create_dir;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use dart::*;
use std::process::Command;

pub fn add_feature(feature_name: &str, sub_feature_name: Option<&str>) {

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
                create_file(&path, &format!("{}_service.dart", feature_name), Some(application_service(feature_name)));
            }
            "data" => {
                create_file(&path, &format!("{}_local_repository.dart", feature_name), Some(local_repository(feature_name)));
                create_file(&path, &format!("{}_remote_repository.dart", feature_name), Some(remote_repository(feature_name)));

                // fake repositories
                create_file(&path, &format!("fake_{}_local_repository.dart", feature_name), Some(fake_local_repository(feature_name)));
                create_file(&path, &format!("fake_{}_remote_repository.dart", feature_name), Some(fake_remote_repository(feature_name)));
            }
            "domain" => {
                create_file(&path, &format!("{}_models.dart", feature_name), None);
                create_file(&path, &format!("{}_unions.dart", feature_name), None);
                create_file(&path, &format!("{}_exceptions.dart", feature_name), None);
            }
            "presentation" => {
                create_file(&path, format!("{}_screen.dart", feature_name).as_str(), Some(stateless_widget(feature_name)));
                create_folder(format!("{}\\controllers", &path).as_str());
                create_file(&path, &format!("controllers\\{}_controller.dart", feature_name), Some(controller(feature_name)));
            }
            _ => (),
        }

        
    }
}

pub fn create_project_structure() {
    if !Path::new(".env").exists() {
        // create .env file in root
        create_file("", ".env", None);
    }

    if Path::new("lib/src").exists() {
        println!("**lib/src folder already exists. Please delete the folder and try again.**");
        return;
    }

    // delete main.dart file
    if Path::new("lib/main.dart").exists() {
        std::fs::remove_file("lib/main.dart").expect("Unable to delete main.dart file");
    }

    let root = "lib";

    // create main.dart file
    create_file(root, "main.dart", Some(main_dart()));

    // create src folder
    create_folder(format!("{}\\src", &root).as_str());

    // create app.dart file
    create_file(format!("{}\\src", &root).as_str(), "app.dart", Some(app_dart()));

    // create features folder
    create_folder(format!("{}\\src\\features", &root).as_str());

    // create utils folder
    create_folder(format!("{}\\src\\utils", &root).as_str());

    // create theme.dart file for utils folder
    create_file(format!("{}\\src\\utils", &root).as_str(), "theme.dart", None);

    // create routes.dart file for utils folder
    create_file(format!("{}\\src\\utils", &root).as_str(), "routes.dart", Some(routes_dart()));

    // create an extension.dart file
    create_file(format!("{}\\src\\utils", &root).as_str(), "extensions.dart", Some(extensions_dart()));

    // create services folder
    create_folder(format!("{}\\src\\services", &root).as_str());

    // create logger_service.dart file
    create_file(
        format!("{}\\src\\services", &root).as_str(),
        "logger_service.dart",
        Some(logger_service()),
    );

    // add home feature
    add_feature("home", None);

}

pub fn create_folder(path: &str) {
    create_dir(&path).expect(format!("Error creating folder: {}", &path).as_str());
}

pub fn create_file<'a>(path: &'a str, name: &'a str, data: Option<String>) {
    
    let merged = if path == "" { format!("{}", &name) } else { format!("{}\\{}", &path, &name) };
    
    println!("Adding {:}", merged);

    let mut file = match File::create(&merged) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file {name}: {}", e);
            return;
        },
    };
        

    if let Some(data) = data {
        // write to file
        file.write_all(data.as_bytes()).expect("Error writing to file.");
    }
}

pub fn add_dependencies() {

    let dependencies = format!("flutter pub add {}", ["auto_route", "flutter_riverpod", "logger", "responsive_framework", "intl", "flutter_dotenv"].join(" "));
    let dev_dependencies = format!("flutter pub add --dev {}", ["build_runner", "auto_route_generator", "riverpod_lint"].join(" "));

     if cfg!(target_os = "windows") {
        println!("\nInstalling dependencies...");
        Command::new("cmd")
                .args(["/C", &dependencies]).output().expect("Failed to install dependencies");
        println!("Dependencies installed successfully!");

        println!("\nInstalling dev dependencies...");
        Command::new("cmd")
                .args(["/C", &dev_dependencies]).output().expect("Failed to install dev dependencies");   
        println!("Dev dependencies installed successfully!");   
    } else {

        println!("\nInstalling dependencies...");
        Command::new("sh")
                .arg("-c")
                .arg(&dependencies)
                .output().expect("Failed to install dependencies");
        println!("Dependencies installed successfully!");

        println!("\nInstalling dev dependencies...");
        Command::new("sh")
                .arg("-c")
                .arg(&dev_dependencies)
                .output().expect("Failed to install dev dependencies");
        println!("Dev dependencies installed successfully!");
    };
}