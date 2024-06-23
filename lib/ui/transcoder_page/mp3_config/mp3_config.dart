import 'package:audio_library_convertor/app_state.dart';
import 'package:audio_library_convertor/messages/dart_signal.pb.dart';
import 'package:flutter/material.dart';

class Mp3ConfigUi extends StatefulWidget {
  const Mp3ConfigUi({super.key});
  static const double height = 190;
  @override
  State<Mp3ConfigUi> createState() => _Mp3ConfigUiState();
}

class _Mp3ConfigUiState extends State<Mp3ConfigUi> {
  Mp3Bitrate _bitrate = TranscoderState.getInstance().getMp3Config().bitrate;
  Mp3Quality _quality = TranscoderState.getInstance().getMp3Config().quality;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: Mp3ConfigUi.height,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          const Text("Mp3 Output Settings"),
          const SizedBox(height: 10),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text("Output Quality:"),
              Container(
                decoration: BoxDecoration(
                    border: Border.all(
                      width: 2,
                      color: Theme.of(context).colorScheme.onTertiaryContainer,
                    ),
                    borderRadius: BorderRadius.circular(16)),
                width: 150,
                child: Center(
                  child: DropdownButton<Mp3Quality>(
                    isExpanded: true,
                    padding: const EdgeInsets.fromLTRB(16, 0, 16, 0),
                    underline: const SizedBox(),
                    focusColor: Theme.of(context).colorScheme.background,
                    items: Mp3Quality.values.map((e) {
                      return DropdownMenuItem(value: e, child: Text(e.name));
                    }).toList(),
                    onChanged: (value) {
                      setState(() {
                        _quality = value ?? _quality;
                      });
                      TranscoderState.getInstance().setMp3Quality(_quality);
                    },
                    value: _quality,
                    borderRadius: BorderRadius.circular(16),
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text("Output Bitrate:"),
              Container(
                decoration: BoxDecoration(
                    border: Border.all(
                      width: 2,
                      color: Theme.of(context).colorScheme.onTertiaryContainer,
                    ),
                    borderRadius: BorderRadius.circular(16)),
                width: 150,
                child: Center(
                  child: DropdownButton<Mp3Bitrate>(
                    isExpanded: true,
                    padding: const EdgeInsets.fromLTRB(16, 0, 16, 0),
                    underline: const SizedBox(),
                    focusColor: Theme.of(context).colorScheme.background,
                    items: Mp3Bitrate.values.map((e) {
                      return DropdownMenuItem(value: e, child: Text(e.name));
                    }).toList(),
                    onChanged: (value) {
                      setState(() {
                        _bitrate = value ?? _bitrate;
                      });
                      TranscoderState.getInstance().setMp3Bitrate(_bitrate);
                    },
                    value: _bitrate,
                    borderRadius: BorderRadius.circular(16),
                  ),
                ),
              ),
            ],
          )
        ],
      ),
    );
  }
}
