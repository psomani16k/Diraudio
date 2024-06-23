import 'dart:io';
import 'package:audio_library_convertor/app_state.dart';
import 'package:audio_library_convertor/messages/dart_signal.pb.dart';
import 'package:audio_library_convertor/ui/elements/ui_elements.dart';
import 'package:audio_library_convertor/ui/transcoder_page/mp3_config/mp3_config.dart';

import 'package:flutter/material.dart';

class TranscoderPage extends StatefulWidget {
  const TranscoderPage({super.key});

  @override
  State<TranscoderPage> createState() => _TranscoderPageState();
}

class _TranscoderPageState extends State<TranscoderPage> {
  final TextEditingController _srcPathController = TextEditingController();
  final TextEditingController _destPathController = TextEditingController();
  bool _copyUnrecognisedFiles =
      TranscoderState.getInstance().getCopyUnrecognisedFiles();
  TargetFormat _targetFormat = TargetFormat.Mp3;
  int _threadCount = TranscoderState.getInstance().getNoOfThreads();

  final Map<TargetFormat, ({Widget ui, double height})> _audioConfigUiData = {
    TargetFormat.Mp3: (ui: const Mp3ConfigUi(), height: Mp3ConfigUi.height),
  };

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox(
        width: 450,
        height: MediaQuery.sizeOf(context).height,
        child: SingleChildScrollView(
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
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 25),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text("Copy Unrecognised Files"),
                    Switch(
                      value: _copyUnrecognisedFiles,
                      onChanged: (value) {
                        setState(() {
                          _copyUnrecognisedFiles = value;
                        });
                        TranscoderState.getInstance()
                            .setCopyUnrecognisedFiles(_copyUnrecognisedFiles);
                      },
                    )
                  ],
                ),
              ),
              const SizedBox(height: 15),
              Padding(
                padding: const EdgeInsets.fromLTRB(25, 0, 0, 0),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text("Target Format:"),
                    Container(
                      decoration: BoxDecoration(
                          border: Border.all(
                            width: 2,
                            color: Theme.of(context)
                                .colorScheme
                                .secondaryContainer,
                          ),
                          borderRadius: BorderRadius.circular(16)),
                      width: 100,
                      child: Center(
                        child: DropdownButton<TargetFormat>(
                          isExpanded: true,
                          padding: const EdgeInsets.fromLTRB(16, 0, 16, 0),
                          underline: const SizedBox(),
                          focusColor: Theme.of(context).colorScheme.background,
                          items: TargetFormat.values.map((e) {
                            return DropdownMenuItem(
                                value: e, child: Text(e.name));
                          }).toList(),
                          onChanged: (value) {
                            setState(() {
                              _targetFormat = value ?? _targetFormat;
                            });
                            TranscoderState.getInstance()
                                .setTargetFormat(_targetFormat);
                          },
                          value: _targetFormat,
                          borderRadius: BorderRadius.circular(16),
                        ),
                      ),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 15),
              Padding(
                padding: const EdgeInsets.only(left: 25),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text("Number of threads:"),
                    SizedBox(
                      width: 100,
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          IconButton(
                            icon: const Icon(Icons.arrow_drop_up_rounded),
                            onPressed: () {
                              if (_threadCount < Platform.numberOfProcessors) {
                                setState(() {
                                  _threadCount++;
                                });
                                TranscoderState.getInstance()
                                    .setNoOfThreads(_threadCount);
                              }
                            },
                          ),
                          Text(_threadCount.toString()),
                          IconButton(
                            icon: const Icon(Icons.arrow_drop_down_rounded),
                            onPressed: () {
                              if (_threadCount > 1) {
                                setState(() {
                                  _threadCount--;
                                });
                                TranscoderState.getInstance()
                                    .setNoOfThreads(_threadCount);
                              }
                            },
                          ),
                        ],
                      ),
                    )
                  ],
                ),
              ),
              const SizedBox(height: 25),
              const Divider(
                endIndent: 30,
                indent: 30,
                thickness: 2,
                height: 2,
              ),
              const SizedBox(height: 20),
              AnimatedContainer(
                duration: Durations.medium3,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(32),
                  color: Theme.of(context).colorScheme.tertiaryContainer,
                ),
                width: 450,
                height: _audioConfigUiData[_targetFormat]?.height,
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: _audioConfigUiData[_targetFormat]?.ui,
                ),
              )
            ],
          ),
        ),
      ),
    );
  }
}
