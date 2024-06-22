import 'package:audio_library_convertor/app_state.dart';
import 'package:audio_library_convertor/ui/elements/ui_elements.dart';
import 'package:flutter/material.dart';

class TranscoderPage extends StatefulWidget {
  const TranscoderPage({super.key});

  @override
  State<TranscoderPage> createState() => _TranscoderPageState();
}

class _TranscoderPageState extends State<TranscoderPage> {
  final TextEditingController _srcPathController = TextEditingController();
  final TextEditingController _destPathController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox(
        width: 450,
        height: MediaQuery.sizeOf(context).height,
        child: Column(
          children: [
            const SizedBox(
              height: 5,
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                SizedBox(
                  width: 300,
                  child: TextFormField(
                    controller: _srcPathController,
                    decoration: InputDecoration(
                      labelText: "Source Directory Path",
                      hintText: "C:\\Path\\To\\Music\\Folder",
                      prefixIcon: const Padding(
                        padding: EdgeInsets.symmetric(horizontal: 16),
                        child: Icon(
                          Icons.folder_open_rounded,
                          size: 21,
                        ),
                      ),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(16),
                      ),
                    ),
                    onChanged: (value) {
                      TranscoderState.getInstance().setSource(value);
                    },
                  ),
                ),
                DiraudioUiElements.diraudiTonalButton(
                  context,
                  "Browse",
                  100,
                  () async {
                    _srcPathController.text =
                        await TranscoderState.getInstance()
                                .setSourcePathViaOS() ??
                            _srcPathController.text;
                  },
                ),
              ],
            ),
            const SizedBox(height: 20),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                SizedBox(
                  width: 300,
                  child: TextFormField(
                    controller: _destPathController,
                    decoration: InputDecoration(
                      labelText: "Destination Directory Path",
                      hintText: "C:\\Path\\To\\Target\\Folder",
                      prefixIcon: const Padding(
                        padding: EdgeInsets.symmetric(horizontal: 16),
                        child: Icon(
                          Icons.folder_open_rounded,
                          size: 21,
                        ),
                      ),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(16),
                      ),
                    ),
                    onChanged: (value) {
                      TranscoderState.getInstance().setDestination(value);
                    },
                  ),
                ),
                DiraudioUiElements.diraudiTonalButton(
                  context,
                  "Browse",
                  100,
                  () async {
                    _destPathController.text =
                        await TranscoderState.getInstance()
                                .setDestinationPathViaOS() ??
                            _destPathController.text;
                  },
                ),
              ],
            ),
            const SizedBox(height: 20),
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 25),
              child: Row(
                children: [
                  Text("Copy Unrecognised Files"),
                ],
              ),
            ),
            const SizedBox(height: 15),
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 25),
              child: Row(
                children: [Text("Target Format: ")],
              ),
            )
          ],
        ),
      ),
    );
  }
}
