import 'package:flutter/material.dart';

class ConversationState with ChangeNotifier {
  final List<String> _messages = [];
  int get messageCount => this._messages.length;

  void addMessage(String message) {
    this._messages.add(message);
    this.notifyListeners();
  }

  String? getMessage(int index) {
    if (index < 0 || index >= this._messages.length) {
      return null;
    }

    return this._messages[index];
  }
}