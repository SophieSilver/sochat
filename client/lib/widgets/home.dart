import "package:client/widgets/chat_window/chat_window.dart";
import "package:flutter/material.dart";

class Home extends StatelessWidget {
  const Home({super.key});
  
  @override
  Widget build(BuildContext context) {
    return ChatWindow();
  }
}