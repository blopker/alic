import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
    override func awakeFromNib() {
        let flutterViewController = FlutterViewController()
        let windowFrame = self.frame
        self.contentViewController = flutterViewController
        self.setFrame(windowFrame, display: true)
        self.setContentSize(NSSize(width: 600, height: 400))

        let trashChannel = FlutterMethodChannel(
            name: "io.kbl.alic",
            binaryMessenger: flutterViewController.engine.binaryMessenger)
        trashChannel.setMethodCallHandler { (call, result) in
            switch call.method {
            case "trash":
                if let args = call.arguments as? [String: Any],
                    let filePath = args["filePath"] as? String
                {
                    self.trashItem(filePath: filePath, result: result)
                } else {
                    result(
                        FlutterError(
                            code: "INVALID_ARGUMENTS", message: "File path is required",
                            details: nil))
                }
            default:
                result(FlutterMethodNotImplemented)
            }
        }

        RegisterGeneratedPlugins(registry: flutterViewController)
        super.awakeFromNib()
    }

    private func trashItem(filePath: String, result: FlutterResult) {
        let fileURL = URL(fileURLWithPath: filePath)
        do {
            var resultingURL: NSURL?
            try FileManager.default.trashItem(at: fileURL, resultingItemURL: &resultingURL)
            result(nil)
        } catch {
            result(
                FlutterError(
                    code: "TRASH_FAILED",
                    message: "Failed to trash the item: \(error.localizedDescription)", details: nil
                ))
        }
    }
}
