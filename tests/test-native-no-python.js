// SPDX-License-Identifier: MIT
// Pure plan/corpus tests: never execute bubblewrap, Python or any host command.
"use strict";
const assert = require("node:assert/strict");
const path = require("node:path");
const subject = require("../tools/run-native-no-python.js");
function record(name, extra = {}) {
    return { reason: "compiler-artifact", target: { name, kind: ["test"],
        src_path: path.join(subject.ROOT, "crates/omavless-runtime/tests", name + ".rs") }, profile: { test: true },
        manifest_path: path.join(subject.ROOT, "crates/omavless-runtime/Cargo.toml"),
        executable: "/synthetic/debug/deps/" + name, ...extra };
}
const source = subject.SUITES.map(n => record(n));
const encode = values => values.map(v => JSON.stringify(v)).join("\n");
let count = 0;
function test(name, run) { run(); count++; }
test("exact named test artifacts, independent of compiler ordering", () => {
    const rows = subject.artifacts(encode([{ reason: "build-finished", success: true }, ...source.slice().reverse()]));
    assert.deepEqual(rows.map(r => r.name), subject.SUITES);
    assert(rows.every(r => r.executable.startsWith("/synthetic/debug/deps/")));
});
test("missing duplicate malformed and oversized corpus rejected", () => {
    for (const raw of [encode(source.slice(1)), encode([...source, source[0]]), "private://invalid", "x".repeat(16 * 1024 * 1024 + 1)]) {
        assert.throws(() => subject.artifacts(raw));
    }
});
test("not-test and relative binaries rejected", () => {
    for (const replacement of [record("cli", { profile: { test: false } }),
        record("cli", { target: { name: "cli", kind: ["bin"] } }),
        record("cli", { executable: "relative" })]) {
        assert.throws(() => subject.artifacts(encode([replacement, ...source.slice(1)])));
    }
});
test("all namespace boundaries and parent death are required", () => {
    const args = subject.plan("/synthetic/debug/deps/cli", "/synthetic/debug/omavless", ["/usr/bin/python3.14"]);
    for (const word of ["--unshare-all", "--die-with-parent", "--new-session", "--clearenv"]) assert(args.includes(word));
    assert(!args.includes("--share-net") && !args.includes("--bind"));
    for (const target of ["/tmp", "/run"]) assert(args.some((v, i) => v === "--tmpfs" && args[i + 1] === target));
    assert(args.includes("/tools/node"));
});
test("matching suite names from another source or package are rejected", () => {
    for (const replacement of [record("cli", {manifest_path:"/other/Cargo.toml"}),
        record("cli", {target:{name:"cli",kind:["test"],src_path:"/other/cli.rs"}})]) {
        assert.throws(() => subject.artifacts(encode([replacement, ...source.slice(1)])));
    }
});
test("resolved executables masked, not just PATH", () => {
    const args = subject.plan("/synthetic/test", "/synthetic/omavless", ["/usr/bin/python3.14", "/usr/bin/pkexec", "/usr/bin/python3.14"]);
    assert.equal(args.filter(v => v === "/usr/bin/python3.14").length, 1);
    for (const name of ["python3.14", "pkexec"]) {
        const i = args.indexOf("/usr/bin/" + name);
        assert.deepEqual(args.slice(i - 2, i + 1), ["--ro-bind", "/dev/null", "/usr/bin/" + name]);
    }
});
test("no host home, bus or GUI environment mounted", () => {
    const args = subject.plan("/synthetic/test", "/synthetic/omavless", []);
    assert(!args.includes("/run/user") && !args.includes("/run/dbus") && !args.includes("/home/kdk/.config"));
    assert(!args.includes("DBUS_SESSION_BUS_ADDRESS") && !args.includes("WAYLAND_DISPLAY"));
    assert(args.includes("/tmp/r6-home"));
});
test("privilege entry points excluded and arbitrary mask paths refused", () => {
    for (const name of ["sudo", "pkexec", "systemctl", "systemd-run", "resolvectl"]) assert(subject.BLOCKED.includes(name));
    for (const name of ["/usr/bin/../sudo", "/private/file", "/usr/bin/evil\nname"]) {
        assert.throws(() => subject.plan("/synthetic/test", "/synthetic/omavless", [name]));
    }
});
test("installed option is fixed purpose, never an arbitrary executable", () => {
    assert.deepEqual(subject.options(["--artifacts", "/synthetic/cargo.jsonl"]),
        {artifacts:"/synthetic/cargo.jsonl", installed:false});
    assert.deepEqual(subject.options(["--artifacts", "/synthetic/cargo.jsonl", "--installed"]),
        {artifacts:"/synthetic/cargo.jsonl", installed:true});
    for (const args of [[], ["--artifacts"], ["--artifacts","relative"],
        ["--artifacts","/synthetic/cargo.jsonl","/arbitrary/program"],
        ["--artifacts","/synthetic/cargo.jsonl","--installed","extra"]]) {
        assert.equal(subject.options(args), null);
    }
});
test("installed executable overlay is read-only, namespace-only and retains isolation", () => {
    const before = subject.plan("/synthetic/debug/deps/cli", "/synthetic/debug/omavless", ["/usr/bin/python3.14"]);
    const after = subject.plan("/synthetic/debug/deps/cli", "/synthetic/debug/omavless", ["/usr/bin/python3.14"], true);
    const i = before.findIndex((v, n) => v === "--ro-bind" && before[n+1] === "/synthetic/debug/omavless");
    assert(i >= 0);
    const expected = before.slice(); expected[i+1] = "/usr/bin/omavless";
    assert.deepEqual(after, expected);
    assert.throws(() => subject.plan("/synthetic/test", "/synthetic/omavless", [], "/arbitrary/program"));
});
console.log("native no-Python isolation plan: " + count + " passed");
