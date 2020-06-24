# ha-events

Home Assistant PC event webhook sender thing, for Ubuntu

 - When the app starts, it sends `ha-pc-on` webhook
 - When you ctrl+C (or send `SIGINT`), it sends `ha-pc-off` webhook
 - When screen locks, it sends `ha-pc-lock`
 - When screen unlocks, it sends `ha-pc-unlock`

Example use cases...

 - Turn on your office lights when your computer starts
 - Turn off your office lights when your computer shuts down
 - When screen lock/unlock happens, turn office lights off/on respectively

## Usage

Compile it, run it with `HA_WEBHOOK_BASE_URL` env var set, for example

```bash
HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ target/debug/ha-events
```

**NOTE** URL needs trailing slash

## Running on startup

The intended setup is that `ha-events` is set up as a "startup application":

More info: https://help.ubuntu.com/stable/ubuntu-help/startup-applications.html.en

 - **Name:** `ha-events`
 - **Description:** `Home Assistant webhook agent`
 - **Command:** `/bin/bash -c "sleep 5 && HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ /path/to/ha-events/target/debug/ha-events"`

I haven't found a way to set this up as a `systemd` service yet, since it
relies on `dbus-monitor` to watch for screen lock/unlock. Therefore it MUST be
run as your user in an environment where `dbus` is already launched; hence why
running as an Ubuntu "Startup Application".

The 5s delay is needed because it seems not everything is ready in time...
or something.

## Closing on logout/shutdown

Open your `~/.bash_logout`, and add:

```bash
killall -s SIGINT ha-events || true
```
