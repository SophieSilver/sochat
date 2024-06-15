import 'package:client/service/rust_service.dart';
import 'package:flutter/material.dart';

class Conversation with ChangeNotifier {
  int get messageCount => RustService.instance.messageCount();

  void sendMessage(String message) {
    RustService.instance.addMessage(message: message);
    this.notifyListeners();
  }

  String? getMessage(int index) => RustService.instance.getMessage(index: index);
}
