// SPDX-License-Identifier: MIT
// Developer-only, synthetic executable evidence; NOT installed R6 acceptance.
// Uses already built Cargo artifacts. Never builds/installs/starts host services.
"use strict";
const fs = require("node:fs");
const path = require("node:path");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const os = require("node:os");
const SUITES = Object.freeze(["cli", "plugin_action_cli", "desktop_cli", "plugin_target", "fresh_setup_cli"]);
const BLOCKED = Object.freeze(["sudo", "pkexec", "systemctl", "systemd-run", "busctl", "dbus-send", "resolvectl", "nmcli"]);
const ROOT = path.resolve(__dirname, "..");
function requireFact(value) { if (!value) throw new Error("conformance_unconfirmed"); }
function artifacts(raw) {
    requireFact(Buffer.byteLength(raw) <= 16 * 1024 * 1024);
    const found = new Map();
    for (const line of raw.split("\n")) {
        if (!line.trim()) continue;
        const value = JSON.parse(line);
        if (value.reason !== "compiler-artifact" || !SUITES.includes(value.target?.name) || !value.executable) continue;
        requireFact(value.profile?.test === true && value.target.kind?.includes("test"));
        requireFact(value.manifest_path === path.join(ROOT, "crates/omavless-runtime/Cargo.toml"));
        requireFact(value.target.src_path === path.join(ROOT, "crates/omavless-runtime/tests", value.target.name + ".rs"));
        requireFact(typeof value.executable === "string" && path.isAbsolute(value.executable));
        requireFact(!found.has(value.target.name));
        found.set(value.target.name, value.executable);
    }
    requireFact(found.size === SUITES.length);
    return SUITES.map(name => ({ name, executable: found.get(name) }));
}
function plan(executable, binary, blockedPaths) {
    requireFact(path.isAbsolute(executable) && path.isAbsolute(binary));
    const args = ["--unshare-all", "--die-with-parent", "--new-session", "--clearenv",
        "--ro-bind", "/usr", "/usr", "--symlink", "usr/bin", "/bin",
        "--symlink", "usr/lib", "/lib", "--symlink", "usr/lib", "/lib64",
        "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp", "--tmpfs", "/run",
        "--dir", "/etc", "--dir", "/sys", "--ro-bind", ROOT, ROOT,
        "--ro-bind", process.execPath, "/tools/node",
        "--ro-bind", binary, binary, "--ro-bind", executable, executable];
    for (const item of [...new Set(blockedPaths)]) {
        requireFact(/^\/usr\/bin\/[A-Za-z0-9._+-]+$/.test(item));
        // Bind a non-executable device on each resolved executable. Absolute
        // interpreter paths fail too; this is stronger than a PATH-only stub.
        args.push("--ro-bind", "/dev/null", item);
    }
    args.push("--setenv", "OMAVLESS_CONFORMANCE_SANDBOX", "1",
        "--setenv", "OMAVLESS_CONFORMANCE_HOST_HOME", os.homedir(), "--setenv", "PATH", "/usr/bin:/bin",
        "--setenv", "HOME", "/tmp/r6-home", "--setenv", "LANG", "C.UTF-8",
        "--chdir", ROOT, "/tools/node", path.join(ROOT, "tools/native-no-python-entry.js"), executable);
    return args;
}
function safeFile(filename, max) {
    const stat = fs.lstatSync(filename);
    requireFact(stat.isFile() && !stat.isSymbolicLink() && stat.size <= max);
    requireFact(fs.realpathSync(filename) === filename);
    return fs.readFileSync(filename);
}
function main(args) {
    if (args.length !== 2 || args[0] !== "--artifacts" || !path.isAbsolute(args[1])) {
        console.log("NOT RUN: supply --artifacts ABS_CARGO_JSONL for isolated synthetic tests only.");
        return 2;
    }
    const suites = artifacts(safeFile(args[1], 16 * 1024 * 1024).toString("utf8"));
    const candidates = new Set(suites.map(s => path.resolve(path.dirname(s.executable), "../omavless")));
    requireFact(candidates.size === 1);
    const binary = [...candidates][0];
    const digest = crypto.createHash("sha256").update(safeFile(binary, 128 * 1024 * 1024)).digest("hex");
    const blocked = fs.readdirSync("/usr/bin").filter(n => /^python[0-9.]*$/.test(n) || BLOCKED.includes(n))
        .map(n => fs.realpathSync("/usr/bin/" + n));
    requireFact(blocked.some(n => /\/python[0-9.]*$/.test(n)));
    let passed = 0;
    for (const suite of suites) {
        safeFile(suite.executable, 128 * 1024 * 1024);
        const result = cp.spawnSync("/usr/bin/bwrap", plan(suite.executable, binary, blocked), {
            encoding: "utf8", timeout: 240000, maxBuffer: 1024 * 1024,
            env: { PATH: "/usr/bin:/bin", LANG: "C.UTF-8" }
        });
        let evidence;
        try { evidence = JSON.parse(result.stdout); } catch (_) { evidence = {}; }
        const ok = !result.error && result.status === 0 && Number.isInteger(evidence.passed) && evidence.passed > 0
            && evidence.ignored === 0 && evidence.pythonUnavailable === true;
        console.log(JSON.stringify({ suite: suite.name, passed: ok ? evidence.passed : false,
            pythonUnavailable: ok, scope: "synthetic_executable_conformance" }));
        if (!ok) return 1;
        passed += evidence.passed;
    }
    requireFact(crypto.createHash("sha256").update(safeFile(binary, 128 * 1024 * 1024)).digest("hex") === digest);
    console.log(JSON.stringify({ passed, suites: suites.length, binarySha256: digest,
        installedAcceptance: false, scope: "synthetic_executable_conformance" }));
    return 0;
}
module.exports = { artifacts, plan, SUITES, BLOCKED, ROOT };
if (require.main === module) {
    try { process.exitCode = main(process.argv.slice(2)); }
    catch (_) { console.log("NOT RUN: isolated conformance preconditions failed."); process.exitCode = 1; }
}
