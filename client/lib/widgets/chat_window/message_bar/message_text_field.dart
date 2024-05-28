import 'dart:math';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class MessageTextField extends StatelessWidget {
  final TextEditingController controller;
  final void Function(String) onSubmit;
  final double lineHeight;
  
  late final _focusNode = FocusNode(onKeyEvent: this._onKeyEvent);

  MessageTextField({
    super.key,
    required this.controller,
    required this.onSubmit,
    required this.lineHeight,
  });
  
  KeyEventResult _onKeyEvent(FocusNode node, KeyEvent event) {
    // If pressing enter and not pressing shift at the same time,
    // submit the text
    final enterPressed = event.logicalKey == LogicalKeyboardKey.enter && event is KeyDownEvent;
    final shiftHeld = HardwareKeyboard.instance.isShiftPressed;
    
    if (enterPressed && !shiftHeld) {
      this.onSubmit(this.controller.text);
      return KeyEventResult.handled;
    }
    
    return KeyEventResult.ignored;
  }

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
          focusNode: this._focusNode,
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
