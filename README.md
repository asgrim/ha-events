# ha-events

Home Assistant PC event webhook sender thing, for Ubuntu

 - When the app starts, it sends `ha-pc-on` webhook
 - When you it receives `SIGINT` (Ctrl+C is pressed) or `SIGTERM`, it sends `ha-pc-off` webhook
 - When screen locks, it sends `ha-pc-lock`
 - When screen unlocks, it sends `ha-pc-unlock`

Example use cases...

 - Turn on your office lights when your computer starts
 - Turn off your office lights when your computer shuts down
 - When screen lock/unlock happens, turn office lights off/on respectively

## Configuration

Compile it, then configure your own startup script and `systemd` service:

```bash
cp start-ha-events.sh.dist start-ha-events.sh
cp ha-events.service.dist ha-events.service
```

You will need to edit `start-ha-events.sh` to configure the environment variables,
which are documented in the file.

Open `ha-events.service` and change the directory in `ExecStart` to point to
`start-ha-events.sh` (absolute path required).

## Running as a systemd service

Use the absolute path to `ha-events.service` file to enable a `systemd` service
entry, reload and start it.

```bash
sudo systemctl enable /your/path/to/ha-events/ha-events.service
sudo systemctl daemon-reload
sudo systemctl start
```

`ha-events` will start next time you boot too.

## Only run with a specific monitor attached

If you only want to run `ha-events` when "docked" (e.g. by detecting if a specific monitor is attached), first, find
where the EDID file is, for example:

```bash
$ find /sys -name edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-HDMI-A-1/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-HDMI-A-2/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-VGA-1/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-eDP-1/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-DP-1/edid
```

I want to detect a "Dell ABC123" monitor on DP-1, so I provide the `EDID_FILE_PATH` and `EXPECTED_MONITOR_ATTACHED`
environment variables to `ha-events` too:

```bash
HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ EDID_FILE_PATH=/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-DP-1/edid EXPECTED_MONITOR_ATTACHED="DELL ABC123" /path/to/ha-events/target/debug/ha-events 
```

## Logging

Logs are sent to `/tmp/ha-events.log`. If you don't see a log, might be worth
checking `sudo systemctl status ha-events` for any suspicious output.
