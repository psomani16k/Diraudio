import 'package:audio_library_convertor/messages/rust_signal.pb.dart';
import 'package:audio_library_convertor/ui/elements/ui_elements.dart';
import 'package:audio_library_convertor/ui/progress_update_processor/update_processor_box/transcoder_conbined_update_processor.dart';
import 'package:flutter/material.dart';

class ProgressUpdateProcessor extends StatefulWidget {
  const ProgressUpdateProcessor({super.key});

  @override
  State<ProgressUpdateProcessor> createState() =>
      _ProgressUpdateProcessorState();
}

class _ProgressUpdateProcessorState extends State<ProgressUpdateProcessor> {
  bool _showingUpdates = false;
  bool _sourceHasFiles = false;
  int _numberOfFilesFound = 0;

  Widget _sourcePathStatusMessage(String message) {
    Color errorOnContainerColor =
        Theme.of(context).colorScheme.onErrorContainer;
    HSLColor notErrorOnContainer = HSLColor.fromColor(errorOnContainerColor);
    Color notErrorOnContainerColor = notErrorOnContainer
        .withHue((notErrorOnContainer.hue + 120) % 360)
        .toColor();

    Color errorContainerColor = Theme.of(context).colorScheme.errorContainer;
    HSLColor notErrorContainer = HSLColor.fromColor(errorContainerColor);
    Color notErrorContainerColor = notErrorContainer
        .withHue((notErrorContainer.hue + 120) % 360)
        .toColor();
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Container(
        width: 310,
        height: 40,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(14),
          color: _sourceHasFiles ? notErrorContainerColor : errorContainerColor,
        ),
        child: Row(
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: Icon(
                _sourceHasFiles
                    ? Icons.check_circle_outline
                    : Icons.error_outline,
                color: _sourceHasFiles
                    ? notErrorOnContainerColor
                    : errorOnContainerColor,
                size: 18,
              ),
            ),
            Text(
              message,
              style: TextStyle(
                color: _sourceHasFiles
                    ? notErrorOnContainerColor
                    : errorOnContainerColor,
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        AnimatedContainer(
          duration: Durations.extralong1,
          curve: Easing.emphasizedDecelerate,
          decoration: BoxDecoration(
            color: _showingUpdates
                ? Theme.of(context).colorScheme.background
                : Theme.of(context).colorScheme.secondaryContainer,
            borderRadius: BorderRadius.only(
              topLeft: _showingUpdates
                  ? const Radius.circular(0)
                  : const Radius.circular(36),
              topRight: _showingUpdates
                  ? const Radius.circular(0)
                  : const Radius.circular(36),
            ),
          ),
          height: 80,
          width: MediaQuery.sizeOf(context).width,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              StreamBuilder(
                stream: TotalNumberOfFilesFound.rustSignalStream,
                builder: (context, snapshot) {
                  if (snapshot.hasData) {
                    var data = snapshot.data!.message;
                    _numberOfFilesFound = data.number;
                    _sourceHasFiles = data.filesFound;
                  }
                  String messageToShow = _sourceHasFiles
                      ? "Found $_numberOfFilesFound files in source directory"
                      : "Please select a valid source directory";
                  return _sourcePathStatusMessage(messageToShow);
                },
              ),
              Padding(
                padding: const EdgeInsets.all(16),
                child: DiraudioUiElements.diraudioFIlledButton(
                  context,
                  AnimatedCrossFade(
                    firstChild: const Text("Convert"),
                    secondChild: const Text("Cancel"),
                    crossFadeState: _showingUpdates
                        ? CrossFadeState.showSecond
                        : CrossFadeState.showFirst,
                    duration: Durations.medium3,
                  ),
                  120,
                  () {
                    //TranscoderState.getInstance().startConversion();
                    setState(() {
                      _showingUpdates = !_showingUpdates;
                    });
                  },
                ),
              ),
            ],
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
          decoration: BoxDecoration(
            color: _showingUpdates
                ? Theme.of(context).colorScheme.background
                : Theme.of(context).colorScheme.secondaryContainer,
          ),
          child: TranscoderCombinedUpdateProcessor(),
        ),
      ],
    );
  }
}
