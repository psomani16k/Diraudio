import 'package:audio_library_convertor/ui/transcoder_page/transcoder_page.dart';
import 'package:flutter/material.dart';

class ProgressUpdateProcessor extends StatefulWidget {
  const ProgressUpdateProcessor({super.key});

  @override
  State<ProgressUpdateProcessor> createState() =>
      _ProgressUpdateProcessorState();
}

class _ProgressUpdateProcessorState extends State<ProgressUpdateProcessor> {
  // int _navigationIndex = 0;
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Diraudio"),
      ),
      body: const TranscoderPage(),
      // drawer: NavigationRail(
      //   destinations: const [
      //     NavigationRailDestination(
      //       icon: Icon(Icons.change_circle_outlined),
      //       selectedIcon: Icon(Icons.change_circle_outlined),
      //       label: Text("Transcode"),
      //     )
      //   ],
      //   selectedIndex: _navigationIndex,
      // ),
    );
  }
}
