use std::fs::create_dir;
use std::fs::File;
use std::path::Path;

// CLI tool for adding features for my Flutter projects or to create an entire folder structure for a new projects.
// Must be in root of project folder to work correctly.

fn main() {
    // Check if lib folder found in root.
    if Path::new("lib").exists() == false {
        panic!(
            "lib folder not found in root. Please run this command in the root of your project."
        );
    }

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        panic!("Missing arguments. Please provide a command and type.");
    }

    match args[1].as_str() {
        "add" => {
            let a = args.get(2);

            if let Some(a) = a {
                let m: Vec<&str> = a.split(":").map(|s| s).collect();

                if m.len() != 2 {
                    panic!("Invalid format");
                }

                match m[0] {
                    "feature" => {
                        if m[1].trim() == "" {
                            panic!("Invalid feature name");
                        }

                        if m[1].trim().contains("\"") {
                            panic!("Detected mulitple features. Try using features keyword instead.");
                        }

                        if Path::new(format!("lib/src/features/{}", m[1]).as_str()).exists() {
                            panic!(
                                "Feature already exists. Please choose a different name or delete the existing feature."
                            );
                        }

                        println!("Adding features for {}...", m[1]);

                        add_feature(m[1]);
                    },
                    "features" => {
                        if m[1].trim() == "" {
                            panic!("Invalid feature name");
                        }

                        let features = m[1].split(",").map(|s| s.trim()).collect::<Vec<&str>>();

                        for feature in features.iter() {

                            if Path::new(format!("lib/src/features/{}", feature).as_str()).exists() {
                                println!(
                                   "Feature {} already exists. Please choose a different name or delete the existing feature.", *feature
                                );
                                continue;
                            }

                            println!("Adding feature for {}...", *feature);

                            add_feature(*feature);
                        }

                        
                    }
                    _ => println!("Unknown type"),
                }
            } else {
                println!("Missing type");
            }
        }
        "create" => {
            println!("Creating project structure...");

            create_project_structure();
        }
        _ => println!("Unknown command"),
    }
}

fn add_feature(feature_name: &str) {
    let layers = ["application", "data", "domain", "presentation"];

    let base_path = "lib\\src\\features";

    // create [feature_name] folder
    create_folder(format!("{base_path}\\{}", feature_name).as_str());

    // create folders for layers
    for &layer in layers.iter() {
        let path = format!("{base_path}\\{}\\{}", feature_name, layer);

        create_folder(&path);

        let mut file_name = String::new();

        match layer {
            "application" => {
                file_name = format!("{}_service.dart", feature_name);
            }
            "data" => {
                file_name = format!("{}_repository.dart", feature_name);
            }
            "domain" => {
                file_name = format!("{}_model.dart", feature_name);
            }
            "presentation" => {
                create_file(&path, format!("{}_screen.dart", feature_name).as_str());

                create_folder(format!("{}\\controllers", &path).as_str());

                file_name = format!("controllers\\{}_controller.dart", feature_name);
            }
            _ => (),
        }

        create_file(&path, &file_name);
    }
}

fn create_project_structure() {
    if Path::new("lib/src").exists() {
        panic!(
            "src folder detected in lib folder. Panicking to prevent overwriting existing files."
        );
    }

    let root = "lib";

    // create src folder
    create_folder(format!("{}\\src", &root).as_str());

    // create features folder
    create_folder(format!("{}\\src\\features", &root).as_str());

    // create utils folder
    create_folder(format!("{}\\src\\utils", &root).as_str());

    // create theme.dart file
    create_file(format!("{}\\src\\utils", &root).as_str(), "theme.dart");

    // create services folder
    create_folder(format!("{}\\src\\services", &root).as_str());

    // create logger_service.dart file
    create_file(
        format!("{}\\src\\services", &root).as_str(),
        "logger_service.dart",
    );

    // create exceptions folder
    create_folder(format!("{}\\src\\exceptions", &root).as_str());

    // add home feature
    add_feature("home");
}

fn create_folder(path: &str) {
    create_dir(&path).expect(format!("Error creating folder: {}", &path).as_str());
}

fn create_file<'a>(path: &'a str, name: &'a str) {
    File::create(format!("{}\\{}", &path, &name))
        .expect(format!("Error creating file: {}/{}", &path, &name).as_str());
}
