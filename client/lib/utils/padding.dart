import 'dart:math';

import 'package:flutter/material.dart';

/// Calculate addiotional visual padding for text, introduced by lineheight.
/// 
/// For example, if the text has font size 16, and line height 1.5,
/// then the height of one line of text would actually be 24 pixels,
/// therefore if consistent visual padding is desired on all sides,
/// then the padding on top and bottom should be 4 pixels smaller.
/// 
/// This function automatically calculates how much extra padding the top and bottom sides will have,
/// so that it can be subtracted from the subsequent padding calculations
/// 
/// # Returns
/// * Observed extra vertical padding on __one__ side 
/// * 0.0 if `style.fontSize` or `style.height` are null
/// 
/// # Note
/// This function presumes and even amount of space added at the top and bottom of the text.
/// That might not always be true. It's recommended to set `textHeightBehavior`'s 
/// `leadingDistribution` to `TextLeadingDistribution.even`
double textExcessVerticalPadding(TextStyle style) {
  final fontSize = style.fontSize;
  final lineHeight = style.height;
  
  if (fontSize == null || lineHeight == null) {
    return 0.0;
  }
  
  final lineHeightPixels = fontSize * lineHeight;
  
  return max(0.0, (lineHeightPixels - fontSize) / 2.0);
}