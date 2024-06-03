import 'package:client/state/conversation_state.dart';
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
  ConversationState messageState =ConversationState();

  void addMessage(String message) {
    this.setState(() {
    this.messageState.addMessage(message);
      
    });
  }

  @override
  Widget build(BuildContext context) {
    AnimatedIcons.add_event;

    return Scaffold(
      body: Column(
        children: <Widget>[
          MessageList(
            conversationState: this.messageState,
          ),
          MessageBar(onMessageSend: (t) {
            this.addMessage(t);
          })
        ],
      ),
    );
  }
}
