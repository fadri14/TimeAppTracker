# TimeTracker

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
crontab -e
```

Write the line below in the file.
```
* * * * * timetracker update
```

## Use
```
time_app_tracker
```

### Sample output :
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
- Add and remove applications in the following application list
- A TUI
