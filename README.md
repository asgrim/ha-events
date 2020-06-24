# ha-events

Home Assistant PC event webhook sender thing

 - When the app starts, it sends `ha-pc-on` webhook
 - When you ctrl+C (or send `SIGINT`), it sends `ha-pc-off` webhook
 - When screen locks, it sends `ha-pc-lock`
 - When screen unlocks, it sends `ha-pc-unlock`

The intended setup is that `ha-events` is set up as a daemon with `systemd`.

Example use cases

 - Turn on your office lights when your computer starts
 - Turn off your office lights when your computer shuts down
 - When screen lock/unlock happens, turn office lights off/on respectively

## Usage

Compile it, run it with `HA_WEBHOOK_BASE_URL` env var set, for example

```bash
HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ target/debug/ha-events
```

**NOTE** URL needs trailing slash
