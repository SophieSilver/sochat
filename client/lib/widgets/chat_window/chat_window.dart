import 'package:client/service/conversation.dart';
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
  Conversation conversationState = Conversation();

  @override
  Widget build(BuildContext context) {
    AnimatedIcons.add_event;

    return Column(
      children: <Widget>[
        MessageList(
          conversationState: this.conversationState,
        ),
        MessageBar(conversation: this.conversationState,)
      ],
    );
  }
}
