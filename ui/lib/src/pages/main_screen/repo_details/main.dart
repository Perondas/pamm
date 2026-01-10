import 'package:flutter/material.dart';

import '../../../models/stored_repo.dart';

class RepoDetails extends StatefulWidget {
  const RepoDetails(this.selectedRepo, {super.key});

  final StoredRepo? selectedRepo;

  @override
  State<RepoDetails> createState() => _RepoDetailsState();
}

class _RepoDetailsState extends State<RepoDetails> {
  @override
  Widget build(BuildContext context) {
    return Expanded(
      flex: 3,
      child: Container(
        color: Colors.white,
        child: widget.selectedRepo == null
            ? Center(child: Text("Select a repository to view details"))
            : Padding(
                padding: const EdgeInsets.all(16.0),
                child: SingleChildScrollView(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        widget.selectedRepo!.name,
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                      const SizedBox(height: 8),
                      Text(widget.selectedRepo!.description),
                      const SizedBox(height: 12),
                      Text(
                        'Path:',
                        style: TextStyle(fontWeight: FontWeight.bold),
                      ),
                      Text(widget.selectedRepo!.path),
                      const SizedBox(height: 12),
                      Text(
                        'Packs:',
                        style: TextStyle(fontWeight: FontWeight.bold),
                      ),
                      const SizedBox(height: 8),
                      if (widget.selectedRepo!.packs.isEmpty)
                        Text('No packs available')
                      else
                        ...widget.selectedRepo!.packs.map(
                          (p) => Padding(
                            padding: const EdgeInsets.symmetric(vertical: 2.0),
                            child: Text('- $p'),
                          ),
                        ),
                    ],
                  ),
                ),
              ),
      ),
    );
  }
}
