import 'dart:io';

import 'package:flutter/material.dart';
import 'package:open_folder/open_folder.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_list/edit_repo_dialog.dart';
import 'package:pamm_ui/src/pages/main_screen/repo_list/fix_repo_dialog.dart';
import 'package:pamm_ui/src/pages/settings_screen/main.dart';
import 'package:pamm_ui/src/rust/api/commands/init_from_remote.dart';
import 'package:pamm_ui/src/services/repo_path_store.dart';
import 'package:pamm_ui/src/services/repo_state_store.dart';
import 'package:pamm_ui/src/services/settings_service.dart';
import 'package:pamm_ui/src/util/media.dart';
import 'package:pamm_ui/src/widgets/confirm_dialog.dart';
import 'package:pamm_ui/src/widgets/media_icon.dart';
import 'package:pamm_ui/src/widgets/proton_dialog.dart';

/// The header bar: repo selector, repo path, repo actions and app settings.
class TopBar extends StatelessWidget {
  const TopBar({
    required this.repos,
    required this.selectedRepo,
    required this.onSelectRepo,
    required this.onAddRepo,
    required this.onReposChanged,
    super.key,
  });

  final List<RepoStateManager> repos;
  final RepoStateManager? selectedRepo;
  final ValueChanged<RepoStateManager> onSelectRepo;
  final VoidCallback onAddRepo;

  /// Called when the set of repos or their config may have changed.
  final Future<void> Function() onReposChanged;

  static const double height = 72;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colors = theme.colorScheme;
    final selected = selectedRepo;

    return Material(
      color: colors.surfaceContainerHigh,
      elevation: 2,
      shadowColor: colors.shadow,
      child: SizedBox(
        height: height,
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            SizedBox(
              width: 380,
              child: _RepoSelector(
                repos: repos,
                selectedRepo: selected,
                onSelectRepo: onSelectRepo,
                onAddRepo: onAddRepo,
                onReposChanged: onReposChanged,
              ),
            ),
            VerticalDivider(width: 1, color: colors.outlineVariant),
            Expanded(
              child: selected == null
                  ? const SizedBox.shrink()
                  : ListenableBuilder(
                      listenable: Listenable.merge([selected, settingsService]),
                      builder: (context, _) =>
                          _buildRepoActions(context, selected),
                    ),
            ),
            VerticalDivider(width: 1, color: colors.outlineVariant),
            const SizedBox(width: 8),
            if (Platform.isLinux)
              IconButton(
                tooltip: "Proton",
                onPressed: () => showDialog<void>(
                  context: context,
                  builder: (_) => const ProtonDialog(),
                ),
                icon: const Icon(Icons.wine_bar),
              ),
            IconButton(
              tooltip: "Settings",
              iconSize: 28,
              onPressed: () {
                Navigator.of(context).push(
                  MaterialPageRoute(builder: (context) => SettingsScreen()),
                );
              },
              icon: const Icon(Icons.settings),
            ),
            const SizedBox(width: 12),
          ],
        ),
      ),
    );
  }

  Widget _buildRepoActions(BuildContext context, RepoStateManager repo) {
    final theme = Theme.of(context);
    final mmMode = settingsService.settings.mmSettings.mmModeEnabled;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: [
          Expanded(
            child: SelectableText(
              repo.repoPath,
              maxLines: 1,
              style: theme.textTheme.titleMedium,
            ),
          ),
          if (repo.configUpdateError != null)
            IconButton(
              tooltip: 'Config update failed',
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text(repo.configUpdateError!),
                    backgroundColor: Colors.red,
                  ),
                );
              },
              icon: const Icon(Icons.warning, color: Colors.red),
            ),
          if (mmMode) ...[
            const SizedBox(width: 8),
            FilledButton.tonal(
              onPressed: () async {
                // The double backslash is correct, but for some
                // reason the library won't open it.
                await OpenFolder.openFolder(
                  repo.repoPath.replaceAll("\\\\", "\\"),
                );
              },
              child: const Text("Explore Folder"),
            ),
          ],
          if (repo.repoState != null) ...[
            const SizedBox(width: 8),
            FilledButton(
              onPressed: () async {
                await showDialog<RepoConfig?>(
                  context: context,
                  builder: (_) => EditRepoDialog(repo.repoState!),
                );
                await onReposChanged();
              },
              child: const Text("Modpack Settings"),
            ),
          ],
        ],
      ),
    );
  }
}

/// Repo selector in the top left: a Material [DropdownMenu] listing all repos
/// plus an "Add Repository" button.
class _RepoSelector extends StatelessWidget {
  const _RepoSelector({
    required this.repos,
    required this.selectedRepo,
    required this.onSelectRepo,
    required this.onAddRepo,
    required this.onReposChanged,
  });

