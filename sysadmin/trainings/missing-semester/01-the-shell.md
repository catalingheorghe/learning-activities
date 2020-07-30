## The shell basics

https://missing.csail.mit.edu/2020/course-shell/

General info about the shell, commands, input output, sudo.

    ```
    # write into a root owned file
    echo 500 | sudo tee sysfsfile

    # open program for file (double-click)
    xdg-open file
    ```

Nice tips/ideas

  - sysfs file - set brightness of screen, set brightness of LEDs
  - chromecast from the CLI - send videos to Chromecast

## Exercises

	```
	mkdir /tmp/missing-semester
	cd /tmp/missing-semester/
	touch semester
	echo '#!/bin/sh' > semester 
	cat semester 
	echo 'curl --head --silent https://missing.csail.mit.edu' >> semester 
	./semester
	sh semester
	chmod +x semester 
	./semester 
	./semester | grep "last-modified" > ~/last-modified.txt
	cat ~/last-modified.txt 
	history
	history | tail -n 30 | cut -d ' ' -f 2
	history | tail -n 30 | cut -d ' ' -f 5

	xdg-open ~/swdev/github/learning-activities/sysadmin/trainings/missing-semester/01-the-shell.md 

	catalin@catalin-t540p:/sys$ cat class/power_supply/BAT0/energy_now 
	31400000
	catalin@catalin-t540p:/sys$ cat class/power_supply/BAT0/energy_full
	43930000
	catalin@catalin-t540p:/sys$ cat class/power_supply/BAT0/power_now 
	14873000
	catalin@catalin-t540p:/sys$ cat class/power_supply/BAT0/status 
	Discharging
	```

### Resources

  - [quoting](https://www.gnu.org/software/bash/manual/html_node/Quoting.html)
  - [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix))

