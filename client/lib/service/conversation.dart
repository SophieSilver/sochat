import 'package:client/service/rust_service.dart';
import 'package:client/src/rust/api/types/id.dart';
import 'package:flutter/material.dart';

class Conversation with ChangeNotifier {
  final UserId self;
  final UserId other;

  Conversation({required this.self, required this.other});

  int get messageCount => RustService.instance.messageCount(from: this.self, to: this.other);

  void sendMessage(String message) {
    RustService.instance.sendMessage(to: this.other, message: message);
    this.notifyListeners();
  }

  @override
  void dispose() {
    super.dispose();
  }

  String? getMessage(int index) => RustService.instance
      .getMessage(from: this.self, to: this.other, index: index);
}