  final List<RepoStateManager> repos;
  final RepoStateManager? selectedRepo;
  final ValueChanged<RepoStateManager> onSelectRepo;
  final VoidCallback onAddRepo;
  final Future<void> Function() onReposChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 12, right: 4),
      child: Row(
        children: [
          Expanded(
            child: ListenableBuilder(
              // Rebuild as repos finish loading.
              listenable: Listenable.merge(repos),
              builder: (context, _) => LayoutBuilder(
                builder: (context, constraints) =>
                    _buildDropdown(context, constraints.maxWidth),
              ),
            ),
          ),
          IconButton(
            tooltip: 'Add Repository',
            onPressed: onAddRepo,
            icon: const Icon(Icons.add),
          ),
        ],
      ),
    );
  }

  Widget _buildDropdown(BuildContext context, double width) {
    final selected = selectedRepo;
    final selectedState = selected?.repoState;

    return DropdownMenu<RepoStateManager>(
      // Reset the field when the selection or the repo list changes.
      key: ValueKey((selected, repos)),
      width: width,
      initialSelection: selected,
      enabled: repos.isNotEmpty,
      requestFocusOnTap: false,
      hintText: repos.isEmpty ? 'Add a repository' : 'Select a repository',
      leadingIcon: selectedState == null
          ? null
          : Padding(
              padding: const EdgeInsets.all(8),
              child: MediaIcon(
                iconFile: mediaFile(
                  selectedState.path,
                  selectedState.repo.customization?.icon,
                ),
                fallback: selectedState.repo.name,
              ),
            ),
      textStyle: Theme.of(context).textTheme.titleMedium,
      inputDecorationTheme: const InputDecorationTheme(
        border: InputBorder.none,
        enabledBorder: InputBorder.none,
        focusedBorder: InputBorder.none,
      ),
      menuHeight: 480,
      onSelected: (repo) {
        if (repo != null) onSelectRepo(repo);
      },
      dropdownMenuEntries: [
        for (final repo in repos) _buildEntry(context, repo),
      ],
    );
  }

  DropdownMenuEntry<RepoStateManager> _buildEntry(
    BuildContext context,
    RepoStateManager repo,
  ) {
    if (!repo.doneLoading) {
      return DropdownMenuEntry(
        value: repo,
        label: repo.repoPath,
        enabled: false,
        leadingIcon: const SizedBox.square(
          dimension: 24,
          child: CircularProgressIndicator(strokeWidth: 2),
        ),
        labelWidget: _entryText(context, repo.repoPath, 'Loading...'),
      );
    }

    if (repo.configLoadError != null) {
      // Not selectable; the trailing buttons offer the ways to fix it.
      return DropdownMenuEntry(
        value: repo,
        label: repo.repoPath,
        enabled: false,
        leadingIcon: const Icon(Icons.error, color: Colors.red),
        labelWidget: _entryText(context, repo.repoPath, 'Error loading repo'),
        trailingIcon: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            IconButton(
              tooltip: 'Fix path',
              onPressed: () => _fixRepo(context, repo),
              icon: const Icon(Icons.build),
            ),
            IconButton(
              tooltip: 'Show error',
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text(repo.configLoadError!),
                    backgroundColor: Colors.red,
                  ),
                );
              },
              icon: const Icon(Icons.bug_report),
            ),
            IconButton(
              tooltip: 'Remove repository',
              onPressed: () => _deleteRepo(context, repo),
              icon: const Icon(Icons.delete_forever),
            ),
          ],
        ),
      );
    }

    final state = repo.repoState!;
    return DropdownMenuEntry(
      value: repo,
      label: state.repo.name,
      leadingIcon: MediaIcon(
        iconFile: mediaFile(state.path, state.repo.customization?.icon),
        fallback: state.repo.name,
      ),
      labelWidget: _entryText(context, state.repo.name, state.path),
      trailingIcon: repo.configUpdateError != null
          ? const Tooltip(
              message: 'Config update failed',
              child: Icon(Icons.warning, color: Colors.red),
            )
          : null,
      style: MenuItemButton.styleFrom(minimumSize: const Size(0, 64)),
    );
  }

  Widget _entryText(BuildContext context, String title, String subtitle) {
    final theme = Theme.of(context);
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(title, overflow: TextOverflow.ellipsis),
        Text(
          subtitle,
          overflow: TextOverflow.ellipsis,
          style: theme.textTheme.bodySmall?.copyWith(
            color: theme.colorScheme.onSurfaceVariant,
          ),
        ),
      ],
    );
  }

  Future<void> _fixRepo(BuildContext context, RepoStateManager repo) async {
    final path = await showDialog<String?>(
      context: context,
      builder: (_) => FixRepoDialog(repo.repoPath),
    );

    if (path != null) {
      await RepoPathStore.remove(repo.repoPath);
      await RepoPathStore.add(path);
      await onReposChanged();
    }
  }

  Future<void> _deleteRepo(BuildContext context, RepoStateManager repo) async {
    final confirm =
        await showDialog<bool?>(
          context: context,
          builder: (_) => ConfirmDialog(
            content:
                'Are you sure you want to delete the repository at "${repo.repoPath}"? This will not delete the files on disk.',
          ),
        ) ??
        false;

    if (confirm) {
      await RepoPathStore.remove(repo.repoPath);
    }
    await onReposChanged();
  }
}
