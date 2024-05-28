import 'package:client/widgets/chat_window/message_bar/message_bar.dart';
import 'package:client/widgets/chat_window/message_list/message_list.dart';
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

  void addMessage(String message) {
    this.setState(() {
      this._messages.add(message);
    });
  }

  @override
  Widget build(BuildContext context) {
    AnimatedIcons.add_event;
    
    return Scaffold(
      body: Column(
        children: <Widget>[
          MessageList(messages: this._messages),
          MessageBar(onMessageSend: (t) {
            this.addMessage(t);
          })
        ],
      ),
    );
  }
}
