import 'package:client/widgets/chat_window/message_bar/message_bar.dart';
import 'package:flutter/material.dart';

class ChatWindow extends StatefulWidget {
  const ChatWindow({super.key});

  @override
  State<StatefulWidget> createState() {
    return _ChatWindowState();
  }
}

class _ChatWindowState extends State<ChatWindow> {
  List<String> _messages = [];

  @override
  void initState() {
    super.initState();
    this._messages = [];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: <Widget>[
          Expanded(child: SizedBox.shrink()),
          MessageBar(onMessageSend: (t) {
            print("submitted message: ${t}");
          })
        ],
      ),
    );
  }
}
