## TODO

### UI/UX

- [ ] Main window with PoE version selector (1 or 2) and profile switching.
	- [ ] Provide session from PoE
- [ ] Item text editor with instant re-query feature.
- [ ] Display of trade search results; allow user to manually adjust recognized item text and query options.
- [ ] Implement per-profile search history (last 10–15 items), stored as simple JSON: `user.config.json`, `user.search-history.json`.
- [ ] Switch between Standalone and Overlay modes (Overlay foundation, but implement only Standalone for the first version).
- [ ] Bindable hotkey for the full workflow (capture + OCR + search + display), including overlay/cloud support.
	- [ ] Make default hotkey for beggining
	- [ ] Provide option to configure the hotkey
- [ ] Notification area for errors: OCR failures, trade site/network errors, unrecognized screenshots, etc.
	- [ ] Simple toast notifications inside the app
	- [ ] System notifications
- [ ] Smooth window resize/move; compatibility with cloud/streaming desktops.

### Screenshot & OCR Integration

- [ ] Use macOS-native screenshot tools, save image to clipboard, forward PNG data to OCR backend.
- [ ] OCR backend as a standalone long-lived process (child, managed and monitored).
- [ ] Reliable socket or equivalent communication between main app and OCR process.
- [ ] Graceful error handling: show user-friendly errors and retry options when OCR or screenshotting fails.
- [ ] Easy mechanism to swap OCR for an ONNX-based engine in the future.

### Trade Query System

- [ ] Build robust logic for crafting and firing off market queries based on OCR outputs (with normalization for PoE item text).
- [ ] Emulate real browser requests carefully — set headers, use cookies, mimic user-agents where needed.
- [ ] Implement modular proxy/trampoline to deal with rate-limits or IP bans from trade sites.
- [ ] Strong detection and handling for site layout/API changes or server-side errors.
- [ ] Always try/catch network failures; never crash or hang, retry with sensible delays.
- [ ] Let the user edit/tweak all parts of the query before searching or for troubleshooting.
- [ ] Show or export the raw query URL for manual use if needed.

### Profiles, Settings & History

- [ ] Profile switcher in UI; separate “recent searches” per profile.
- [ ] Per-profile recent search history (auto-saved to lightweight JSON).
- [ ] Config/preferences (PoE version, saved window position, hotkey, etc.) stored and updated in local JSON.
- [ ] Backup/export of configs and search history.

### Process & Security Management

- [ ] Strict process supervision: OCR and background services must not hang, leak, or become zombies.
- [ ] Watchdog/restart for any unresponsive subprocess, with user notification.
- [ ] Socket/IPC with robust timeouts and health checks.
- [ ] All risky/background operations (OCR, HTTP requests) must be sandboxed — main UI should never freeze or crash due to errors.
- [ ] Minimal logging; do not store or expose sensitive user data anywhere.

### Cloud/Virtual/Streaming Environment Support

- [ ] Test clipboard, screenshot, and windowing in GeForceNOW and other remote/streamed desktop scenarios.
- [ ] Fallbacks for restricted system environments where screen/clipboard access or hotkeys might not work.

### Extensibility & Localization

- [ ] Architecture: English language only at launch, but enable future extension for additional locales.
- [ ] All OCR/trading logic modular to allow plugging another OCR provider (e.g., ONNX engine) without major rewrite.

---
