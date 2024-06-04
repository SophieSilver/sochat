import 'package:client/widgets/side_panel/side_panel_content.dart';
import 'package:flutter/material.dart';

class SidePanel extends StatelessWidget {
  const SidePanel({super.key});

  @override
  Widget build(BuildContext context) {
    const double sidePanelWidth = 350.0;

    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Material(
      elevation: 4.0,
      surfaceTintColor: colorScheme.surfaceTint,
      child: Row(
        children: [
          SizedBox(
            width: sidePanelWidth,
            child: SidePanelContent(),
          ),
          VerticalDivider(
            color: colorScheme.secondaryContainer,
            width: 2.0,
            thickness: 2.0,
          ),
        ],
      ),
    );
  }
}
