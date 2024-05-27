import 'dart:math';
import 'dart:ui';

import 'package:flutter/material.dart';

class MessageTextField extends StatelessWidget {
  final TextEditingController controller;
  final void Function(String) onSubmit;
  final double lineHeight;

  const MessageTextField(
      {super.key,
      required this.controller,
      required this.onSubmit,
      required this.lineHeight});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    final textStyle = theme.textTheme.bodyLarge;

    final fontSize = textStyle?.fontSize ?? 16;
    final verticalPadding = max(0.0, (this.lineHeight - fontSize) / 2.0);
    final borderRadius = this.lineHeight / 2.0;

    final padding = EdgeInsets.symmetric(
      vertical: verticalPadding,
      horizontal: borderRadius * 0.6,
    );

    return Expanded(
      child: Material(
        elevation: 10.0,
        shadowColor: Colors.transparent,
        borderRadius: BorderRadius.circular(borderRadius),
        surfaceTintColor: colorScheme.secondary,
        child: TextField(
          minLines: 1,
          maxLines: 12,
          autofocus: true,
          textInputAction: TextInputAction.newline,
          onSubmitted: this.onSubmit,
          style: textStyle,
          selectionHeightStyle: BoxHeightStyle.includeLineSpacingMiddle,
          decoration: InputDecoration(
            border: InputBorder.none,
            isDense: true,
            contentPadding: padding,
            hintText: "Write a message...",
          ),
        ),
      ),
    );
  }
}
