// SPDX-License-Identifier: MIT
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
// Deliberately no processes, filesystem, timers, Qt or network in the subject.
const subject = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname, '../plugin/NativePresentation.js'), 'utf8'), subject);
function fixture() {
  return {
    snapshot: {instanceId:'synthetic-instance', revision:7, lastKnownActual:'connected',
      desired:{connected:true, profileId:'synthetic-profile', mode:'global', generation:4},
      profiles:[{id:'synthetic-profile', name:'Synthetic name'}], subscriptions:[], lastProfileId:'synthetic-profile'},
    observation: {instanceId:'synthetic-instance', revision:7, lastKnownActual:'connected',
      availability:'observed', manualRecoveryRequired:false,
      desired:{connected:true, mode:'global', generation:4},
      facts:{ownedCoreRunning:true, desiredProfileMatchesOwned:true, ownedControllerConfigVerified:true,
        visibleMihomoCount:1, ownedAuxiliaryMihomoCount:0, visibleTunCount:1}}
  };
}
function read(f, failed=false, pending=null, unknown=false) {
  return subject.ipc(f.snapshot, f.observation, failed, pending, unknown);
}
let passed = 0;
function test(name, fn) {
  try { fn(); passed++; } catch (error) { error.message = name + ': ' + error.message; throw error; }
}
test('coherent connection is explicitly cached and not network verification', () => {
  const r = read(fixture());
  assert.equal(r.diagnostics.state, 'connected');
  assert.equal(r.diagnostics.observationAvailable, true);
  assert.equal(r.diagnostics.desiredMode, 'global');
  assert.match(r.status, /cached/);
  assert.match(r.routing, /desired mode: Full VPN; effective routes not verified/);
  assert.match(r.details, /Mihomo 1.*TUN 1/);
  for (const key of ['liveHealthVerified','routesVerified','dnsVerified','internetVerified']) assert.equal(r.diagnostics[key], false);
});
test('disconnected requires absent tunnel resources, not desired state alone', () => {
  const f=fixture();
  f.snapshot.lastKnownActual=f.observation.lastKnownActual='disconnected';
  f.snapshot.desired.connected=f.observation.desired.connected=false;
  assert.equal(read(f).diagnostics.state,'unavailable');
  Object.assign(f.observation.facts,{ownedCoreRunning:false,desiredProfileMatchesOwned:false,
    ownedControllerConfigVerified:false,visibleMihomoCount:0,visibleTunCount:0});
  assert.equal(read(f).diagnostics.state,'disconnected');
});
test('all observation fences reject stale samples', () => {
  for (const change of [o=>o.instanceId='different',o=>o.revision++,o=>o.desired.generation++,
    o=>o.desired.connected=false,o=>o.desired.mode='rule',o=>o.lastKnownActual='failed',
    o=>o.availability='unavailable',o=>o.facts=null]) {
    const f=fixture(); change(f.observation);
    const d=read(f).diagnostics;
    assert.equal(d.state,'unavailable'); assert.equal(d.observationAvailable,false);
    assert.equal(d.visibleMihomoCount,null); assert.equal(d.controllerConfigVerified,null);
  }
});
test('missing or failed metadata does not expose retained counts or desired mode', () => {
  for (const f of [{snapshot:null,observation:null},{snapshot:{},observation:null}]) {
    const d=read(f).diagnostics;
    assert.equal(d.state,'unavailable'); assert.equal(d.metadataAvailable,false);
    assert.equal(d.desiredMode,'unavailable'); assert.equal(d.desiredState,'unavailable');
  }
  const d=read(fixture(),true).diagnostics;
  assert.equal(d.metadataAvailable,false); assert.equal(d.profiles,null); assert.equal(d.observationAvailable,false);
});
test('pending and unknown outcomes suppress cached healthy claims', () => {
  let r=read(fixture(),false,{action:'synthetic'},false);
  assert.equal(r.diagnostics.state,'pending'); assert.equal(r.diagnostics.observationAvailable,false);
  assert.equal(r.diagnostics.pending,true);
  r=read(fixture(),false,{action:'synthetic'},true);
  assert.equal(r.diagnostics.state,'outcomeUnknown'); assert.equal(r.diagnostics.observationAvailable,false);
  assert.match(r.status,/outcome unknown/);
});
test('manual recovery is never hidden by a pending action or absent observation', () => {
  const f=fixture(); f.snapshot.lastKnownActual='manualRecoveryRequired'; f.observation=null;
  const r=read(f,false,{},true);
  assert.equal(r.diagnostics.state,'manualRecoveryRequired'); assert.equal(r.diagnostics.manualRecoveryRequired,true);
  assert.match(r.status,/manual recovery required/); assert.equal(r.diagnostics.observationAvailable,false);
});
test('all supported transition states and desired modes have fixed labels', () => {
  for (const state of ['starting','reconnecting','stopping','failed']) {
    const f=fixture(); f.snapshot.lastKnownActual=f.observation.lastKnownActual=state;
    assert.equal(read(f).diagnostics.state,state);
  }
  for (const mode of ['rule','global','direct']) {
    const f=fixture(); f.snapshot.desired.mode=f.observation.desired.mode=mode;
    assert.equal(read(f).diagnostics.desiredMode,mode);
    assert.match(read(f).routing,/Cached desired mode/);
  }
});
test('duplicate core or missing TUN cannot imply connected; auxiliary remains explicit', () => {
  const f=fixture(); f.observation.facts.visibleMihomoCount=2;
  assert.equal(read(f).diagnostics.state,'unavailable');
  f.observation.facts.ownedAuxiliaryMihomoCount=1;
  assert.equal(read(f).diagnostics.state,'connected');
  assert.equal(read(f).diagnostics.visibleMihomoCount,2);
  assert.equal(read(f).diagnostics.ownedAuxiliaryMihomoCount,1);
  f.observation.facts.visibleTunCount=0;
  assert.equal(read(f).diagnostics.state,'unavailable');
});
test('counter bounds and malformed facts fail closed without echo', () => {
  for (const [key,value] of [['visibleMihomoCount',65],['visibleTunCount',9],['ownedAuxiliaryMihomoCount',2],
    ['visibleTunCount',-1],['visibleTunCount',1.5],['visibleMihomoCount',Infinity],
    ['visibleMihomoCount',NaN],['visibleTunCount','SECRET_POISON'],['ownedCoreRunning','SECRET_POISON']]) {
    const f=fixture(); f.observation.facts[key]=value;
    const r=read(f); assert.equal(r.diagnostics.observationAvailable,false);
    assert.equal(r.diagnostics.state,'unavailable'); assert(!JSON.stringify(r).includes('SECRET_POISON'));
  }
});
test('maximum legal inventories are bounded and larger ones unavailable', () => {
  const f=fixture(); f.snapshot.profiles=Array(256).fill({name:'Synthetic'});
  f.snapshot.subscriptions=Array(64).fill({name:'Synthetic'});
  assert.equal(read(f).diagnostics.profiles,256); assert.equal(read(f).diagnostics.subscriptions,64);
  f.snapshot.profiles.push({}); assert.equal(read(f).diagnostics.metadataAvailable,false);
  f.snapshot.profiles=[]; f.snapshot.subscriptions.push({});
  assert.equal(read(f).diagnostics.metadataAvailable,false);
});
test('public output is an exact fixed allowlist and excludes private poison fields', () => {
  const f=fixture(), poison='SECRET_POISON';
  f.snapshot.profiles[0]={name:poison,id:poison,server:poison,uri:poison,password:poison,key:poison};
  f.snapshot.subscriptions=[{name:poison,url:poison,id:poison}];
  f.snapshot.instanceId=f.observation.instanceId=poison;
  f.snapshot.desired.profileId=f.snapshot.lastProfileId=poison;
  f.snapshot.routing={storedPreset:poison}; f.observation.rawError=poison;
  f.observation.facts.controllerPath=poison; f.observation.facts.controllerSecret=poison;
  const r=read(f,false,null,false), encoded=JSON.stringify(r);
  assert(!encoded.includes(poison)); assert(encoded.length<2048);
  assert.deepEqual(Object.keys(r.diagnostics).sort(),[
    'schemaVersion','scope','metadataAvailable','observationAvailable','state','desiredMode','desiredState',
    'pending','outcomeUnknown','manualRecoveryRequired','profiles','subscriptions','visibleMihomoCount',
    'ownedAuxiliaryMihomoCount','visibleTunCount','ownedCoreRunning','controllerConfigVerified',
    'liveHealthVerified','routesVerified','dnsVerified','internetVerified'].sort());
  assert.equal(r.diagnostics.schemaVersion,1); assert.equal(r.diagnostics.scope,'cached_native_ipc');
  for (const value of Object.values(r.diagnostics)) assert(['string','number','boolean'].includes(typeof value));
});
test('enum poison cannot become public text', () => {
  for (const field of ['mode','lastKnownActual']) {
    const f=fixture();
    if (field==='mode') f.snapshot.desired.mode='SECRET_POISON';
    else f.snapshot.lastKnownActual='SECRET_POISON';
    assert(!JSON.stringify(read(f)).includes('SECRET_POISON'));
    assert.equal(read(f).diagnostics.metadataAvailable,false);
  }
});
test('projection has no effects and returned values cannot mutate source or next read', () => {
  const f=fixture(), before=JSON.stringify(f);
  function freeze(value) { if (value && typeof value==='object') { Object.values(value).forEach(freeze); Object.freeze(value); } }
  freeze(f);
  const r=read(f); r.diagnostics.state='changed';
  assert.equal(JSON.stringify(f),before); assert.equal(read(f).diagnostics.state,'connected');
});
test('real QML handler wiring preserves legacy branches and never starts work', () => {
  const source=fs.readFileSync(path.join(__dirname,'../plugin/Panel.qml'),'utf8');
  const r=read(fixture());
  const c=vm.createContext({root:{nativeIpc:r},vless:{nativeOwner:true,
    statusText:'legacy status',routingTitle:'legacy routing',routingSummary:'summary',
    detailsText:()=> 'legacy details'}});
  for (const name of ['status','routing','details']) {
    const line=source.split('\n').find(l=>l.includes('function '+name+'(): string'));
    assert(line); vm.runInContext(line.replace('(): string','()'),c);
    assert.equal(c[name](),r[name]);
  }
  const start=source.indexOf('    function diagnostics(): string {');
  const end=source.indexOf('\n    }',start)+6;
  vm.runInContext(source.slice(start,end).replace('(): string','()'),c);
  assert.deepEqual(JSON.parse(c.diagnostics()),JSON.parse(JSON.stringify(r.diagnostics)));
  c.vless.nativeOwner=false;
  assert.equal(c.status(),'legacy status'); assert.equal(c.routing(),'legacy routing · summary');
  assert.equal(c.details(),'legacy details');
  assert.match(source,/readonly property var nativeIpc: NativePresentation\.ipc/);
});
console.log('native IPC cached reads: '+passed+' passed');
