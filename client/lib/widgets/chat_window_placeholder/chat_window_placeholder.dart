import 'package:flutter/material.dart';

class ChatWindowPlaceholder extends StatelessWidget {
  const ChatWindowPlaceholder({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    // final colorScheme = theme.colorScheme;
    final textStyle = theme.textTheme;

    return Center(
      child: Container(
        padding: EdgeInsets.symmetric(horizontal: 50.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Text("No conversations open", style: textStyle.headlineSmall),
            SizedBox(height: 20.0),
            Text("Add a new contact and start a conversation", style: textStyle.titleLarge, textAlign: TextAlign.center,),
            SizedBox(height: 20.0),
            Icon(Icons.arrow_back, size: 50.0,)
          ],
        ),
      ),
    );
  }
}
