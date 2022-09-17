# Simple Pomodoro timer that gets things done

Stop procrastination by dividing your work into focused intervals.

Info: https://en.wikipedia.org/wiki/Pomodoro_Technique .

# Quick Start
```bash
$ git clone https://github.com/sokratisvas/pomodoro
$ cd pomodoro
```
You can specify your own intervals by:
``` sh
$ cargo run ses 4 w 25 s 5 l 15
```

| **Total Sessions** |    **4**   |
|:------------------:|:----------:|
|  **Work Interval** | **25 min** |
|   **Short Break**  |  **5 min** |
|   **Long Break**   | **15 min** 

Pick one of the default workflows:
``` sh
$ cargo run def
```
``` sh
Workflow parameters not specified. Default workflows:
1) 4 sessions of 45 work, 8 short, 20 long
2) 4 sessions of 30 work, 5 short, 15 long
3) 5 sessions of 25 work, 5 short, 15 long
Pick a default workflow (1 - 3): 
> _
```
Or choose a built-in workflow:
``` sh
$ cargo run 
```
| **Total Sessions** |    **4**   |
|:------------------:|:----------:|
|  **Work Interval** | **45 min** |
|   **Short Break**  |  **8 min** |
|   **Long Break**   | **20 min** 
