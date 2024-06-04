import 'package:flutter/material.dart';

class SidePanelContent extends StatelessWidget {
  const SidePanelContent({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.all(10.0),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.baseline,
            textBaseline: TextBaseline.alphabetic,
            children: [
              Text(
                "Your ID: ",
                style: TextStyle(inherit: true, fontSize: 16.0, height: 1.5),
              ),
              SelectableText(
                "JNkjnknjkfs2342fjsdfn_",
                style: TextStyle(
                  fontFamily: "Consolas",
                  inherit: true,
                  fontSize: 16.0,
                  height: 1.5,
                ),
              ),
            ],
          ),
          SizedBox(height: 10.0),
          OutlinedButton(onPressed: () {}, child: Text("Copy ID")),
          // FilledButton(onPressed: () {}, child: Text("hi"),),
          SizedBox(height: 30.0),
          Row(
            crossAxisAlignment: CrossAxisAlignment.baseline,
            textBaseline: TextBaseline.alphabetic,
            children: [
              Text(
                "Other ID: ",
                style: TextStyle(inherit: true, fontSize: 16.0, height: 1.5),
              ),
              SizedBox(
                width: 250.0,
                child: TextField(
                  onSubmitted: (value) {
                    print(value);
                  },
                  decoration: InputDecoration(
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(100.0),
                    ),
                    isDense: true,
                  ),
                  style: TextStyle(
                    fontFamily: "Consolas",
                    inherit: true,
                    fontSize: 16.0,
                    height: 1.5,
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
