#![allow(dead_code, unused)]

pub trait StringTrait {
  fn capitalize(self) -> String;
  fn to_pascal_case(self) -> String;
}

impl StringTrait for String {
  fn capitalize(self) -> String {
    let mut c = self.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }

fn to_pascal_case(self) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in self.chars() {
      if c == '_' {
        capitalize_next = true;
      } else if capitalize_next {
        result.push(c.to_ascii_uppercase());
        capitalize_next = false;
      } else {
        result.push(c);
      }
    }
    result
    }
}

// main.dart
pub fn main_dart() -> String {
    return "
        import 'dart:io';

        import 'package:flutter/foundation.dart';
        import 'package:flutter/material.dart';
        import 'package:flutter_riverpod/flutter_riverpod.dart';
        import 'package:flutter_dotenv/flutter_dotenv.dart';

        import 'src/app.dart';
        import 'src/services/logger_service.dart';

        void main() async {
            WidgetsFlutterBinding.ensureInitialized();

            await dotenv.load(fileName: '.env');

            final container = ProviderContainer(overrides: [
                loggerServiceProvider.overrideWith((ref) => LoggerService(ref)),
            ], observers: [
                AsyncErrorLogger()
            ]);

            await container.read(loggerServiceProvider).init();

            _registerErrorHandlers(container.read(loggerServiceProvider));

            try {
                // ignore: missing_provider_scope
                runApp(UncontrolledProviderScope(container: container, child: const MyApp()));
            } catch (error, stack) {
                container.read(loggerServiceProvider).critical(error: error, stackTrace: stack, methodName: 'main');
                if (kDebugMode) return;
                exit(0);
            }
        }

        void _registerErrorHandlers(LoggerService loggerService) {
            //* displays custom error widget if exception occurs within build method.
            ErrorWidget.builder = (FlutterErrorDetails errorDetails) {
                loggerService.critical(error: errorDetails.exception, stackTrace: errorDetails.stack, methodName: 'ErrorWidget.builder');
                return Scaffold(
                    body: Center(
                        child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                                const Text('Oops! Something went wrong.'),
                                Text(errorDetails.exception.toString())
                            ],
                        )
                    ),
                );
            };

            //* handles all unhandled flutter framework exceptions.
            FlutterError.onError = (FlutterErrorDetails details) {
                loggerService.critical(error: details.exception, stackTrace: details.stack, methodName: 'FlutterError.onError');
                if (kDebugMode) return;
                exit(0);
            };

            // * Handle errors from the underlying platform/OS
            PlatformDispatcher.instance.onError = (Object error, StackTrace stack) {
                loggerService.critical(error: error, stackTrace: stack, methodName: 'PlatformDispatcher.instance.onError');
                return true;
            };
        }

    ".trim().to_string();
}

// app.dart
pub fn app_dart() -> String {
    return "
        import 'package:flutter/material.dart';
        import 'package:flutter_riverpod/flutter_riverpod.dart';
        import 'package:responsive_framework/responsive_framework.dart';

        import 'utils/routes.dart';

        class MyApp extends ConsumerStatefulWidget {
            const MyApp({super.key});

            @override
            ConsumerState<MyApp> createState() => _MyAppState();
        }

        class _MyAppState extends ConsumerState<MyApp> {
            late final AppRouter _appRouter;

            @override
            void initState() {
                _appRouter = AppRouter(ref);
                super.initState();
            }

            @override
            Widget build(BuildContext context) {
                return SafeArea(
                    child: MaterialApp.router(
                    title: 'Flutter Demo',
                    debugShowCheckedModeBanner: false,
                    theme: ThemeData(
                        useMaterial3: true,
                        appBarTheme: const AppBarTheme(backgroundColor: Color.fromARGB(255, 80, 66, 105), centerTitle: true, toolbarHeight: 40, foregroundColor: Colors.white),
                        elevatedButtonTheme: ElevatedButtonThemeData(
                        style: ElevatedButton.styleFrom(
                            foregroundColor: Colors.white,
                            backgroundColor: Colors.grey,
                        ),
                        ),
                        inputDecorationTheme: const InputDecorationTheme(
                        errorStyle: TextStyle(color: Colors.red, fontSize: 12.0, fontWeight: FontWeight.bold, fontStyle: FontStyle.italic),
                        ),
                    ),
                    routerConfig: _appRouter.config(navigatorObservers: () => [MyRouteObserver()]),
                    builder: (context, child) => ResponsiveBreakpoints.builder(
                            child: child!,
                            breakpoints: [
                            const Breakpoint(start: 0, end: 450, name: MOBILE),
                            const Breakpoint(start: 451, end: 800, name: TABLET),
                            const Breakpoint(start: 801, end: 1920, name: DESKTOP),
                            const Breakpoint(start: 1921, end: double.infinity, name: '4K'),
                            ],
                        ),
                    ),
                );
            }
        }
    ".trim().to_string();
}

