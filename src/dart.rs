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

        class {feature_name} extends StatelessWidget {{
            const {feature_name}({{Key? key}}) : super(key: key);

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

//         class {feature_name} extends StatefulWidget {{
//             const {feature_name}({{Key? key}}) : super(key: key);

//             @override
//             _{feature_name}State createState() => _{feature_name}State();
//         }}

//         class _{feature_name}State extends State<{feature_name}> {{
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