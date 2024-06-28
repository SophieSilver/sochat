import 'dart:math';

import 'package:client/service/conversation.dart';
import 'package:client/widgets/chat_window/message_list/message_bubble.dart';
import 'package:flutter/material.dart';

class MessageList extends StatefulWidget {
  final Conversation conversation;

  const MessageList({super.key, required this.conversation});

  @override
  State<MessageList> createState() => _MessageListState();
}

class _MessageListState extends State<MessageList> {
  final _listController = ScrollController();
  bool needsScrollDown = false;

  @override
  void initState() {
    super.initState();
    this.widget.conversation.addListener(this._onNewMessage);
  }

  @override
  void didUpdateWidget(covariant MessageList oldWidget) {
    super.didUpdateWidget(oldWidget);

    if (oldWidget.conversation == this.widget.conversation) {
      return;
    }

    oldWidget.conversation.removeListener(this._onNewMessage);
    this.widget.conversation.addListener(this._onNewMessage);
  }

  void _onNewMessage() {
    this.setState(() {
      this.needsScrollDown = true;
    });
  }

  void _scrollToBottom(Duration timestamp) {
    final position = this._listController.position;

    // adding some amount of time to the animation proportional to the distance to the bottom of the list
    final distanceToEnd =
        position.maxScrollExtent - this._listController.offset;
    // capping at 1 second to not take too long
    final addedMilliseconds = min(1000, (distanceToEnd).toInt());
    final addedDuration = Duration(milliseconds: addedMilliseconds);

    position.animateTo(
      position.maxScrollExtent,
      duration: Durations.short4 + addedDuration,
      curve: Curves.easeOutCirc,
    );
  }

  Widget? _itemBuilder(BuildContext context, int index) {
    print(index);
    final message = this.widget.conversation.getMessage(index);

    return message == null ? null : MessageBubble(text: message);
  }

  @override
  Widget build(BuildContext context) {
    if (this.needsScrollDown) {
      this.needsScrollDown = false;
      WidgetsBinding.instance.addPostFrameCallback(this._scrollToBottom);
    }

    return Expanded(
      child: Container(
        margin: EdgeInsets.all(7.5),
        child: ListView.builder(
          itemBuilder: this._itemBuilder,
          cacheExtent: 5000,
          itemCount: this.widget.conversation.messageCount,
          controller: this._listController,
        ),
      ),
    );
  }

  @override
  void dispose() {
    this.widget.conversation.removeListener(this._onNewMessage);
    super.dispose();
  }
}
