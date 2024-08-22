import 'package:client/service/conversation.dart';
import 'package:client/service/rust_service.dart';
import 'package:client/src/rust/api/types/id.dart';
import 'package:client/widgets/side_panel/contact_list/contact_list.dart';
import 'package:client/widgets/side_panel/new_conversation_tab/new_conversation_tab.dart';
import 'package:flutter/material.dart';

class SidePanelContent extends StatefulWidget {
  final void Function(Conversation) onSwitchConversation;

  const SidePanelContent({super.key, required this.onSwitchConversation});

  @override
  State<SidePanelContent> createState() => _SidePanelContentState();
}

class _SidePanelContentState extends State<SidePanelContent> {
  List<Conversation> conversations = [];
  int? selectedConversation;

  void addConversation(UserId other) {
    final UserId selfId = RustService.instance.thisId;

    if (other.equals(selfId)) {
      return;
    }
    final newConversation = Conversation(self: selfId, other: other);

    this.setState(() {
      this.conversations.add(newConversation);
    });
  }

  void selectConversation(int conversationIndex) {
    this.setState(() {
      this.selectedConversation = conversationIndex;
    });
    this
        .widget
        .onSwitchConversation(this.conversations[this.selectedConversation!]);
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        NewConversationTab(onSubmitId: this.addConversation),
        this.conversations.isEmpty
            ? Expanded(
                child: Center(
                    child: Text(
                "No Contacts",
                style: theme.textTheme.headlineSmall,
              )))
            : ContactList(
                conversationList: this.conversations,
                selectedConversation: this.selectedConversation,
                onSelected: this.selectConversation,
              ),
      ],
    );
  }
}
