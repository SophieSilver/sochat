import 'package:client/widgets/chat_window/message_bar/message_text_field.dart';
import 'package:flutter/material.dart';

class MessageBar extends StatefulWidget {
  final void Function(String) onMessageSend;

  const MessageBar({super.key, required this.onMessageSend});

  @override
  State<StatefulWidget> createState() {
    return _MessageBarState();
  }
}

class _MessageBarState extends State<MessageBar> {
  final TextEditingController controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Material(
      elevation: 10.0,
      child: Container(
        padding: const EdgeInsets.all(10.0),
        decoration: BoxDecoration(
            border: Border(
                top: BorderSide(
          color: colorScheme.shadow.withAlpha(35),
          width: 2.0,
        ))),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            MessageTextField(
                controller: controller, lineHeight: 32.0, onSubmit: (_) {}),
            Container(
              color: Colors.red,
              height: 32.0,
              width: 50.0,
            )
          ],
        ),
      ),
    );
  }
}
