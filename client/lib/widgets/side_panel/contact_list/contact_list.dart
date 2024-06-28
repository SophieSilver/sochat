import 'package:client/service/conversation.dart';
import 'package:client/widgets/side_panel/contact_list/contact_list_entry/contact_list_entry.dart';
import 'package:flutter/material.dart';

class ContactList extends StatelessWidget {
  final List<Conversation> conversationList;
  final int? selectedConversation;
  final void Function(int) onSelected;

  const ContactList({
    super.key,
    required this.conversationList,
    required this.selectedConversation,
    required this.onSelected,
  });

  @override
  Widget build(BuildContext context) {
    return Expanded(
        child: ListView(
      children: this
          .conversationList
          .indexed
          .map((pair) => ContactListEntry(
                conversation: pair.$2,
                index: pair.$1,
                isSelected: this.selectedConversation == pair.$1,
                onSelected: this.onSelected,
              ))
          .toList(),
    ));
  }
}
