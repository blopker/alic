import 'package:flutter/material.dart';
import 'package:package_info_plus/package_info_plus.dart';

import 'strings.dart';
import 'update.dart';
import 'widgets.dart';

enum MenuSelection {
  about,
  updates,
  showMessage,
}

class MacMenuBar extends StatefulWidget {
  const MacMenuBar({super.key, required this.child});

  final Widget child;

  @override
  State<MacMenuBar> createState() => _MacMenuBarState();
}

class _MacMenuBarState extends State<MacMenuBar> {
  void _checkForUpdates() async {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Checking for updates...'),
      ),
    );

    // Check for updates
    Update? update;
    String? error;
    try {
      update = await checkForUpdate();
    } catch (e) {
      error = e.toString();
    }
    if (!mounted) return;
    ScaffoldMessenger.of(context).removeCurrentSnackBar();
    if (update != null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          duration: const Duration(seconds: 20),
          showCloseIcon: true,
          content: Text('Update available: ${update.version}'),
          action: SnackBarAction(
            label: 'Download',
            onPressed: () {
              update!.open();
            },
          ),
        ),
      );
    } else if (error != null) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          showCloseIcon: true,
          content: Text('Error: $error'),
        ),
      );
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          showCloseIcon: true,
          content: Text('No updates available'),
        ),
      );
    }
  }

  void _handleMenuSelection(MenuSelection value) async {
    final packageInfo = await PackageInfo.fromPlatform();
    if (!mounted) return;
    switch (value) {
      case MenuSelection.about:
        showAboutDialog(
          context: context,
          applicationName: packageInfo.appName,
          applicationVersion:
              "${packageInfo.version}+${packageInfo.buildNumber}",
          children: const [
            TextLink(
              url: Strings.repoURL,
              text: 'View on GitHub',
            )
          ],
        );
      case MenuSelection.showMessage:
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Hello from the menu!'),
          ),
        );
      case MenuSelection.updates:
        _checkForUpdates();
    }
  }

  @override
  Widget build(BuildContext context) {
    return PlatformMenuBar(
      menus: [
        PlatformMenu(
          label: 'Alic',
          menus: [
            PlatformMenuItemGroup(
              members: [
                PlatformMenuItem(
                  label: 'About',
                  onSelected: () {
                    _handleMenuSelection(MenuSelection.about);
                  },
                ),
                PlatformMenuItem(
                  label: 'Check for Updates...',
                  onSelected: () {
                    _handleMenuSelection(MenuSelection.updates);
                  },
                ),
              ],
            ),
            const PlatformProvidedMenuItem(
                type: PlatformProvidedMenuItemType.quit),
          ],
        ),
      ],
      child: widget.child,
    );
  }
}
