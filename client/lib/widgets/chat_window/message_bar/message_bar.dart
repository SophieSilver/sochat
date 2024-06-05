import 'package:client/service/conversation.dart';
import 'package:client/widgets/chat_window/message_bar/message_text_field.dart';
import 'package:flutter/material.dart';

class MessageBar extends StatefulWidget {
  final Conversation conversation;

  const MessageBar({super.key, required this.conversation});

  @override
  State<StatefulWidget> createState() {
    return _MessageBarState();
  }
}

class _MessageBarState extends State<MessageBar> {
  final TextEditingController _textFieldController = TextEditingController();

  void _submitMessage(String text) {
    final trimmedText = text.trim();
    if (trimmedText.isEmpty) {
      return;
    }

    // make the textfield empty
    this._textFieldController.value = TextEditingValue.empty;
    this.widget.conversation.sendMessage(trimmedText);
  }

  @override
  Widget build(BuildContext context) {
    const sendButtonSize = 42.0;
    const iconPadding = 7.0;
    const iconUnpaddedSize = sendButtonSize - (iconPadding * 2.0);

    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    
    
    // FIXME: solve the issue with messagebar casting shadow on the side panel
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
                controller: _textFieldController,
                lineHeight: sendButtonSize,
                onSubmit: this._submitMessage),
            // margin
            SizedBox(width: 10.0),
            // Circular IconButton
            Ink(
              decoration: ShapeDecoration(
                shape: CircleBorder(),
                color: colorScheme.primaryContainer,
              ),
              child: IconButton(
                onPressed: () => this._submitMessage(this._textFieldController.text),
                icon: Icon(Icons.send_rounded),
                iconSize: iconUnpaddedSize,
                padding: EdgeInsets.all(iconPadding),
                color: colorScheme.onPrimaryContainer,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
