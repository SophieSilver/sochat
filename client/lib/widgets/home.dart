import "package:client/widgets/chat_window/chat_window.dart";
import "package:client/widgets/side_panel/side_panel.dart";
import "package:flutter/material.dart";

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Material(
      child: Row(
        children: [
          SidePanel(),
          Expanded(
            child: ChatWindow(),
          )
        ],
      ),
    );
  }
}
