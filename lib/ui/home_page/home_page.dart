import 'package:audio_library_convertor/app_state.dart';
import 'package:audio_library_convertor/ui/elements/ui_elements.dart';
import 'package:audio_library_convertor/ui/progress_update_processor/progress_update_processor.dart';
import 'package:audio_library_convertor/ui/transcoder_page/transcoder_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  // int _navigationIndex = 0;
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Diraudio"),
      ),
      body: Stack(
        children: [
          TranscoderPage(),
          SizedBox(
              height: MediaQuery.sizeOf(context).height,
              width: MediaQuery.sizeOf(context).width,
              child: ProgressUpdateProcessor()),
        ],
      ),
    );
  }
}
