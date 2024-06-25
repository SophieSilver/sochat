import 'package:client/service/conversation.dart';
import 'package:client/src/rust/api/types/id.dart';
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
  
  void addConversation(UserId other) {
    final UserId selfId = UserId.parse("AAAAAAAAAAAAAAAAAAAAAA");
    
    if (other == selfId) {
              return;
            }
    final newConversation = Conversation(self: selfId, other: other);
    
    this.setState(() {
      this.conversations.add(newConversation);
    });
    // TODO: switch to the new convo
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        NewConversationTab(
          onSubmitId: this.addConversation
        ),
        this.conversations.isEmpty
            ? Expanded(
                child: Center(
                    child: Text(
                "No Contacts",
                style: theme.textTheme.headlineSmall,
              )))
            : Placeholder(),
      ],
    );
  }
}
