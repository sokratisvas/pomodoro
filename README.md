# Simple Pomodoro timer that gets things done

Stop procrastination by dividing your work into focused intervals.
![](pomodemo.gif)

#Quick Start
You can specify your own intervals by:
``` sh
$ cargo run ses 4 w 25 s 5 l 15
```
| Total Sessions   | 4          |
| Work Interval    | 25 min     |
| Short Break      | 5 min      |
| Long Break       | 15 min     |

Or choose the default values:
``` sh
$ cargo run 
```
| Total Sessions   | 4          |
| Work Interval    | 45 min     |
| Short Break      | 8 min      |
| Long Break       | 20 min     |

