import 'package:client/service/conversation.dart';
import 'package:client/widgets/chat_window/message_bar/message_bar.dart';
import 'package:client/widgets/chat_window/message_list/message_list.dart';
import 'package:flutter/material.dart';

class ChatWindow extends StatefulWidget {
  final Conversation conversation;
  
  const ChatWindow({super.key, required this.conversation});

  @override
  State<StatefulWidget> createState() {
    return _ChatWindowState();
  }
}

class _ChatWindowState extends State<ChatWindow> {
  @override
  Widget build(BuildContext context) {
    AnimatedIcons.add_event;

    return Column(
      children: <Widget>[
        MessageList(
          conversation: this.widget.conversation,
        ),
        MessageBar(conversation: this.widget.conversation,)
      ],
    );
  }
}
