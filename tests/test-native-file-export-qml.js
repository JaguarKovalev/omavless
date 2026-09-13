// SPDX-License-Identifier: MIT
// Opt-in real Quickshell Process composition, synthetic input only; no runtime/store.
const fs = require('node:fs'), os = require('node:os'), path = require('node:path');
const cp = require('node:child_process'), assert = require('node:assert/strict');
const source = fs.readFileSync(path.join(__dirname, '../plugin/Service.qml'), 'utf8');
const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'omavless-export-qml-'));
try {
  const backend = path.join(dir, 'backend.sh');
  fs.writeFileSync(backend, `case "$1" in
native-profile-file) printf '%s\\n' '{"api":"omavless.control","version":1,"id":"export","ok":true,"revision":7,"result":{"format":"uri","content":"synthetic-content"}}';;
native-export-write) input=$(cat); [ "$input" = '/synthetic/destination
synthetic-content' ];;
*) exit 2;;
esac
`, {mode:0o600});
  const functions = ['validNativeExportPath','nativeFileExportCurrent','startNativeFileExport','finishNativeFileExport'].map(name => {
    const start = source.indexOf('  function '+name+'(');
    assert(start >= 0);
    return source.slice(start, source.indexOf('\n  }',start)+4);
  }).join('\n');
  const start = source.lastIndexOf('  Component {', source.indexOf('id: nativeFileExportComponent'));
  const component = source.slice(start,source.indexOf('\n  Component {',start+1));
  const qml = `import QtQuick
import Quickshell
import Quickshell.Io
import ${JSON.stringify(path.join(__dirname,'../plugin/NativeSnapshot.js'))} as NativeSnapshot
ShellRoot {
 id: root
 property bool nativeCanAct: true
 property var nativeSnapshot: ({instanceId:"instance", revision:7, profiles:[{id:"one"}]})
 property var nativeFileExportContext: null
 property var nativeFileExportProcess: null
 property string nativeFileExportStatus: ""
 property string nativeFileExportKind: ""
 property string backendPath: ${JSON.stringify(backend)}
 ${functions}
 ${component}
 Component.onCompleted: startNativeFileExport({uuid:"one"}, "/synthetic/destination")
 onNativeFileExportStatusChanged: {
   if (nativeFileExportStatus === "saved" || nativeFileExportStatus === "failed") {
     console.log("EXPORT_RESULT=" + nativeFileExportStatus)
     Qt.callLater(function() { Qt.quit() })
   }
 }
 Timer { interval:5000; running:true; onTriggered: Qt.quit() }
}
`;
  fs.writeFileSync(path.join(dir,'shell.qml'),qml,{mode:0o600});
  const result = cp.spawnSync('quickshell',['-p',path.join(dir,'shell.qml')],{encoding:'utf8',timeout:10000,env:{...process.env,QT_QPA_PLATFORM:'offscreen'}});
  const output = result.stdout + result.stderr;
  assert.match(output,/EXPORT_RESULT=saved/,output);
  console.log('Real QML file-export process test passed');
} finally { fs.rmSync(dir,{recursive:true,force:true}); }
