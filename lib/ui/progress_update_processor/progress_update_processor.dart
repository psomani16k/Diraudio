import 'package:audio_library_convertor/app_state.dart';
import 'package:audio_library_convertor/messages/rust_signal.pb.dart';
import 'package:audio_library_convertor/ui/elements/ui_elements.dart';
import 'package:flutter/material.dart';

class ProgressUpdateProcessor extends StatefulWidget {
  const ProgressUpdateProcessor({super.key});

  @override
  State<ProgressUpdateProcessor> createState() =>
      _ProgressUpdateProcessorState();
}

class _ProgressUpdateProcessorState extends State<ProgressUpdateProcessor> {
  final List<ProgressUpdate> _progressHistory = [];
  bool _showingUpdates = false;

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
    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        Container(
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.secondaryContainer,
            borderRadius: const BorderRadius.only(
              topLeft: Radius.circular(36),
              topRight: Radius.circular(36),
            ),
          ),
          height: 80,
          width: MediaQuery.sizeOf(context).width,
          child: Align(
            alignment: Alignment.centerRight,
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: DiraudioUiElements.diraudioFIlledButton(
                context,
                "Convert",
                120,
                () {
                  //TranscoderState.getInstance().startConversion();
                  setState(() {
                    _showingUpdates = !_showingUpdates;
                  });
                },
              ),
            ),
          ),
        ),
        AnimatedContainer(
          duration: Durations.extralong1,
          curve: Easing.emphasizedDecelerate,
          height: _showingUpdates
              ? (MediaQuery.sizeOf(context).height -
                  80 -
                  (Theme.of(context).appBarTheme.toolbarHeight ?? 56))
              : 0,
        ),
      ],
    );
  }
}
