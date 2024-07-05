import 'package:client/service/conversation.dart';
import 'package:client/widgets/side_panel/side_panel_content.dart';
import 'package:flutter/material.dart';

class SidePanel extends StatelessWidget {
  final void Function(Conversation) onSwitchConversation;
  const SidePanel({super.key, required this.onSwitchConversation});

  @override
  Widget build(BuildContext context) {
    const double sidePanelWidth = 350.0;

    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Material(
      color: colorScheme.surfaceContainerLow,
      child: Row(
        children: [
          SizedBox(
            width: sidePanelWidth,
            child: SidePanelContent(
              onSwitchConversation: this.onSwitchConversation,
            ),
          ),
          VerticalDivider(
            color: colorScheme.surfaceContainerHigh,
            width: 2.0,
            thickness: 2.0,
          ),
        ],
      ),
    );
  }
}
