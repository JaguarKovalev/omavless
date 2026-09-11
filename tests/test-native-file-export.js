// SPDX-License-Identifier: MIT
const assert = require('node:assert/strict'), fs = require('node:fs'), vm = require('node:vm');
const source = fs.readFileSync(__dirname + '/../plugin/Service.qml', 'utf8');
const panel = fs.readFileSync(__dirname + '/../plugin/Panel.qml', 'utf8');
const parser = vm.createContext({});
vm.runInContext(fs.readFileSync(__dirname + '/../plugin/NativeSnapshot.js', 'utf8'), parser);
const presentation = vm.createContext({});
vm.runInContext(fs.readFileSync(__dirname + '/../plugin/NativePresentation.js', 'utf8'), presentation);
let count = 0;
function test(name, fn) { try { fn(); count++; } catch (e) { e.message = name + ': ' + e.message; throw e; } }
function context() {
  const c = vm.createContext({NativeSnapshot:parser, nativeCanAct:true, nativeSnapshot:{instanceId:'instance',revision:7,profiles:[{id:'one'},{id:'two'}]}, nativeFileExportContext:null, nativeFileExportProcess:null, nativeFileExportStatus:'',backendPath:'/synthetic/backend.sh'});
  c.root = c;
  // QML initial properties cross a QVariant map; object identity is not retained.
  c.nativeFileExportComponent = {createObject:(_, properties) => ({...JSON.parse(JSON.stringify(properties)), destroy(){this.destroyed=true}})};
  for (const name of ['validNativeExportPath','nativeFileExportCurrent','startNativeFileExport','startNativeReportFileExport','finishNativeFileExport']) {
    const start=source.indexOf('  function '+name+'('), end=source.indexOf('\n  }', start)+4;
    assert(start>=0); vm.runInContext(source.slice(start,end), c);
  }
  return c;
}
const frame = (revision=7) => JSON.stringify({api:'omavless.control',version:1,id:'export',ok:true,revision,result:{format:'uri',content:'synthetic-private-content'}});
test('absolute bounded path without controls; shell syntax remains inert data', () => {
  const c=context();
  for(const path of ['/tmp/private file','/tmp/$(do-not-run)','/tmp/a;not-a-command']) assert(c.validNativeExportPath(path));
  for(const path of ['', 'relative', '/tmp/a\nb', '/tmp/a\r', '/tmp/a\0', '/'+'x'.repeat(4096), '/'+'界'.repeat(1400)]) assert(!c.validNativeExportPath(path));
});
test('export selected record only and no destination argv', () => {
  const c=context(); assert(c.startNativeFileExport({uuid:'two'}, '/tmp/private-destination'));
  const p=c.nativeFileExportProcess;
  assert.equal(p.command.join('|'),'bash|/synthetic/backend.sh|native-profile-file|two');
  assert(!p.command.join('|').includes('private-destination'));
  assert(!c.startNativeFileExport({uuid:'one'}, '/tmp/another'));
});
test('private content and destination go only to writer stdin', () => {
  const c=context(); c.startNativeFileExport({uuid:'one'}, '/tmp/private-destination');
  const p=c.nativeFileExportProcess; c.finishNativeFileExport(p,0,frame());
  assert(p.destroyed);
  const writer=c.nativeFileExportProcess;
  assert.equal(writer.command.join('|'),'bash|/synthetic/backend.sh|native-export-write');
  assert.equal(writer.privateInput,'/tmp/private-destination\nsynthetic-private-content');
  writer.writeAdmitted=true;
  c.finishNativeFileExport(writer,0,''); assert.equal(c.nativeFileExportStatus,'saved'); assert.equal(c.nativeFileExportContext,null); assert(writer.destroyed);
});
test('denied writer admission cannot report saved even for zero exit', () => {
  const c=context(); c.startNativeFileExport({uuid:'one'},'/tmp/destination');
  c.finishNativeFileExport(c.nativeFileExportProcess,0,frame());
  const writer=c.nativeFileExportProcess; writer.writeAdmitted=false;
  c.finishNativeFileExport(writer,0,''); assert.equal(c.nativeFileExportStatus,'failed');
});
test('no file write after stale revision, instance, admission or deleted record', () => {
  for(const change of [c=>c.nativeSnapshot.revision++,c=>c.nativeSnapshot.instanceId='replacement',c=>c.nativeCanAct=false,c=>c.nativeSnapshot.profiles=[]]) {
    const c=context(); c.startNativeFileExport({uuid:'one'},'/tmp/destination'); const p=c.nativeFileExportProcess;
    change(c); c.finishNativeFileExport(p,0,frame()); assert.equal(c.nativeFileExportProcess,null); assert.equal(c.nativeFileExportStatus,'failed'); assert(p.destroyed);
  }
});
test('failure malformed and mismatched reply do not release private data', () => {
  for(const [code,reply] of [[1,frame()],[0,'password=synthetic-private-content'],[0,frame(8)]]) {
    const c=context(); c.startNativeFileExport({uuid:'one'},'/tmp/destination'); c.finishNativeFileExport(c.nativeFileExportProcess,code,reply);
    assert.equal(c.nativeFileExportStatus,'failed'); assert.equal(c.nativeFileExportContext,null);
  }
});
test('explicit confirmation UI and Process cleanup guards', () => {
  assert(panel.includes('fileMode: Dialogs.FileDialog.SaveFile'));
  assert(panel.includes('"native.fileExport.saveTitle"'));
  assert(!panel.includes('DontConfirmOverwrite'));
  assert(!panel.includes('exportWindow.value'));
  assert(panel.includes('context.revision !== vless.nativeSnapshot.revision'));
  assert(panel.includes('context.instanceId !== vless.nativeSnapshot.instanceId'));
  assert(panel.includes('exportWindow.selectedFile = ""'));
  // Other restored profile actions may follow Edit; export must retain its
  // position without freezing the complete keyboard navigation array.
  assert(panel.includes('rowQr, rowExport, rowEdit'));
  const component=source.slice(source.indexOf('id: nativeFileExportComponent'),source.indexOf('id: nativeQrRenderComponent'));
  assert(component.includes('property Timer watchdog: Timer'));
  assert(component.includes('writeAdmitted = root.nativeFileExportCurrent(context)'));
  assert(component.includes('if (writeAdmitted) write(privateInput)'));
  assert(component.includes('privateInput = ""'));
  assert(!component.includes('console.'));
});
test('chooser local URL conversion is bounded and never interprets shell syntax', () => {
  for (const path of ['/tmp/report.json','/tmp/Пример файла #1%.conf','/tmp/$(false);a.conf']) {
    const url='file://' + path.split('/').map(encodeURIComponent).join('/');
    assert.equal(presentation.exportLocalPath(url),path);
  }
  for(const url of ['https://example.invalid/file','file://remote/tmp/a','file:/tmp/a','file:///tmp/a?query','file:///tmp/a#fragment','file:///tmp/%FF','file:///tmp/%00','file:///tmp/a%0Ab','file:///tmp/%2E%2E/a','file:///tmp/./a','file:///'+'x'.repeat(4097)]) assert.equal(presentation.exportLocalPath(url),'');
  assert.equal(presentation.exportLocalPath('file:///tmp/%252e%252e'),'/tmp/%2e%2e');
  assert.equal(presentation.exportDefaultUrl('/home/example','report'),'file:///home/example/omavless-report.json');
  assert.equal(presentation.exportDefaultUrl('/home/example/','profile'),'file:///home/example/omavless-profile.conf');
  assert.equal(presentation.exportDefaultUrl('/home/example','private-profile-name'),'');
  assert.equal(presentation.exportDefaultUrl('relative','report'),'');
});
function panelContext() {
  const vless=context();
  const c=vm.createContext({vless, NativePresentation:presentation,pendingFileExport:null,
    StandardPaths:{HomeLocation:1,writableLocation:()=>'/home/example'},Qt:{callLater:fn=>fn()},
    exportWindow:{selectedFile:'',open(){this.visible=true},close(){this.visible=false}},
    close(){this.closed=true},open(){this.opened=true}});
  c.root=c;
  for (const name of ['requestFileExport','requestReportExport','openFileExportChooser','cancelFileExport','confirmFileExport']) {
    const start=panel.indexOf('  function '+name+'('), end=panel.indexOf('\n  }',start)+4;
    vm.runInContext(panel.slice(start,end),c);
  }
  return c;
}
test('Save As prefills generic name, serializes selection and Cancel never reads or writes', () => {
  const c=panelContext();c.requestReportExport();
  assert(c.exportWindow.visible && c.closed);
  assert.equal(c.exportWindow.selectedFile,'file:///home/example/omavless-report.json');
  assert.equal(c.exportWindow.defaultSuffix,'json');
  const context=c.pendingFileExport;c.requestFileExport({uuid:'one'});
  assert.equal(c.pendingFileExport,context);
  assert.equal(c.vless.nativeFileExportProcess,null);
  c.cancelFileExport();
  assert.equal(c.pendingFileExport,null);assert.equal(c.exportWindow.selectedFile,'');
  assert.equal(c.vless.nativeFileExportProcess,null);assert.equal(c.vless.nativeFileExportStatus,'');
  c.requestFileExport({uuid:'one'});
  assert.equal(c.exportWindow.defaultSuffix,'conf');
  assert(c.exportWindow.selectedFile.endsWith('/omavless-profile.conf'));
});
test('chooser confirmation preserves writer boundary and clears private selection', () => {
  const c=panelContext();c.requestReportExport();
  c.exportWindow.selectedFile='file:///tmp/report%20name.json';c.confirmFileExport();
  assert.equal(c.vless.nativeFileExportContext.path,'/tmp/report name.json');
  assert.equal(c.vless.nativeFileExportProcess.command.at(-1),'native-support-report');
  assert.equal(c.exportWindow.selectedFile,'');assert.equal(c.pendingFileExport,null);assert(c.opened);
});
test('stale, revoked and invalid chooser results do not reach exporter', () => {
  for(const change of [c=>c.vless.nativeSnapshot.revision++,c=>c.vless.nativeSnapshot.instanceId='new',c=>c.vless.nativeCanAct=false,c=>c.exportWindow.selectedFile='file://remote/a',c=>c.exportWindow.selectedFile='file:///tmp/%00']) {
    const c=panelContext();c.requestReportExport();change(c);c.confirmFileExport();
    assert.equal(c.vless.nativeFileExportProcess,null);assert.equal(c.vless.nativeFileExportStatus,'failed');
    assert.equal(c.exportWindow.selectedFile,'');assert.equal(c.pendingFileExport,null);assert(c.opened);
  }
});
test('report export uses canonical report projection, not profile export', () => {
  const c=context(); let parsed=0;
  c.NativeSnapshot={editorText:parser.editorText,configurationReport:(raw,revision)=>{parsed++;assert.equal(raw,'synthetic response');assert.equal(revision,7);return '{"safe":true}';},parseQrExport:()=>{throw Error('wrong exporter')}};
  assert(c.startNativeReportFileExport('/tmp/report.json'));
  const reader=c.nativeFileExportProcess;
  assert.equal(reader.command.join('|'),'bash|/synthetic/backend.sh|native-support-report');
  c.finishNativeFileExport(reader,0,'synthetic response');
  assert.equal(parsed,1);assert.equal(c.nativeFileExportProcess.privateInput,'/tmp/report.json\n{"safe":true}');
  assert(c.nativeFileExportCurrent(c.nativeFileExportContext));
  c.nativeSnapshot.revision++;assert(!c.nativeFileExportCurrent(c.nativeFileExportContext));
});
test('stale and malformed reports never reach writer', () => {
  for(const stale of [false,true]) {
    const c=context();c.startNativeReportFileExport('/tmp/report.json');
    if(stale)c.nativeSnapshot.instanceId='replacement';
    c.finishNativeFileExport(c.nativeFileExportProcess,0,'secret-invalid-report');
    assert.equal(c.nativeFileExportProcess,null);assert.equal(c.nativeFileExportStatus,'failed');
  }
  assert(panel.includes('context.kind === "report") vless.startNativeReportFileExport'));
  assert(panel.slice(panel.indexOf('function panelTabTargets()'),panel.indexOf('function availablePanelTabTargets()')).includes('nativeSupportSetting.exportFocusTarget'));
});
console.log(`${count} native file-export tests passed`);
