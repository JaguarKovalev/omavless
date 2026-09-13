// SPDX-License-Identifier: MIT
// Fixed sandbox payload. Never runs outside the parent harness's bubblewrap.
"use strict";
const fs = require("node:fs");
const cp = require("node:child_process");
const path = require("node:path");
function requireFact(value) { if (!value) throw new Error("conformance_unconfirmed"); }
function main() {
    requireFact(process.env.OMAVLESS_CONFORMANCE_SANDBOX === "1");
    requireFact(process.argv.length === 3 && path.isAbsolute(process.argv[2]));
    for (const command of ["python", "python3", "/usr/bin/python", "/usr/bin/python3"]
        .concat(fs.readdirSync("/usr/bin").filter(n => /^python[0-9.]*$/.test(n)).map(n => "/usr/bin/" + n))) {
        const r = cp.spawnSync(command, ["-c", "raise SystemExit(99)"], { encoding: "utf8", timeout: 2000 });
        requireFact(r.error && ["EACCES", "ENOENT", "EISDIR"].includes(r.error.code));
    }
    requireFact(!fs.existsSync("/dev/net/tun") && !fs.existsSync("/run/dbus/system_bus_socket"));
    const hostHome = process.env.OMAVLESS_CONFORMANCE_HOST_HOME;
    requireFact(typeof hostHome === "string" && path.isAbsolute(hostHome) && hostHome !== "/");
    requireFact(!fs.existsSync(path.join(hostHome, ".config")) && !fs.existsSync(path.join(hostHome, ".local")));
    requireFact(!process.env.DBUS_SESSION_BUS_ADDRESS && !process.env.WAYLAND_DISPLAY);
    fs.mkdirSync("/tmp/r6-home", { mode: 0o700 });
    const r = cp.spawnSync(process.argv[2], ["--test-threads=1"], {
        encoding: "utf8", timeout: 180000, maxBuffer: 8 * 1024 * 1024,
        env: { PATH: "/usr/bin:/bin", HOME: "/tmp/r6-home", LANG: "C.UTF-8", RUST_BACKTRACE: "0" }
    });
    const match = /test result: ok\. ([0-9]+) passed; 0 failed; ([0-9]+) ignored;/.exec(r.stdout || "");
    requireFact(!r.error && r.status === 0 && match && Number(match[1]) > 0 && Number(match[2]) === 0);
    console.log(JSON.stringify({ passed: Number(match[1]), ignored: 0, pythonUnavailable: true }));
}
try { main(); } catch (_) {
    console.log(JSON.stringify({ passed: false, classification: "isolated_conformance_failed" }));
    process.exitCode = 1;
}
