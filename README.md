# Rudder

## Description

A CLI tool for creating a starter project folder structure and/or adding features to my Flutter projects.

### Creating a folder structure

> Command: **rudder create**

    .env
    - lib
        - src
            - features
                - home
                    - application
                        > home_service.dart
                    - data
                        > fake_home_remote_repository.dart
                        > fake_home_local_repository.dart
                        > home_remote_repository.dart
                        > home_local_repository.dart
                    - domain
                        > home_models.dart
                        > home_unions.dart
                        > home_exceptions.dart
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
                > extensions.dart
                ...

The pattern that I use is the [feature-first](https://codewithandrea.com/articles/flutter-project-structure/) pattern.

A .env file is added to the root of the project. Used with the flutter_dotenv dependency.

Inside the lib folder, the main.dart file is recreated with dynamically generated code.

A subfolder named src is added to the lib folder. Inside of the src folder, three subfolders and one file are added:

- features
- services
- utils
- app.dart

Code is automatically generated for the app.dart file.

> **Features**

This folder contains all of the features (screens) for your application. Within each feature, there are four subfolders that are added:

- application
- data
- domain
- presentation

Code is automatically generated for most of the files within the feature folder. The code is incorporated with a few dependencies, such as Riverpod and AutoRoute.

> **Services**

This folder contains the services files such as logger, biometrics, HTTP client, ect.

> **Utils**

This folder contains utility files such as theme, routes, extensions, ect.

### Adding a feature/multiple features

> Command: **rudder add --features feature_name**

This command adds a **single** feature to the features folder that contains all the appropriate subfolders and files.

> Command: **rudder add --features feature_name_one,feature_name_two**

This command adds **multiple** features to the features folder that contains all the appropriate subfolders and files. The feature names must be separated by a comma with no spaces before or after.

### Adding a sub feature

> Command: **rudder add --feature feature_name --sub_feature sub_feature_name**

This command adds a sub feature to an existing parent feature (1 level). You can only add one sub feature at a time for one parent feature at a time.

### Additional

For more information on commands and options available, use the --help command in the terminal:

> Command: **rudder --help**
>
> Command: **rudder create --help**
>
> Command: **rudder add --help**
