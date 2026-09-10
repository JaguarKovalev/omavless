// SPDX-License-Identifier: MIT
const fs = require('node:fs'), vm = require('node:vm'), assert = require('node:assert/strict');
const service = fs.readFileSync(__dirname + '/../plugin/Service.qml', 'utf8');
const panel = fs.readFileSync(__dirname + '/../plugin/Panel.qml', 'utf8');
function context() {
  const c = vm.createContext({nativeCanAct:true,nativeEditorRunning:false,nativeImportBusy:false,
    nativeQuitFailed:true,nativeQuitPending:false,nativeQuitProcess:{running:false},_nativeOperationSerial:0,
    nativeSnapshot:{instanceId:'synthetic-instance',revision:7},backendPath:'/synthetic/backend.sh'});
  const start=service.indexOf('  function quitNativeApplication('),end=service.indexOf('\n  }',start)+4;
  vm.runInContext(service.slice(start,end),c);return c;
}
let count=0; function test(name,f) {try{f();count++}catch(e){e.message=name+': '+e.message;throw e}}
test('exit is a fixed fenced command with no store data in argv',()=>{
  const c=context(); assert(c.quitNativeApplication());assert(c.nativeQuitProcess.running);
  assert(c.nativeQuitPending);
  assert.equal(c.nativeQuitFailed,false);
  assert.equal(c.nativeQuitProcess.command.slice(0,5).join('|'),'bash|/synthetic/backend.sh|native-quit|synthetic-instance|7');
  assert.match(c.nativeQuitProcess.command[5],/^quit-[a-z0-9]+-1$/);
});
test('unsafe or busy UI cannot submit Quit',()=>{
  for(const [key,value] of [['nativeCanAct',false],['nativeEditorRunning',true],['nativeImportBusy',true]]){
    const c=context();c[key]=value;assert.equal(c.quitNativeApplication(),false);assert.equal(c.nativeQuitProcess.running,false);
  }
});
test('quit blocks new UI actions and does not kill authorization on short timer',()=>{
  assert.match(service,/nativeCanAct: nativeFactsCurrent && !nativePending && !nativeQuitting/);
  assert.match(service,/nativeQuitting: nativeQuitPending \|\| nativeQuitProcess.running/);
  const quit=service.slice(service.indexOf('id: nativeQuitProcess'),service.indexOf('property var nativeTestResult'));
  assert(!quit.includes('Timer {'));assert(!quit.includes('signal('));assert(!quit.includes('output.text'));
  assert.match(quit,/nativeQuitFailed = code !== 0/);
});
test('explicit settings confirmation is separate from ordinary panel close',()=>{
  assert.match(panel,/implicitHeight: settingContent.implicitHeight \+ Style.space\(16\)/);
  assert.match(panel,/nativeQuitSetting.focusTarget/);
  assert.match(panel,/id: nativeQuitSetting[\s\S]*onAction: root.quitConfirmation = true/);
  const dialog=panel.slice(panel.indexOf('id: quitDialog'),panel.indexOf('id: deleteDialog'));
  assert.match(dialog,/selectedIndex = 0/);assert.match(dialog,/onCanceled: root.quitConfirmation = false/);
  assert.match(dialog,/onConfirmed:[\s\S]*vless.quitNativeApplication\(\)/);
  assert.match(panel,/modalInputActive:[\s\S]*\|\| quitConfirmation/);
  const close=panel.slice(panel.indexOf('onOpenedChanged: {'),panel.indexOf('Connections {',panel.indexOf('onOpenedChanged: {')));
  assert(!close.includes('quitNativeApplication()'));
});
test('complete English/Russian fixed text, private input not a translation source',()=>{
  const i18n=vm.createContext({});vm.runInContext(fs.readFileSync(__dirname+'/../plugin/I18n.js','utf8'),i18n);
  for(const key of ['title','action','running','description','confirmation','failed']) {
    for(const locale of ['en','ru']) {
      const text=i18n.translate('native.quit.'+key,locale,{});assert(text.length>0);assert(!text.includes('Missing translation'));
    }
  }
});
console.log(`${count} native full Quit tests passed`);