// routes.dart
pub fn routes_dart() -> String {
    return "
        import 'dart:developer';

        import 'package:auto_route/auto_route.dart';
        import 'package:flutter/material.dart';
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        import 'routes.gr.dart';

        @AutoRouterConfig(replaceInRouteName: 'Screen,Route')
        class AppRouter extends $AppRouter implements AutoRouteGuard {
            final WidgetRef ref;

            AppRouter(this.ref);

            @override
            RouteType get defaultRouteType => const RouteType.custom(
                    transitionsBuilder: TransitionsBuilders.fadeIn,
                    durationInMilliseconds: 400,
                );

            @override
            List<AutoRoute> get routes => [
                    AutoRoute(
                    path: '/roster',
                    initial: true,
                    page: EmptyRoutePage.page,
                    children: [
                        AutoRoute(
                        path: '',
                        page: RosterRoute.page,
                        ),
                        AutoRoute(
                        path: 'documents/:patientId',
                        page: DocumentsShellRoute.page,
                        children: [
                            AutoRoute(
                            path: '',
                            page: DocumentsRoute.page,
                            ),
                        ],
                        ),
                    ],
                    ),
                ];

            @override
            void onNavigation(NavigationResolver resolver, StackRouter router) {
                bool isAuthenticated = true;
                log('isAuthenticated: $isAuthenticated');
                if (isAuthenticated || resolver.route.name == 'login') {
                    // we continue navigation
                    resolver.next();
                } else {
                    // else we navigate to the Login page so we get authenticateed
                    push(const AuthenticationRoute());
                }
            }
        }

        class MyRouteObserver extends AutoRouterObserver {
            @override
            void didPush(Route route, Route? previousRoute) {
                log('Route was pushed: ${route.settings.name}');
            }

            @override
            void didPop(Route route, Route? previousRoute) {
                log('Route was popped: ${route.settings.name}');
            }

            @override
            void didRemove(Route route, Route? previousRoute) {
                log('Route was removed: ${route.settings.name}');
            }

            @override
            void didReplace({Route? newRoute, Route? oldRoute}) {
                log('Route was replaced: ${newRoute?.settings.name}');
            }
        }

        // Filler page to allow for a root route to have children
        @RoutePage()
        class EmptyPageRoute extends AutoRouter {
            const EmptyPageRoute({
                super.key,
            });
        }
        
        // Another filler page with different name. This example shows a pathParam annotation.
        // In this example, the DocumentsRoute() accepts an inherited path param of patientId.
        @RoutePage()
        class DocumentsShellScreen extends AutoRouter {
            const DocumentsShellScreen({
                super.key,
                @pathParam required int patientId,
            });
        }

        // @RoutePage()
        // class DocumentsScreen extends StatelessWidget {
        //     final int patientId;

        //     const DocumentsScreen({super.key,  @PathParam.inherit() required this.patientId});

        //     @override
        //     Widget build(BuildContext context) {
        //         return Scaffold(
        //         appBar: AppBar(
        //             title: const Text('Documents'),
        //             leading: const AutoLeadingButton(),
        //         ),
        //         body: Center(
        //             child: Text('Document id $patientId'),
        //         ),
        //         );
        //     }
        // }

    ".trim().to_string();
}

// extensions.dart
pub fn extensions_dart() -> String {
    return "
        extension StringExtension on String {
            String capitalize() {
                return '${this[0].toUpperCase()}${substring(1)}';
            }
        }
    ".trim().to_string();
}

// statless flutter widget
pub fn stateless_widget(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
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
                        title: const Text('{feature_name}'),
                    ),
                    body: const Center(
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
//     let feature_name = feature_name.to_string().to_pascal_case().capitalize();
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
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
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
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
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

// local repository
pub fn fake_local_repository(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        abstract class AbstractFake{feature_name}LocalRepository {{
            Future<void> save();
        }}

        class Fake{feature_name}LocalRepository implements AbstractFake{feature_name}LocalRepository{{
            final Ref ref;

            Fake{feature_name}LocalRepository(this.ref);

            @override
            Future<void> save() async => throw UnimplementedError();
        }}

        final fake{feature_name}LocalRepositoryProvider = Provider<Fake{feature_name}LocalRepository>((ref) => Fake{feature_name}LocalRepository(ref));
        
        "
        
    ).trim().to_string();
    }

