import 'package:enigo_flutter_example/channel.dart';
import 'package:flutter/material.dart';
import 'package:enigo_flutter/enigo_flutter.dart';
import 'package:flutter/services.dart';
import "package:window_manager/window_manager.dart";
import "package:hotkey_manager/hotkey_manager.dart";

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();

  windowManager.waitUntilReadyToShow(
    const WindowOptions(
      size: Size(200, 300),
      minimumSize: Size(200, 300),
      maximumSize: Size(200, 300),
      skipTaskbar: true,
    ),
    () async {
      await windowManager.show();
      await windowManager.focus();
    },
  );


  await hotKeyManager.unregisterAll();

  await RustLib.init();

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: Home(),
    );
  }
}

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<Home> createState() => HomeState();
}

class HomeState extends State<Home> {
  String _text = "text";
  String _appName = "name";

  @override
  void initState() {
    hotKeyManager.register(
      HotKey(
        key: PhysicalKeyboardKey.keyA,
        modifiers: [HotKeyModifier.shift, HotKeyModifier.alt],
      ),
      keyDownHandler: (HotKey hotKey) async {
        if (await windowManager.isVisible()) {
          await windowManager.hide();
        } else {
          _appName = await WUtil.recordTopWindow();
          final (x, y) = enigo.location();
          await windowManager.setPosition(Offset(x.toDouble(), y.toDouble()));
          await windowManager.show();
          setState(() {});
        }
      },
    );
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        toolbarHeight: 42,
        title: Text(_appName),
      ),
      body: Center(
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.all(12),
              child: TextField(
                onChanged: (value) {
                  _text = value;
                },
              ),
            ),
            TextButton(
              onPressed: () async {
                await windowManager.blur();
                await windowManager.hide();
                if (await WUtil.setTopWindow()) {
                  enigo.text(text: _text);
                }
              },
              child: const Text("模拟输入"),
            )
          ],
        ),
      ),
    );
  }
}
