import 'dart:io';
import 'package:audio_library_convertor/app_state.dart';
import 'package:audio_library_convertor/messages/dart_signal.pb.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import './messages/generated.dart';

void main() async {
  // set initial state of the app here
  AppState state = AppState.getInstance();
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  TextEditingController _sourceController = TextEditingController();
  TextEditingController _destinationController = TextEditingController();
  final AppState _stateInstance = AppState.getInstance();

  @override
  void initState() {
    // TODO: implement initState
    super.initState();
  }

  int _noOfThreads = 1;
  final int _availableNoOfThreads = Platform.numberOfProcessors;

  void _pickSourceFolder() async {
    String? src = await _stateInstance.setSourcePathViaOS();
    setState(() {
      _sourceController.text = src ?? _sourceController.text;
    });
  }

  void _pickDestinationFolder() async {
    String? dest = await _stateInstance.setDestinationPathViaOS();
    setState(() {
      _destinationController.text = dest ?? _destinationController.text;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      themeMode: ThemeMode.dark,
      home: Scaffold(
        appBar: AppBar(
          title: Text('Folder Picker Example'),
        ),
        body: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            children: <Widget>[
              Row(
                children: <Widget>[
                  Expanded(
                    child: TextField(
                      controller: _sourceController,
                      decoration: const InputDecoration(
                        labelText: 'Source folder',
                      ),
                      onChanged: (value) {},
                    ),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton(
                    onPressed: _pickSourceFolder,
                    child: Text('Open Explorer'),
                  ),
                ],
              ),
              SizedBox(height: 16),
              Row(
                children: <Widget>[
                  Expanded(
                    child: TextField(
                      controller: _destinationController,
                      decoration: InputDecoration(
                        labelText: 'Destination folder',
                      ),
                    ),
                  ),
                  SizedBox(width: 8),
                  ElevatedButton(
                    onPressed: _pickDestinationFolder,
                    child: Text('Open Explorer'),
                  ),
                ],
              ),
              FilledButton(
                onPressed: () async {
                  bool result =
                      await _stateInstance.startConversion(TargetFormat.Mp3);
                  print(result);
                },
                child: Text("convert"),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
