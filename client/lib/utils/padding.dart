import 'package:flutter/material.dart';

/// Calculate addiotional padding for text, introduced by lineheight.
/// 
/// For example, if the text has font size 16, and line height 1.5,
/// then the height of one line of text would actually be 24 pixels,
/// therefore if consistent visual padding is desired on all sides,
/// then the padding on top and bottom should be 4 pixels smaller.
/// 
/// This function automatically calculates how much extra padding the top and bottom sides will have,
/// so that it can be subtracted from the subsequent padding calculations
double textExtraVerticalPadding(TextStyle style) {
  // TODO: implement
  throw UnimplementedError();
}