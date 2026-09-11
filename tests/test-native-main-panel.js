// SPDX-License-Identifier: MIT
// Actual presentation functions, synthetic metadata only.
const assert=require('node:assert/strict'), fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const source=fs.readFileSync(path.join(__dirname,'../plugin/Panel.qml'),'utf8');
const presentation=vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname,'../plugin/NativePresentation.js'),'utf8'),presentation);
const standalone={id:'local',name:'Local',protocol:'vless',subscriptionId:'',favorite:false,missing:false};
const managed={id:'managed',name:'Match',protocol:'vless',subscriptionId:'sub',favorite:true,missing:false};
function context(){
  const calls=[];
  const c=vm.createContext({NativePresentation:presentation,subscriptionSortModes:{},profileFilter:'',nativeExpanded:{},nativeSelectedProfile:'local',nativeSubscriptionId:'',pendingSubscriptionDelete:null,
    nativeView:{state:'disconnected',connected:false,mode:'rule',lastProfileId:'local',profiles:[standalone,managed],subscriptions:[{id:'sub',name:'Synthetic'}]},
    vless:{nativeCanAct:true,nativeOwner:true,refreshNativeDesktopCapabilities:()=>calls.push(['desktop-capabilities']),requestNativeAction:(...args)=>calls.push(args),nativeSnapshot:{instanceId:'instance',revision:4}},page:'main',nativeFlick:{contentY:90},
    nativeCursor:-1,nativeProfiles:{itemAt:()=>null},Qt:{callLater:f=>f()}});
  c.root=c;c.calls=calls;
  for(const name of ['nativeRecord','buildNativeRows','sortNativeProbeProfiles','sortNativeProbeResults','subscriptionSortMode','toggleNativeSubscription','nativeToggleConnection','openSettings','openSubscriptions','browseNativeSubscription','moveNativeCursor','activateNativeCursor','requestNativeSubscriptionDelete','editSubscription']){
    const start=source.indexOf('  function '+name+'('),end=source.indexOf('\n  }',start)+4;
    assert(start>=0&&end>start);vm.runInContext(source.slice(start,end),c);
  }
  Object.defineProperty(c,'nativeRows',{get:()=>c.buildNativeRows()});
  Object.defineProperty(c,'nativeSubscription',{get:()=>c.nativeView.subscriptions.find(s=>s.id===c.nativeSubscriptionId)||null});
  return c;
}
let count=0;function test(name,f){try{f();count++;}catch(e){e.message=name+': '+e.message;throw e;}}
test('reopening main resets both scrollers without erasing search',()=>{
  const c=context();c.opened=true;c.panelFlick={contentY:210};c.nativeFlick.contentY=470;c.profileFilter='kept';
  c.vless.refresh=()=>{};
  for(const name of ['subscriptionPrompt','routingPresetPrompt','startupPrompt','onboardingWizard','routingToolsPrompt']) c[name]={dismiss(){}};
  for(const name of ['cancelImport','cancelRename','cancelFileExport','syncNativeOnboarding']) c[name]=()=>{};
  c.keyCatcher={forceActiveFocus(){}};
  const start=source.indexOf('  onOpenedChanged: {');
  const body=source.slice(start+'  onOpenedChanged: {'.length,source.indexOf('\n  }',start));
  vm.runInContext(body,c);
  assert.equal(c.page,'main');assert.equal(c.nativeFlick.contentY,0);assert.equal(c.panelFlick.contentY,0);assert.equal(c.profileFilter,'kept');
});
test('standalone and collapsed subscriptions preserve grouping',()=>{
  const c=context();let rows=c.buildNativeRows();assert.equal(rows.length,2);assert.equal(rows[0].profile.id,'local');assert.equal(rows[1].subscription.id,'sub');
  c.toggleNativeSubscription('sub');rows=c.buildNativeRows();assert.equal(rows.length,3);assert.equal(rows[2].profile.id,'managed');
  c.toggleNativeSubscription('sub');assert.equal(c.buildNativeRows().length,2);
});
test('search expands matching subscription without mutating stored expansion',()=>{
  const c=context();c.profileFilter='MATCH';const rows=c.buildNativeRows();assert.equal(rows.length,2);assert.equal(rows[0].expanded,true);assert.equal(rows[1].profile.id,'managed');assert.equal(c.nativeExpanded.sub,undefined);
  c.profileFilter='absent';assert.equal(c.buildNativeRows().length,0);
});
test('power toggles only verified connected or disconnected state',()=>{
  const c=context();c.nativeToggleConnection();assert.deepEqual(c.calls.pop(),['connect','local','rule']);
  c.nativeView.connected=true;c.nativeView.state='connected';c.nativeToggleConnection();assert.deepEqual(c.calls.pop(),['disconnect','','']);
  c.nativeView.connected=false;c.nativeView.state='unavailable';c.nativeToggleConnection();assert.equal(c.calls.length,0);
  c.nativeView.state='disconnected';c.vless.nativeCanAct=false;c.nativeToggleConnection();assert.equal(c.calls.length,0);
});
test('settings opens and reads helper inventory without runtime mutation, locale reuses existing persistence',()=>{
  const c=context();assert(c.openSettings());assert.equal(c.page,'settings');assert.equal(c.nativeFlick.contentY,0);assert.deepEqual(c.calls,[['desktop-capabilities']]);
  assert.match(source,/id: nativeLanguageRow[^\n]*onAction: root.cycleLanguageSetting\(\)/);
});
test('familiar header, truthful states, same-height focusable controls and no outer power box',()=>{
  const section=source.slice(source.indexOf('id: nativeHeader'),source.indexOf('id: nativeSettingsBack'));
  assert.match(section,/PanelHero/);assert.match(section,/title: "OmaVLESS"/);assert.match(section,/native.state./);
  assert.equal((section.match(/size: nativeHeader.controlHeight/g)||[]).length,2);
  assert.match(section,/height: nativeHeader.controlHeight/);assert.match(section,/cursorRing: false/);assert.match(section,/Keys.onReturnPressed/);
  assert(!source.includes('vless.nativeOwner ? "?"'));
});
test('private names stay plaintext and selected row exposes managed-safe actions',()=>{
  const section=source.slice(source.indexOf('id: nativeProfiles'),source.indexOf('AdvancedDiagnostics {'));
  assert.match(section,/subscription.name; textFormat: Text.PlainText/);
  assert.match(section,/profile.name : ""; textFormat: Text.PlainText/);
  for(const id of ['rowRename','rowPin','rowDelete','rowQr','rowEdit'])assert(section.includes('id: '+id));
  assert.equal(context().nativeRecord(managed).managed,true);assert.equal(context().nativeRecord(standalone).managed,false);
  assert.match(section,/visible: nativeRow.selected/);
});
test('compact import and inline actions retain labels and keyboard targets',()=>{
  for(const id of ['nativeImportClipboard','nativeImportFile'])
    assert.match(source,new RegExp('OmaNavigationButton \\{ id: '+id+';[^\\n]*tooltipText:[^\\n]*focusable: true'));
  for(const id of ['rowRename','rowPin','rowDelete','rowQr','rowEdit'])
    assert.match(source,new RegExp('PanelActionButton \\{ id: '+id+'; size: Style.space\\(24\\);[^\\n]*tooltipText:[^\\n]*focusable: true'));
  assert.match(source,/visible: !nativeRow.selected; text: nativeRow.isProfile/);
});
test('subscription detail navigation is local and clears search without changing connection',()=>{
  const c=context();assert(c.openSubscriptions());assert.equal(c.page,'subscriptions');assert.equal(c.nativeFlick.contentY,0);
  c.profileFilter='absent';assert.equal(c.browseNativeSubscription('missing'),false);
  assert(c.browseNativeSubscription('sub'));assert.equal(c.page,'subscription');assert.equal(c.profileFilter,'');
  assert.equal(c.nativeSubscriptionId,'sub');assert.equal(c.nativeCursor,-1);assert.equal(c.nativeFlick.contentY,0);
  assert.equal(c.nativeRows.length,1);assert.equal(c.nativeRows[0].profile.id,'managed');assert.equal(c.calls.length,0);
});
test('two equal-name subscriptions isolate detail search servers and action targets by id',()=>{
  const c=context();c.nativeView.subscriptions.push({id:'sub-two',name:'Synthetic'});
  c.nativeView.profiles.push({...managed,id:'managed-two',subscriptionId:'sub-two',name:'Match two'});
  const actions=[];c.vless.requestNativeSubscriptionAction=(...args)=>actions.push(args);c.vless.startNativeSubscription=(...args)=>actions.push(args);
  for(const [id,server] of [['sub','managed'],['sub-two','managed-two']]){
    assert(c.browseNativeSubscription(id));assert.equal(c.nativeSubscription.id,id);assert.equal(c.nativeRows.length,1);assert.equal(c.nativeRows[0].profile.id,server);
    c.profileFilter='MATCH';assert.equal(c.nativeRows.length,1);c.profileFilter='Local';assert.equal(c.nativeRows.length,0);c.profileFilter='';
    c.editSubscription(c.nativeSubscription);assert.equal(actions.pop()[0],id);
    assert(c.requestNativeSubscriptionDelete(c.nativeSubscription));assert.equal(c.pendingSubscriptionDelete.id,id);
    const line=source.split('\n').find(line=>line.includes('id: nativeSubscriptionRefresh;'));
    vm.runInContext(line.match(/onClicked: (.*?) \}$/)[1],c);assert.equal(actions.pop()[1],id);
  }
  assert.equal(c.calls.length,0);
});
test('pending delete identity does not retarget when viewing another subscription',()=>{
  const c=context();c.nativeView.subscriptions.push({id:'sub-two',name:'Synthetic'});c.browseNativeSubscription('sub');
  c.requestNativeSubscriptionDelete(c.nativeSubscription);const pending=c.pendingSubscriptionDelete;c.browseNativeSubscription('sub-two');
  assert.equal(c.pendingSubscriptionDelete,pending);assert.equal(pending.id,'sub');assert.equal(pending.instanceId,'instance');assert.equal(pending.revision,4);
});
test('deleted selected subscription returns overview and clears detail/search without mutations',()=>{
  const c=context();c.browseNativeSubscription('sub');c.profileFilter='private-search';
  c.syncNativeOnboarding=()=>{}; // Independently exercised by test-native-onboarding.js.
  c.nativeView.subscriptions=[];c.vless.nativeSnapshot={instanceId:'instance',revision:5,subscriptions:[],profiles:[standalone],desired:{connected:false},lastProfileId:'local'};
  const start=source.indexOf('    function onNativeSnapshotChanged() {'),end=source.indexOf('\n    }',start)+6;
  vm.runInContext(source.slice(start,end),c);c.onNativeSnapshotChanged();
  assert.equal(c.page,'subscriptions');assert.equal(c.nativeSubscriptionId,'');assert.equal(c.nativeSubscription,null);assert.equal(c.profileFilter,'');assert.equal(c.calls.length,0);
});
test('detail arrows and Enter act only on selected subscription servers',()=>{
  const c=context();c.browseNativeSubscription('sub');c.moveNativeCursor(1);assert.equal(c.nativeSelectedProfile,'managed');c.activateNativeCursor();
  assert.deepEqual(c.calls.pop(),['connect','managed','rule']);c.openSubscriptions();assert.equal(c.page,'subscriptions');assert.equal(c.nativeSubscriptionId,'');
  c.activateNativeCursor();assert.equal(c.calls.length,0);
});
test('overview has only open actions and detail owns one refresh edit delete set',()=>{
  const from=source.indexOf('id: nativeSubscriptions\n'),to=source.indexOf('id: nativeSubscriptionRefresh;',from);
  const overview=source.slice(from,source.indexOf('          PlainText {',from));
  assert(!overview.includes('requestNativeSubscriptionAction'));assert(!overview.includes('nativeSubscriptionEdit'));assert(!overview.includes('nativeSubscriptionDelete'));
  assert(to>from);for(const id of ['nativeSubscriptionRefresh','nativeSubscriptionEdit','nativeSubscriptionDelete'])assert.equal(source.split('id: '+id+';').length-1,1);
  assert.match(source,/page === "subscription" \? \[nativeSettingsBack, nativeSubscriptionTest, nativeSubscriptionSort, nativeSubscriptionRefresh, nativeSubscriptionEdit, nativeSubscriptionDelete, nativeSearch\]/);
});
test('detail Back and both Escape routes return overview rather than closing panel',()=>{
  for(const route of ['control','catcher','back']){
    const c=context();c.browseNativeSubscription('sub');let closed=0;c.close=()=>closed++;c.Qt.Key_Escape=27;
    if(route==='control'){
      const from=source.indexOf('  function handlePanelControlKey(event) {'),to=source.indexOf('\n  }',from)+4;
      vm.runInContext(source.slice(from,to),c);const event={key:27,accepted:false};c.handlePanelControlKey(event);assert.equal(event.accepted,true);
    } else if(route==='catcher'){
      const from=source.indexOf('      onCloseRequested: {'),to=source.indexOf('\n      }',from)+8;
      vm.runInContext('this.closeRequested = function() '+source.slice(from,to).replace('      onCloseRequested: ',''),c);c.closeRequested();
    } else {
      const line=source.split('\n').find(line=>line.includes('id: nativeSettingsBack;'));
      vm.runInContext(line.match(/onClicked: (\{.*\}) \}$/)[1],c);
    }
    assert.equal(c.page,'subscriptions');assert.equal(c.nativeSubscriptionId,'');assert.equal(closed,0);assert.equal(c.calls.length,0);
  }
});
test('arrow selection is local; Enter uses fenced native actions or toggles a group',()=>{
  const c=context();c.moveNativeCursor(1);assert.equal(c.nativeCursor,0);assert.equal(c.calls.length,0);
  c.activateNativeCursor();assert.deepEqual(c.calls.pop(),['connect','local','rule']);
  c.nativeView.connected=true;c.nativeView.state='connected';c.nativeView.activeId='local';
  c.activateNativeCursor();assert.deepEqual(c.calls.pop(),['disconnect','','']);
  c.moveNativeCursor(1);c.activateNativeCursor();assert.equal(c.nativeExpanded.sub,true);assert.equal(c.calls.length,0);
  c.moveNativeCursor(1);assert.equal(c.nativeSelectedProfile,'managed');c.vless.nativeCanAct=false;c.activateNativeCursor();assert.equal(c.calls.length,0);
  c.page='settings';const before=c.nativeCursor;c.moveNativeCursor(-1);c.activateNativeCursor();assert.equal(c.nativeCursor,before);
});
test('transition, missing and stale cursor never connect',()=>{
  const c=context();c.nativeCursor=999;c.activateNativeCursor();assert.equal(c.calls.length,0);
  c.nativeCursor=0;c.nativeView.state='starting';c.activateNativeCursor();assert.equal(c.calls.length,0);
  c.nativeView.state='disconnected';c.nativeView.profiles=[{...standalone,missing:true}];c.activateNativeCursor();assert.equal(c.calls.length,0);
});
test('settings and subscriptions use native metadata, no legacy mutations',()=>{
  const section=source.slice(source.indexOf('id: nativeFlick'),source.indexOf('AdvancedDiagnostics {'));
  assert.match(section,/id: nativeSubscriptions/);assert.match(section,/root.nativeView.subscriptions/);
  assert.match(section,/root.nativeModeLabel\("rule"\)/);
  assert(!section.includes('vless.useRoutingPreset('));
  assert.match(section,/onClicked: vless.refreshAllSubscriptions\(\)/);
  assert.match(source,/profileSearch.activeFocus \|\| nativeSearch.activeFocus/);
});
test('refresh-order null facts remain unavailable rather than throwing',()=>{
  const start=source.indexOf('  function nativeLocalStatus('),end=source.indexOf('\n  }',start)+4;
  const c=vm.createContext({vless:{nativeFactsCurrent:true,nativeObservation:null},textFor:key=>key});
  vm.runInContext(source.slice(start,end),c);
  assert.equal(c.nativeLocalStatus(),'native.localUnverified');
  c.vless.nativeObservation={facts:null};assert.equal(c.nativeLocalStatus(),'native.localUnverified');
  c.vless.nativeObservation.facts={ownedCoreRunning:false,visibleTunCount:0};assert.equal(c.nativeLocalStatus(),'native.localStopped');
});
test('focused native controls route arrows back to the list without stealing search input',()=>{
  const start=source.indexOf('  function handleNativeNavigationKey('),end=source.indexOf('\n  }',start)+4;
  const calls=[];
  const c=vm.createContext({vless:{nativeOwner:true},nativeSearch:{activeFocus:false},modalInputActive:false,page:'main',
    Qt:{Key_Up:1,Key_Down:2,Key_Tab:3,Key_Backtab:4,Key_Escape:5},
    moveNativeCursor:d=>calls.push(['move',d]),focusPanelControl:d=>calls.push(['tab',d]),
    keyCatcher:{forceActiveFocus:()=>calls.push(['focus'])},handlePanelControlKey:()=>calls.push(['control'])});
  c.root=c;vm.runInContext(source.slice(start,end),c);
  for(const [key,d] of [[1,-1],[2,1]]){const event={key,accepted:false};c.handleNativeNavigationKey(event);assert(event.accepted);assert.deepEqual(calls.splice(0),[['move',d],['focus']]);}
  c.page='settings';c.handleNativeNavigationKey({key:2});assert.deepEqual(calls.splice(0),[['tab',1]]);
  c.nativeSearch.activeFocus=true;const input={key:1,accepted:false};c.handleNativeNavigationKey(input);assert(!input.accepted);assert.equal(calls.length,0);
  c.nativeSearch.activeFocus=false;c.modalInputActive=true;c.handleNativeNavigationKey({key:2});assert.equal(calls.length,0);
  assert.match(source,/id: nativeFlick\s+Keys.onPressed: function\(event\) \{ root.handleNativeNavigationKey\(event\) \}/);
});
test('native latency sort is opt-in, preserves active/favorite and puts failed checks last',()=>{
 const c=context();c.nativeSubscriptionId='sub';c.subscriptionSortModes={sub:'pingAsc'};
 const values={slow:{reachable:true,latencyMs:90},fast:{reachable:true,latencyMs:10},failed:{reachable:false,latencyMs:-1}};
 c.vless.probeResult=id=>values[id]||null;
 const rows=['failed','slow','fast'].map(id=>({id,name:id,favorite:false}));
 assert.deepEqual(Array.from(c.sortNativeProbeProfiles(rows,'sub'),p=>p.id),['fast','slow','failed']);
 c.sortNativeProbeResults();assert.deepEqual(Array.from(c.sortNativeProbeProfiles(rows,'sub'),p=>p.id),['slow','fast','failed']);
 c.nativeView.activeId='failed';assert.equal(c.sortNativeProbeProfiles(rows,'sub')[0].id,'failed');
 assert(source.includes('id: nativeSubscriptionTest'));assert(source.includes('id: nativeSubscriptionSort'));
 assert(source.includes('root.nativeProbeLabel(nativeRow.profile.id)'));
});
test('main has immediate equal-width mode actions before traffic and profiles',()=>{
  const modes=source.slice(source.indexOf('id: nativeModeButtons'),source.indexOf('PanelSectionHeader',source.indexOf('id: nativeModeButtons')));
  assert.match(modes,/visible: root.page === "main" \|\| root.page === "settings"/);
  assert(source.indexOf('id: nativeModeButtons')<source.indexOf('id: nativeTrafficSection'));
  for(const [id,mode] of [['nativeGlobal','global'],['nativeRule','rule'],['nativeDirect','direct']]) {
    const line=modes.split('\n').find(s=>s.includes('id: '+id+';'));
    assert(line);assert(line.includes('Layout.preferredWidth: 1'));assert(line.includes('focusable: true'));
    assert(line.includes('enabled: vless.nativeCanAct && root.nativeView.mode !== "'+mode+'"'));
    assert(line.includes('onClicked: vless.requestNativeAction("mode", "", "'+mode+'")'));
  }
  assert(!source.includes('nativeModeSetting'));
  assert.match(source,/nativePowerControl, nativeGlobal, nativeRule, nativeDirect/);
});
test('profile frame is square padded content under the existing single scroller',()=>{
  const frame=source.slice(source.indexOf('id: nativeProfilesFrame'),source.indexOf('AdvancedDiagnostics {'));
  assert.match(frame,/radius: 0/);assert.match(frame,/borderSpec: Border.flat/);
  assert.match(frame,/implicitHeight: nativeProfilesContent.implicitHeight \+ Style.space\(20\)/);
  assert.match(frame,/nativeProfilesFrame.width - Style.space\(20\)/);
  assert(frame.includes('id: nativeSearch'));assert(frame.includes('id: nativeProfiles'));
  assert(!/Flickable\s*\{|ScrollView\s*\{/.test(frame));
});
test('main HTTPS Test is presentation-hidden without removing Ping or its probe implementation',()=>{
  assert.match(source,/readonly property bool showMainConnectionTest: false/);
  const block=source.slice(source.indexOf('id: nativeConnectionTestSection'),source.indexOf('id: nativeSettingsBack'));
  assert.match(block,/visible: root.showMainConnectionTest && root.page === "main" && root.nativeView.connected/);
  assert.match(source,/if \(page === "main" && showMainConnectionTest\) targets.push\(nativeTestButton\)/);
  assert(block.includes('onClicked: vless.startNativeConnectionTest()'));
  assert(source.includes('id: nativePingTest;'));
  assert.match(source,/RowLayout \{\s+Layout.fillWidth: true\s+spacing: Style.space\(12\)\s+DetailPair \{\s+Layout.fillWidth: true\s+label: root.textFor\("metric.ping"\)/);
});
test('settings sections group related controls in a stable visual order',()=>{
  const region=source.slice(source.indexOf('id: nativeColumn'),source.indexOf('AdvancedDiagnostics {'));
  const order=['settings.appearance','id: nativeLanguageRow','settings.connections','id: nativeModeButtons','id: nativeRoutingPresetSetting','id: nativeRoutingToolsSetting','id: nativeProvidersRefresh','id: nativeSubscriptionsSetting','settings.setup_startup','id: nativeCoreSetupRow','native.core.scope','id: nativeOnboardingSetting','id: nativeStartupSummaryRow','id: nativeHelpersRefresh','settings.diagnostics_privacy','id: nativeDiagnosticsSetting','id: nativeSupportSetting','id: nativeSupportExportSetting','id: nativeExitIpSetting','settings.application','id: nativeQuitSetting'];
  let previous=-1;for(const marker of order){const at=region.indexOf(marker);assert(at>previous,marker);previous=at;}
});
test('native Settings Tab order follows visual action order without hidden Test',()=>{
  const from=source.indexOf('  function panelTabTargets()'),to=source.indexOf('\n  function availablePanelTabTargets()',from);
  const names=Array.from(new Set(source.slice(from,to).match(/\bnative[A-Z]\w*/g)));
  const c=vm.createContext({page:'settings',vless:{nativeOwner:true},showMainConnectionTest:false});
  for(const name of names)c[name]={focusTarget:name,count:0};
  c.nativeSettingsBack='back';c.nativeRefresh='refresh';c.nativeGlobal='global';c.nativeRule='rule';c.nativeDirect='direct';
  vm.runInContext(source.slice(from,to),c);
  const result=Array.from(c.panelTabTargets());
  assert.deepEqual(result.slice(0,23),['back','refresh','nativeLanguageRow','nativeThroughputSetting','global','rule','direct','nativeRoutingPresetSetting','nativeRoutingToolsSetting','nativeProvidersRefresh','nativeSubscriptionsSetting','nativeCoreSetupRow','nativeOnboardingSetting','nativeStartupSummaryRow','nativeHelpersRefresh','nativeFileImportRow','nativeProfileEditorRow','nativeQrExportRow','nativeDiagnosticsSetting','nativeSupportSetting','nativeSupportExportSetting','nativeExitIpSetting','nativeQuitSetting']);
  assert(!result.includes(c.nativeTestButton));
});
test('native text uses explicit theme font roles rather than the oversized default',()=>{
  const region=source.slice(source.indexOf('id: nativeColumn'),source.indexOf('AdvancedDiagnostics {'));
  const texts=region.match(/PlainText \{[^{}]*(?:\{[^{}]*\}[^{}]*)*\}/g)||[];
  assert(texts.length>20);
  for(const block of texts){assert(block.includes('font.family: root.fontFamily'));assert.match(block,/font.pixelSize: Style.font\.(body|caption|title|display)/);}
});
console.log('native main panel: '+count+' passed');
