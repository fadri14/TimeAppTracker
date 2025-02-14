# Time App Tracker

## Description
This program allows to save the time spent on the pc and the desired applications in a database.

## Prerequisite
- cron (Verify that the service is installed and enabled)
- rust (install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

## Installation
Installation of the application.
```
cargo install time_app_tracker
```
Setting the timer.
```
bash -c 'TEMP_CRON_FILE=$(mktemp)
crontab -l > "$TEMP_CRON_FILE"
echo "* * * * * $HOME/.cargo/bin/time_app_tracker update" >> "$TEMP_CRON_FILE"
crontab "$TEMP_CRON_FILE"
rm "$TEMP_CRON_FILE"'
```

## Usage

### Help
```
time_app_tracker --help
```

#### Output of help
```
Usage: time_app_tracker [-i] [-u] [-a <add>] [-d <del>]

CLI to track usage times

Options:
  -i, --int         launch the interface
  -u, --update      launch update
  -a, --add         add a application
  -d, --del         delete a application
  --help, help      display usage information
```

### Example of interface
```
time_app_tracker --int
```

#### Output
```
	PC time:
Thu 13-02-2025 : 5h12
Wed 12-02-2025 : 16m
Tue 11-02-2025 : 3h45
Mon 10-02-2025 : 0m
Sun 09-02-2025 : 0m
Sat 08-02-2025 : 0m
Fri 07-02-2025 : 0m

	Application time for 13-02-2025 :
alacritty : 4h37
nvim : 1h29
librewolf : 2h40
freetube : 26m
signal-desktop : 0m
discord : 0m
netflix : 0m
xournalpp : 1h40
spotube : 50m
nautilus : 1m
gnome-calculator : 0m
evince : 10m
```

## Roadmap
- Add statistics
- Add feature to pause timer
- Make data visualization more modular
- Add the possibility to notify when an application has exceeded a certain time
- Add support for flatpaks
- A TUI

