import 'package:client/src/rust/api/types/id.dart';
import 'package:flutter/material.dart';

class NewConversationTab extends StatefulWidget {
  final void Function(UserId) onSubmitId;

  const NewConversationTab({super.key, required this.onSubmitId});

  @override
  State<NewConversationTab> createState() => _NewConversationTabState();
}

class _NewConversationTabState extends State<NewConversationTab> {
  TextEditingController idTextController = TextEditingController();
  bool isErrored = false;
  int lastFuture = 0;

  void submitId(String value) {
    if (value.isEmpty) {
      return;
    }

    try {
      final id = UserId.parse(value);
      this.widget.onSubmitId(id);
    } on Exception catch (_) {
      this.setState(() {
        this.isErrored = true;
      });

      this.lastFuture += 1;
      final thisFuture = this.lastFuture;
      Future.delayed(Duration(seconds: 5), () {
        if (thisFuture != this.lastFuture) {
          return;
        }
        this.setState(() {
          this.isErrored = false;
        });
      });
    }

    this.idTextController.clear();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Material(
      elevation: 16.0,
      shadowColor: Colors.transparent,
      color: colorScheme.surfaceContainerHigh,
      surfaceTintColor: colorScheme.surfaceTint,
      child: Container(
        padding: EdgeInsets.symmetric(vertical: 8.0, horizontal: 8.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              "Add Contact",
              style: theme.textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 5.0,),
            Row(
              children: [
                Expanded(
                    child: TextField(
                  maxLines: 1,
                  maxLength: 22,
                  decoration: InputDecoration(
                    labelText: "Contact's ID",
                    isDense: true,
                    errorText: this.isErrored ? "Invalid ID" : null,
                  ),
                  style: TextStyle(fontSize: 16, fontFamily: "Consolas"),
                  controller: this.idTextController,
                  onChanged: (_) {
                    this.setState(() {
                      this.isErrored = false;
                    });
                  },
                  onSubmitted: this.submitId,
                )),
                SizedBox(width: 10.0),
                IconButton.filled(
                    onPressed: () {
                      this.submitId(this.idTextController.text);
                    },
                    iconSize: 28,
                    icon: Icon(Icons.add)),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
