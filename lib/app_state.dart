import 'dart:io';
import 'package:audio_lib/messages/dart_signal.pb.dart';
import 'package:audio_lib/messages/dart_signal.pbserver.dart';
import 'package:file_picker/file_picker.dart';

class TranscoderState {
  final Mp3Config _mp3config =
      Mp3Config(bitrate: Mp3Bitrate.Kbps320, quality: Mp3Quality.Best);
  TargetFormat _targetFormat = TargetFormat.Mp3;
  int _noOfThreads = Platform.numberOfProcessors;
  bool _copyUnrecognisedFiles = true;
  String _srcPath = "";
  String _destPath = "";

  // singleton class features
  static final TranscoderState _instance = TranscoderState._();
  TranscoderState._();

  /// Returns singleton instance of the [TranscoderState] class
  static TranscoderState getInstance() {
    return _instance;
  }

  // setters
  /// Set mp3 output bitrate and encoding quality

  void setMp3Bitrate(Mp3Bitrate bitrate) {
    _mp3config.bitrate = bitrate;
  }

  /// Set mp3 encoding quality
  void setMp3Quality(Mp3Quality quality) {
    _mp3config.quality = quality;
  }

  /// Sets the number of threads to be used
  /// Does not allow setting more threads than available in the system hardware
  void setNoOfThreads(int noOfThreads) {
    if (noOfThreads > Platform.numberOfProcessors || noOfThreads < 1) {
      return;
    }
    _noOfThreads = noOfThreads;
  }

  /// Sets copyUnrecognisedFiles
  /// If true, will simply copy any file in the source directory that in not a supported audio file
  void setCopyUnrecognisedFiles(bool copyUnrecognisedFiles) {
    _copyUnrecognisedFiles = copyUnrecognisedFiles;
  }

  /// Opens a file picker and sets the path of the selected folder as the source directory
  ///
  /// Returns the selected path or null if no path was selected
  Future<String?> setSourcePathViaOS() async {
    _srcPath = await FilePicker.platform.getDirectoryPath() ?? "";
    return _srcPath;
  }

  /// Opens a file picker and sets the path of the selected folder as the destination directory
  ///
  /// Returns the selected path or null if no path was selected
  Future<String?> setDestinationPathViaOS() async {
    _destPath = await FilePicker.platform.getDirectoryPath() ?? "";
    return _destPath;
  }

  /// Manually set the source path
  /// Does not allow empty or invalid paths
  /// Returns true if path is valid, false otherwise
  Future<bool> setSource(String path) async {
    if (!await Directory(path).exists()) {
      return false;
    }
    _srcPath = path;
    return true;
  }

  /// Manually set the destination path
  /// Does not allow empty or invalid paths
  /// Returns true if path is valid, false otherwise
  Future<bool> setDestination(String path) async {
    if (!await Directory(_srcPath).exists()) {
      return false;
    }
    _destPath = path;
    return true;
  }

  /// Sets the targer format
  void setTargetFormat(TargetFormat targetFormat) {
    _targetFormat = targetFormat;
  }

  // getters
  /// Returns the source path, null if it is not set
  String? getSrcPath() {
    return _srcPath;
  }

  /// Returns the destination path, null if it is not set
  String? getDestPath() {
    return _destPath;
  }

  /// Returns the number of threads set to perform the convertion
  /// 1 <= noOfThreads <= [Platform.numberOfProcessors]
  int getNoOfThreads() {
    return _noOfThreads;
  }

  /// Returns wheather unrecognised files are to be copied or not
  /// If true, will simply copy any file in the source directory that in not a supported audio file
  bool getCopyUnrecognisedFiles() {
    return _copyUnrecognisedFiles;
  }

  /// Returns the current state of the mp3 configuration
  Mp3Config getMp3Config() {
    return _mp3config;
  }

  /// Returns the currently selected target format
  TargetFormat getTargetFormat() {
    return _targetFormat;
  }

  // methods
  Future<bool> startConversion() async {
    if (_srcPath.isEmpty) {
      throw const FormatException("Source path not set.");
    } else if (!await Directory(_srcPath).exists()) {
      throw const FormatException("Source path is invalid.");
    } else if (_destPath.isEmpty) {
      throw const FormatException("Destination path not set.");
    } else if (!await Directory(_destPath).exists()) {
      throw const FormatException("Destination path is invalid.");
    }
    Convert(
      copyUnrecognisedFiles: _copyUnrecognisedFiles,
      destPath: _destPath,
      srcPath: _srcPath,
      mp3Config: _mp3config,
      targetFormat: _targetFormat,
      noOfThreads: _noOfThreads,
    ).sendSignalToRust();
    return true;
  }
}
