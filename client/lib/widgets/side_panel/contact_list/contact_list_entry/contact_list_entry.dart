import 'package:client/service/conversation.dart';
import 'package:flutter/material.dart';

class ContactListEntry extends StatelessWidget {
  final Conversation conversation;
  final int index;
  final bool isSelected;
  final void Function(int) onSelected;

  const ContactListEntry({
    super.key,
    required this.conversation,
    required this.index,
    required this.isSelected,
    required this.onSelected,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final idText = this.conversation.other.toString();
    final backgroundColor = this.isSelected
        ? colorScheme.secondaryContainer
        : colorScheme.surfaceContainer;

    return Material(
      color: backgroundColor,
      shadowColor: Colors.transparent,
      child: InkWell(
        onTap: () => this.onSelected(this.index),
        splashFactory: NoSplash.splashFactory,
        child: Container(
          decoration: BoxDecoration(
            border: Border(
                bottom: BorderSide(color: colorScheme.surfaceContainerHigh)),
          ),
          padding: EdgeInsets.symmetric(vertical: 8.0, horizontal: 8.0),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              CircleAvatar(
                radius: 30.0,
                backgroundColor: this.isSelected
                    ? colorScheme.primaryContainer
                    : colorScheme.secondaryContainer,
                child: Text(idText[0]),
              ),
              Container(
                padding: EdgeInsets.symmetric(vertical: 8.0, horizontal: 12.0),
                child: Text(
                  idText,
                  style: theme.textTheme.bodyLarge
                      ?.copyWith(fontFamily: "Consolas"),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
