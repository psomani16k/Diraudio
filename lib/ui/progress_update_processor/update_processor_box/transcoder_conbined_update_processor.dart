import 'package:audio_library_convertor/messages/rust_signal.pb.dart';
import 'package:flutter/material.dart';

class TranscoderCombinedUpdateProcessor extends StatefulWidget {
  const TranscoderCombinedUpdateProcessor({super.key});

  @override
  State<TranscoderCombinedUpdateProcessor> createState() =>
      _TranscoderCombinedUpdateProcessorState();
}

List<ProgressUpdate> _updateHistory = [];

class _TranscoderCombinedUpdateProcessorState
    extends State<TranscoderCombinedUpdateProcessor> {
  Text _processSingleProgressUpdate(ProgressUpdate progressUpdate) {
    return Text(
      "Thread ${progressUpdate.handlingThread}: ${progressUpdate.msg}",
      style: TextStyle(
        color: (progressUpdate.messageType == MessageType.Fail)
            ? Theme.of(context).colorScheme.error
            : Theme.of(context).colorScheme.onTertiaryContainer,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(30, 0, 30, 30),
      child: Container(
        height: MediaQuery.sizeOf(context).height,
        width: MediaQuery.sizeOf(context).width,
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.tertiaryContainer,
          borderRadius: BorderRadius.circular(30),
        ),
        child: StreamBuilder(
          stream: ProgressUpdate.rustSignalStream,
          builder: (context, snapshot) {
            if (snapshot.hasData) {
              _updateHistory.add(snapshot.data!.message);
            }
            return SingleChildScrollView(
              child: Column(
                  children: _updateHistory.map((e) {
                return _processSingleProgressUpdate(e);
              }).toList()),
            );
          },
        ),
      ),
    );
  }
}