import 'package:audio_lib/ui/file_transcoding/progress_update_processor/progress_update_processor.dart';
import 'package:audio_lib/ui/file_transcoding/transcoder_page/transcoder_page.dart';
import 'package:flutter/material.dart';

class FileTranscoding extends StatelessWidget {
  const FileTranscoding({super.key});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        const TranscoderPage(),
        SizedBox(
          height: MediaQuery.sizeOf(context).height,
          width: MediaQuery.sizeOf(context).width,
          child: const ProgressUpdateProcessor(),
        ),
      ],
    );
  }
}
