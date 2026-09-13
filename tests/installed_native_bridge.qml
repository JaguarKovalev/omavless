// TEST TOOL ONLY. Actual installed Service; no fake backend, socket or responses.
import QtQuick
import Quickshell

ShellRoot {
    id: test
    property var service: null
    property string scratch: Quickshell.env("OMAVLESS_R6_BRIDGE_SCRATCH")
    property int step: 0
    property string stage: "installed_service_readiness"
    property bool started: false
    property bool failed: false
    property string profileId: ""
    property string subscriptionId: ""
    property string subscriptionUrl: ""
    property bool subscriptionReady: false
    property bool subscriptionEditing: false

    function fail() {
        if (failed) return
        failed = true
        console.log("BRIDGE FAIL: " + stage)
        // The fixed-purpose shell driver stops only its own Quickshell child
        // after observing this terminal marker. No process-wide QML kill API.
        poll.stop()
        deadline.stop()
    }
    function next(name) {
        console.log("BRIDGE PASS: " + stage)
        stage = name
        started = false
        step++
    }
    function admit(ok) { if (!ok) fail() }
    function settled() {
        return service.nativeCanAct && service.nativePending === null
            && !service.nativeActionRunning && !service.nativeOutcomeUnknown
    }
    function check() {
        if (!service || failed) return
        if (service.nativeOutcomeUnknown || service.nativeActionCode !== "") { fail(); return }
        switch (step) {
        case 0:
            if (!settled()) return
            if (service.nativeSnapshot.desired.connected
                || service.nativeSnapshot.profiles.length !== 0
                || service.nativeSnapshot.subscriptions.length !== 0) { fail(); return }
            next("profile_file_preview_and_confirmation")
            break
        case 1:
            if (!started) { started = true; admit(service.startNativeImport("file", scratch + "/profile.txt")); return }
            if (!service._nativeImportContext || !service._nativeImportContext.ready) return
            admit(service.confirmNativeImport("Synthetic bridge profile"))
            next("profile_import_persisted")
            break
        case 2:
            if (!settled() || service.nativeSnapshot.profiles.length !== 1) return
            profileId = service.nativeSnapshot.profiles[0].id
            next("subscription_file_preview_and_confirmation")
            break
        case 3:
            if (!started) { started = true; admit(service.startNativeImport("file", scratch + "/subscription.txt")); return }
            if (!subscriptionReady) return
            if (subscriptionEditing || !subscriptionUrl.startsWith("http://127.0.0.1:")) { fail(); return }
            admit(service.requestNativeSubscriptionAction("subscription-add", "", "Synthetic bridge feed", subscriptionUrl))
            next("subscription_http_add")
            break
        case 4:
            if (!settled() || service.nativeSnapshot.subscriptions.length !== 1) return
            if (service.nativeSnapshot.profiles.length !== 2) { fail(); return }
            subscriptionId = service.nativeSnapshot.subscriptions[0].id
            next("subscription_http_refresh")
            break
        case 5:
            if (!started) { started = true; admit(service.requestNativeSubscriptionAction("subscription-refresh", subscriptionId, "", "")); return }
            if (!settled()) return
            if (service.nativeSubscriptionCode !== "saved") { fail(); return }
            next("subscription_editor_read")
            break
        case 6:
            if (!started) {
                started = true; subscriptionReady = false
                admit(service.startNativeSubscription(subscriptionId, "", "", "manual")); return
            }
            if (!subscriptionReady) return
            if (!subscriptionEditing || !subscriptionUrl.startsWith("http://127.0.0.1:")) { fail(); return }
            admit(service.requestNativeSubscriptionAction("subscription-update", subscriptionId,
                "Synthetic bridge edited", subscriptionUrl))
            next("subscription_http_update")
            break
        case 7:
            if (!settled()) return
            if (service.nativeSnapshot.subscriptions[0].name !== "Synthetic bridge edited") { fail(); return }
            next("subscription_delete")
            break
        case 8:
            if (!started) {
                started = true
                admit(service.requestNativeSubscriptionAction("subscription-delete", subscriptionId, "", "", {
                    id: subscriptionId, instanceId: service.nativeSnapshot.instanceId,
                    revision: service.nativeSnapshot.revision
                })); return
            }
            if (!settled() || service.nativeSnapshot.subscriptions.length !== 0) return
            if (service.nativeSnapshot.profiles.length !== 1) { fail(); return }
            next("routing_rule_add")
            break
        case 9:
            if (!started) {
                started = true
                admit(service.requestNativeRoutingAction("custom-rule-add", "domain\ndirect\nexample.invalid")); return
            }
            if (!settled()) return
            service.nativeRoutingToolsVisible = true
            next("routing_rule_read")
            break
        case 10:
            if (service.nativeRoutingBusy || !service._nativeRulesFence) return
            if (service.customRules.length !== 1 || service.customRules[0].value !== "example.invalid") { fail(); return }
            admit(service.requestNativeRoutingAction("custom-rule-delete", service.customRules[0].id))
            next("routing_rule_delete")
            break
        case 11:
            if (!settled() || service.nativeRoutingBusy) return
            if (!started) { started = true; service.loadCustomRules(); return }
            if (!service._nativeRulesFence) return
            if (service.customRules.length !== 0) { fail(); return }
            service.nativeRoutingToolsVisible = false
            next("profile_export_native_writer")
            break
        case 12:
            if (!started) {
                started = true
                admit(service.startNativeFileExport({uuid:profileId}, scratch + "/export.txt")); return
            }
            if (service.nativeFileExportStatus === "pending") return
            if (service.nativeFileExportStatus !== "saved") { fail(); return }
            next("support_report_matching_parser_and_writer")
            break
        case 13:
            if (!started) { started = true; admit(service.startNativeReportFileExport(scratch + "/report.json")); return }
            if (service.nativeFileExportStatus === "pending") return
            if (service.nativeFileExportStatus !== "saved") { fail(); return }
            next("profile_delete")
            break
        case 14:
            if (!started) { started = true; admit(service.requestNativeProfileAction("profile-delete", profileId)); return }
            if (!settled() || service.nativeSnapshot.profiles.length !== 0) return
            next("final_native_disconnected_state")
            break
        case 15:
            if (!settled()) return
            if (service.nativeSnapshot.desired.connected || service.nativeSnapshot.subscriptions.length !== 0) { fail(); return }
            console.log("BRIDGE PASS: " + stage)
            console.log("INSTALLED QML BRIDGE MATRIX PASS")
            poll.stop()
            deadline.stop()
        }
    }
    Component.onCompleted: {
        if (Quickshell.env("HOME") !== "/home/omavless-r6-domain"
            || !scratch.startsWith("/home/omavless-r6-domain/bridge-evidence.")) { fail(); return }
        var component = Qt.createComponent("file:///home/omavless-r6-domain/.config/omarchy/plugins/kdk.omavless/plugin/Service.qml")
        if (component.status !== Component.Ready) { fail(); return }
        service = component.createObject(test, {panelVisible:true})
        if (!service) { fail(); return }
        service.nativeSubscriptionReady.connect(function(name, url, kind, editing) {
            subscriptionUrl = url
            subscriptionEditing = editing
            subscriptionReady = true
        })
    }
    Timer { id: poll; interval: 100; repeat: true; running: true; onTriggered: { try { test.check() } catch (_) { test.fail() } } }
    Timer { id: deadline; interval: 120000; running: true; onTriggered: test.fail() }
}
