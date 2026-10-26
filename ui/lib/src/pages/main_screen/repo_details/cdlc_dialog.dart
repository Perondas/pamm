import 'package:flutter/material.dart';

class CdlcDialog extends StatelessWidget {
  const CdlcDialog({super.key});

  static const List<Cdlc> cdlcs = [
    Cdlc("Global Mobilization", "gm"),
    Cdlc("SOG Prairie Fire", "vn"),
    Cdlc("CSLA Iron Curtain", "csla"),
    Cdlc("Western Sahara", "ws"),
    Cdlc("Spearhead 1944", "spe"),
    Cdlc("Reaction Forces", "rf"),
    Cdlc("Expeditionary Forces", "ef"),
  ];

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      content: SizedBox(
        width: 300,
        height: 450,
        child: ListView.builder(
          itemBuilder: (context, index) {
            return ListTile(
              title: Text(cdlcs[index].display),
              subtitle: Text(cdlcs[index].arg),
              trailing: IconButton(
                onPressed: () {
                  Navigator.of(context).pop(cdlcs[index].arg);
                },
                icon: Icon(Icons.add),
              ),
            );
          },
          itemCount: cdlcs.length,
        ),
      ),
    );
  }
}

class Cdlc {
  const Cdlc(this.display, this.arg);

  final String display;
  final String arg;
}
