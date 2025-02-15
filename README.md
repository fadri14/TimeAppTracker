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
echo "* * * * * $HOME/.cargo/bin/time_app_tracker --update" >> "$TEMP_CRON_FILE"
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
Usage: time_app_tracker [-p] [-s] [-u] [-a <add>] [-d <del>] [--main] [--apps] [--date <date>] [-n <number>]

CLI to track usage times

Options:
  -p, --pause       pause the timer
  -s, --status      get the status of timer
  -u, --update      launch update
  -a, --add         add a application
  -d, --del         delete a application
  --main            retrieve data on main time
  --apps            retrieve application data
  --date            select the date of the retrieved data, foramt : YYYY-mm-dd
  -n, --number      select the number of day of the retrieved data
  -h, --help, help  display usage information
```

### Example of interface
```
time_app_tracker --main --apps --date 2025-02-15 -n 3
```

#### Output
```
	PC time :
Sat 2025-02-15 : 3h26
Fri 2025-02-14 : 4h33
Thu 2025-02-13 : 4h03

	Stats of PC time :
Max : 4h33
Min : 3h26
Mean: 3h00

	Application time for 2025-02-15 :
alacritty : 2h52
nvim : 2h14
librewolf : 1h25
freetube : 45m
signal-desktop : 0m
discord : 0m
netflix : 0m
xournalpp : 0m
spotube : 54m
nautilus : 0m
gnome-calculator : 0m
evince : 58m
```

## Roadmap
- Add support for flatpaks
- Add the possibility to notify when an application has exceeded a certain time
- A TUI

