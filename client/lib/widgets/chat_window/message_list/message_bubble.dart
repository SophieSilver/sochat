import 'dart:math';
import 'dart:ui';

import 'package:client/utils/padding.dart';
import 'package:flutter/material.dart';

class MessageBubble extends StatelessWidget {
  final String text;

  const MessageBubble({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    const double paddingToFontSizeRatio = 1.5;
    const double pinchedRadiusCoefficient = 0.4;

    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    var textStyle = theme.textTheme.bodyLarge;

    print(textStyle);

    if (textStyle?.fontSize == null) {
      textStyle = textStyle?.copyWith(fontSize: 16.0);
    }
    if (textStyle?.fontSize == null) {
      textStyle = textStyle?.copyWith(height: 1.5);
    }

    final extraVerticalPadding =
        textStyle == null ? 0.0 : textExcessVerticalPadding(textStyle);
    
    final verticalPadding =
        (textStyle?.fontSize ?? 16.0) * paddingToFontSizeRatio / 2.0;
    final horizontalPadding = verticalPadding + extraVerticalPadding;

    final borderRadius =
        (textStyle?.fontSize ?? 16.0) / 2.0 + horizontalPadding;

    final pinchedBorderRadius = borderRadius * pinchedRadiusCoefficient;

    // top level container needed to align the child to the left
    return Align(
      alignment: Alignment.centerLeft,
      child: Container(
        margin: EdgeInsets.all(2.5),
        decoration: BoxDecoration(
          color: colorScheme.primaryContainer,
          borderRadius: BorderRadius.circular(borderRadius).copyWith(
            bottomLeft: Radius.circular(pinchedBorderRadius),
            // topLeft: Radius.circular(pinchedBorderRadius),
          ),
        ),
        padding: EdgeInsets.symmetric(
          horizontal: horizontalPadding,
          vertical: verticalPadding,
        ),
        child: SelectableText(
          this.text,
          style: textStyle?.copyWith(color: colorScheme.onPrimaryContainer),
          selectionHeightStyle: BoxHeightStyle.strut,
          textHeightBehavior: TextHeightBehavior(
            leadingDistribution: TextLeadingDistribution.even,
          ),
        ),
      ),
    );
  }
}
