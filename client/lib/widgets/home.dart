import "package:client/service/conversation.dart";
import "package:client/widgets/chat_window/chat_window.dart";
import "package:client/widgets/chat_window_placeholder/chat_window_placeholder.dart";
import "package:client/widgets/side_panel/side_panel.dart";
import "package:flutter/material.dart";

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  Conversation? conversation;

  void switchConversation(Conversation value) {
    this.setState(() {
      this.conversation = value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      child: Row(
        children: [
          SidePanel(
            onSwitchConversation: this.switchConversation,
          ),
          Expanded(
            child: this.conversation != null
                ? ChatWindow(
                    conversation: this.conversation!,
                  )
                : ChatWindowPlaceholder(),
          )
        ],
      ),
    );
  }
}
