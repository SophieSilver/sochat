import 'package:client/widgets/chat_window/message_list/message_bubble.dart';
import 'package:flutter/material.dart';

class MessageList extends StatelessWidget {
  final List<String> messages;

  const MessageList({super.key, required this.messages});

  @override
  Widget build(BuildContext context) {
    print(this.messages);

    return Expanded(
      child: Container(
        margin: EdgeInsets.all(7.5),
        child: ListView(
          children: this
              .messages
              .map((String text) => MessageBubble(text: text))
              .toList(growable: false),
        ),
      ),
    );
  }
}
