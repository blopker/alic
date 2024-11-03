import Cocoa
import FlutterMacOS

@main
class AppDelegate: FlutterAppDelegate {
    override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
    override func application(_ application: NSApplication, open urls: [URL]) {
        guard
            let flutterViewController = mainFlutterWindow?.contentViewController
                as? FlutterViewController
        else {
            return
        }

        let channel = FlutterMethodChannel(
            name: "io.kbl.alic", binaryMessenger: flutterViewController.engine.binaryMessenger)

        for url in urls {
            channel.invokeMethod("openImage", arguments: url.path)
        }
    }
}