// remote repository
pub fn fake_remote_repository(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        abstract class AbstractFake{feature_name}RemoteRepository {{
            Future<void> save();
        }}

        class Fake{feature_name}RemoteRepository implements AbstractFake{feature_name}RemoteRepository {{
            final Ref ref;

            Fake{feature_name}RemoteRepository(this.ref);

            @override
            Future<void> save() async => throw UnimplementedError();
        }}

        final fake{feature_name}RemoteRepositoryProvider = Provider<Fake{feature_name}RemoteRepository>((ref) => Fake{feature_name}RemoteRepository(ref));
        
        "
    ).trim().to_string();
    }

// application service
pub fn application_service(feature_name: &str) -> String {
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        abstract class Abstract{feature_name}Service {{
            Future<void> save();
        }}

        class {feature_name}Service implements Abstract{feature_name}Service {{
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
    let feature_name = feature_name.to_string().to_pascal_case().capitalize();
    return format!(
        "
        import 'package:flutter_riverpod/flutter_riverpod.dart';

        class {feature_name}Controller  extends StateNotifier<bool> {{
            final Ref ref;

            {feature_name}Controller(this.ref) : super(true);

            Future<void> fetchSomething() async => throw UnimplementedError();
        }}

        final {}ControllerProvider = StateNotifierProvider.autoDispose<{feature_name}Controller, bool>((ref) => {feature_name}Controller(ref));
            
        ",
        feature_name.to_lowercase()
    ).trim().to_string();
    }

// logger service
pub fn logger_service() -> String {
    return "
    
    import 'package:logger/logger.dart';
    import 'package:flutter_riverpod/flutter_riverpod.dart';


    final loggerServiceProvider = Provider<LoggerService>((ref) => throw UnimplementedError());

            class LoggerService extends Logger {
                final Ref ref;

                LoggerService(this.ref)
                    : super(
                            printer: PrettyPrinter(
                                methodCount: 0, // number of method calls to be displayed
                                errorMethodCount: 3, // number of method calls if stacktrace is provided
                                lineLength: 50, // width of the output
                                colors: true, // Colorful log messages
                                printEmojis: true, // Print an emoji for each log message
                                printTime: false // Should each log print contain a timestamp
                                ));

                Future<LoggerService> init() async {
                    verbose('LoggerService initialized');
                    return this;
                }

                void verbose(String text) => v(text);

                void info(String text) => i(text);

                void debug(String text) => d(text);

                void warning({required Object warning, required String methodName, StackTrace? stackTrace, Object? instance, bool sendToServer = true}) async {
                    w('Warning', warning, stackTrace);
                }

                void error({required Object error, required String methodName, StackTrace? stackTrace, Object? instance, bool sendToServer = true}) async {
                    e('Error', error, stackTrace);
                }

                void critical({required Object error, required String methodName, StackTrace? stackTrace, Object? instance, bool sendToServer = true}) async {
                    wtf('Critical Failure', error, stackTrace);
                }
        }

        class RiverPodLogger extends ProviderObserver {
            @override
            void didUpdateProvider(ProviderBase provider, Object? previousValue, Object? newValue, ProviderContainer container) {
                //
            }
        }
            ".trim().to_string();
        
    }

    // async_errors.dart
    pub fn async_errors() -> String {
        return "
            import '../app_exceptions.dart';
            import 'package:flutter_riverpod/flutter_riverpod.dart';

            class AsyncErrorLogger extends ProviderObserver {

                @override
                void didUpdateProvider(
                    ProviderBase provider,
                    Object? previousValue,
                    Object? newValue,
                    ProviderContainer container,
                ) {
                    final logger = container.read(loggerServiceProvider);
                    final error = _findError(newValue);
                    if (error != null) {
                        if (error.error is AppException) {
                            logger.error(error: error.error as AppException, stackTrace: error.stackTrace, methodName: 'AsyncErrorLogger', sendToServer: false);

                            // only prints the AppException data
                            // errorLogger.logAppException(error.error as AppException);
                        } else {
                            logger.error(error: error.error, stackTrace: error.stackTrace, methodName: 'AsyncErrorLogger', sendToServer: true);
                        }
                    }
                }

                AsyncError<dynamic>? _findError(Object? value) {
                    if (value is AsyncError) {
                        return value;
                    } else {
                        return null;
                    }
                }
            }
        ".trim().to_string();
    }

    // app_exceptions.dart
    pub fn app_exceptions() -> String {
        return "
            import 'package:freezed_annotation/freezed_annotation.dart';
            part 'app_exceptions.freezed.dart';

            @freezed
            class AppException with _$AppException {
                const factory AppException.invalidUsernamePassword(Object e) = InvalidUsernamePassword;
            }

            extension AppExceptionMessages on AppException {
                String get message {
                    return when(
                        invalidUsernamePassword: () => 'Invalid username and/or password.',
                    );
                }
                }
        ".trim().to_string();
    }

    // app_sizes.dart
    pub fn app_sizes() -> String {
        return "
            import 'package:flutter/material.dart';

            /// Constant sizes to be used in the app (paddings, gaps, rounded corners etc.)
            class Sizes {
            static const p4 = 4.0;
            static const p8 = 8.0;
            static const p12 = 12.0;
            static const p16 = 16.0;
            static const p20 = 20.0;
            static const p24 = 24.0;
            static const p32 = 32.0;
            static const p48 = 48.0;
            static const p64 = 64.0;
            }

            /// Constant gap widths
            const gapW4 = SizedBox(width: Sizes.p4);
            const gapW8 = SizedBox(width: Sizes.p8);
            const gapW12 = SizedBox(width: Sizes.p12);
            const gapW16 = SizedBox(width: Sizes.p16);
            const gapW20 = SizedBox(width: Sizes.p20);
            const gapW24 = SizedBox(width: Sizes.p24);
            const gapW32 = SizedBox(width: Sizes.p32);
            const gapW48 = SizedBox(width: Sizes.p48);
            const gapW64 = SizedBox(width: Sizes.p64);

            /// Constant gap heights
            const gapH4 = SizedBox(height: Sizes.p4);
            const gapH8 = SizedBox(height: Sizes.p8);
            const gapH12 = SizedBox(height: Sizes.p12);
            const gapH16 = SizedBox(height: Sizes.p16);
            const gapH20 = SizedBox(height: Sizes.p20);
            const gapH24 = SizedBox(height: Sizes.p24);
            const gapH32 = SizedBox(height: Sizes.p32);
            const gapH48 = SizedBox(height: Sizes.p48);
            const gapH64 = SizedBox(height: Sizes.p64);
        ".trim().to_string();
    }

    // app_colors.dart
    pub fn app_colors() -> String {
        return "
            import 'package:flutter/material.dart';
            
            /// App colors
            abstract class AppColors {
                static const darkBlue = Color(0xFF555E82);
            }
        ".trim().to_string();
    }

    // app_text_styles.dart
    pub fn app_text_styles() -> String {
        return "
            import 'package:flutter/material.dart';

            /// App TextStyles
            abstract class AppTextStyles {
                // Body styles /////////////////////////////////////////

                /// fontSize: 16 fontWeight: FontWeight.w400 letterSpacing: 0.5
                static const b1 = TextStyle(fontSize: 16, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.5);

                /// fontSize: 15 fontWeight: FontWeight.w400 letterSpacing: 0.5
                static const b2 = TextStyle(fontSize: 15, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.5);

                /// fontSize: 14 fontWeight: FontWeight.w400 letterSpacing: 0.25
                static const b3 = TextStyle(fontSize: 14, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.25);

                /// fontSize: 13 fontWeight: FontWeight.w400 letterSpacing: 0.25
                static const b4 = TextStyle(fontSize: 13, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.25);

                /// fontSize: 12 fontWeight: FontWeight.w400 letterSpacing: 0.25
                static const b5 = TextStyle(fontSize: 12, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.25);

                // Header styles /////////////////////////////////////////

                /// fontSize: 24 fontWeight: FontWeight.w300 letterSpacing: 0.0
                static const h1 = TextStyle(fontSize: 24, color: AppColors.kText, fontWeight: FontWeight.w300, letterSpacing: 0.0);

                /// fontSize: 23 fontWeight: FontWeight.w300 letterSpacing: 0.0
                static const h2 = TextStyle(fontSize: 23, color: AppColors.kText, fontWeight: FontWeight.w300, letterSpacing: 0.0);

                /// fontSize: 22 fontWeight: FontWeight.w400 letterSpacing: 0.0
                static const h3 = TextStyle(fontSize: 22, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.0);

                /// fontSize: 21 fontWeight: FontWeight.w400 letterSpacing: 0.15
                static const h4 = TextStyle(fontSize: 21, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.15);

                /// fontSize: 20 fontWeight: FontWeight.w400 letterSpacing: 0.15
                static const h5 = TextStyle(fontSize: 20, color: AppColors.kText, fontWeight: FontWeight.w400, letterSpacing: 0.15);

                /// fontSize: 19 fontWeight: FontWeight.w500 letterSpacing: 0.15
                static const h6 = TextStyle(fontSize: 19, color: AppColors.kText, fontWeight: FontWeight.w500, letterSpacing: 0.15);
            }
        ".trim().to_string();
    }

    // helper.dart
    pub fn helper_dart() -> String {
        return "
            import 'dart:math';

            abstract class Helper {
                static int getRandomNumber(int min, int max) {
                    final random = Random();
                    return min + random.nextInt(max - min);
                }
            }
        ".trim().to_string();
    }