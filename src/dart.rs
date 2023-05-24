pub trait Capitalize {
  fn capitalize(self) -> String;
}

impl Capitalize for String {
  fn capitalize(self) -> String {
    let mut c = self.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }
}

// statless flutter widget
pub fn stateless_widget(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().capitalize();
    return format!(
        "
        import 'package:flutter/material.dart';
        import 'package:auto_route/auto_route.dart';

        @RoutePage()
        class {feature_name}Screen extends StatelessWidget {{
            const {feature_name}Screen({{Key? key}}) : super(key: key);

            @override
            Widget build(BuildContext context) {{
                return Scaffold(
                    appBar: AppBar(
                        title: Text('{feature_name}'),
                    ),
                    body: Center(
                        child: Text('{feature_name}'),
                    ),
                );
            }}
        }}
        "
    ).trim().to_string();
    }

// stateful flutter widget
// pub fn stateful_widget(feature_name: &str) -> String {
//     let feature_name = feature_name.to_string().capitalize();
//     return format!(
//         "
//         import 'package:flutter/material.dart';
//         import 'package:auto_route/auto_route.dart';

//         @RoutePage()
//         class {feature_name}Screen extends StatefulWidget {{
//             const {feature_name}Screen({{Key? key}}) : super(key: key);

//             @override
//             _{feature_name}ScreenState createState() => _{feature_name}ScreenState();
//         }}

//         class _{feature_name}ScreenState extends State<{feature_name}Screen> {{
//             @override
//             Widget build(BuildContext context) {{
//                 return Scaffold(
//                     appBar: AppBar(
//                         title: Text('{feature_name}'),
//                     ),
//                     body: Center(
//                         child: Text('{feature_name}'),
//                     ),
//                 );
//             }}
//         }}
//         "
//     ).trim().to_string();
//     }

// local repository
pub fn local_repository(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        abstract class Abstract{feature_name}LocalRepository {{
            Future<void> save();
        }}

        class {feature_name}LocalRepository implements Abstract{feature_name}LocalRepository{{
            final Ref ref;

            {feature_name}LocalRepository(this.ref);

            @override
            Future<void> save() async => throw UnimplementedError();
        }}

        final {}LocalRepositoryProvider = Provider<{feature_name}LocalRepository>((ref) => {feature_name}LocalRepository(ref));
        
        ",
        feature_name.to_lowercase()
    ).trim().to_string();
    }

// remote repository
pub fn remote_repository(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        abstract class Abstract{feature_name}RemoteRepository {{
            Future<void> save();
        }}

        class {feature_name}RemoteRepository implements Abstract{feature_name}RemoteRepository {{
            final Ref ref;

            {feature_name}RemoteRepository(this.ref);

            @override
            Future<void> save() async => throw UnimplementedError();
        }}

        final {}RemoteRepositoryProvider = Provider<{feature_name}RemoteRepository>((ref) => {feature_name}RemoteRepository(ref));
        
        ",
        feature_name.to_lowercase()
    ).trim().to_string();
    }

// application service
pub fn application_service(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        abstract class Abstract{feature_name}Service {{
            Future<void> save();
        }}

        class {feature_name}Service implements Abstract{feature_name}ApplicationService {{
            final Ref ref;

            {feature_name}Service(this.ref);

            @override
            Future<void> save() async => throw UnimplementedError();
        }}

        final {}ServiceProvider = Provider<{feature_name}Service>((ref) => {feature_name}Service(ref));
        
        ",
        feature_name.to_lowercase()
    ).trim().to_string();
    } 

// controller
pub fn controller(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        class {feature_name}Controller  extends StateNotifier<bool> {{
            final Ref ref;

            {feature_name}Controller(this.ref) : super(true);

            @override
            Future<void> fetchSomething() async => throw UnimplementedError();
        }}

        final {}ControllerProvider = StateNotifierProvider.autoDispose<{feature_name}Controller, bool>((ref) => {feature_name}Controller(ref));
            
        ",
        feature_name.to_lowercase()
    ).trim().to_string();
    }