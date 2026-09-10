# R5 isolated subscription probe controller adapter

This local-only checkpoint continues `R5_NATIVE_PROBE_PLAN.md`. It does not
register a daemon method, start a core, resolve endpoints or change the installed
plugin. Python `run_mihomo_probe` remains the executable migration reference;
Python cannot be removed on this evidence.

`omavless_mihomo::probe_controller` supplies only fixed authenticated Unix HTTP
requests. Its owner must retain a supervised auxiliary-child lease throughout
I/O. The lease keeps the exact child PID waitable and prevents PID reuse; this
adapter's constructor alone is not process-lifetime or spawn authorization.

## Fixed contract

- Require same-user 0700 parent and 0600 Unix socket; capture both device/inode
  identities and recheck before I/O, before writing and after reading.
- Authenticate socket peer UID and the owner-supplied exact child PID before
  sending any request bytes. No path-based or process-name authentication.
- Startup must normalize the authenticated child socket through the existing
  secure-owned socket helper before constructing this adapter. This adapter does
  not chmod arbitrary paths or create an auxiliary owner itself.
- Version liveness and exact `OMAVLESS_TEST` selector/member readiness are
  separate reads. A live `/version` alone is not configured readiness.
- Exactly the three plan-owned public URLs, fixed group name, 5000-ms timeout
  and expected 200–299 range; no arbitrary URL, endpoint, header, secret or HTTP
  method parameter. The future executor calls one group round per URL, not one
  call per alias.
- HTTP/1.0 close framing reuses the established bounded controller parser.
  Maximum total response is 64 KiB and whole request budget is at most 10 s;
  callers may shorten that budget. At most 50 ms per blocking read/write allows
  cooperative cancellation without waiting for a full network timeout.
- HTTP 504 is preserved for the existing plan collector's meaningful empty
  round semantics. Other HTTP responses are likewise classified there; failed
  transport/parsing is never reported as provider unreachability.
- Fixed English errors never include response fragments, private paths or
  controller error prose. The caller must not log the private raw response.

This intentionally strengthens Python's legacy controller trust boundary with
exact owned-child/socket proofs and explicit response/cancellation bounds.
It preserves the Python public URL loop/query and response collector semantics,
already compared through the 153 plan differential and 14 response-merge cases.

## Deterministic acceptance

Synthetic Unix listeners cover the three fixed rounds and preserved HTTP 504,
version versus exact group readiness, wrong PID receiving zero request bytes,
unsafe permissions and symlink refusal, socket replacement before/during I/O,
invalid UID/deadline admission, cancellation before connect and during a silent
response, trickle deadline exhaustion, response byte cap and private malformed
response error non-leakage. These listeners are test-owned and no external
endpoint or private fixture is used.

The parent-owned auxiliary lease, scratch staging, pinned resolver, scheduler
publication and actual installed active-VPN/urgent-disconnect checks remain
separate required integration work. No installed-core or provider success is
claimed by these deterministic adapter tests.
