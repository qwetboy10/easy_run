# Easy Run

## Examples of use
- `easy_run ./long_running_process`
    - Same as `nohup ./long_running_process &; disown`
- `easy_run --list`
- `easy_run --attach 0`
    - Press Ctrl+A to un-attach
- `easy_run --attach ./long_running_process`
- `easy_run --kill 0`
- `easy_run --logs 0`
- `easy_run --restart 0`
- `easy_run --name bob ./long_running_process`
- `easy_run --kill bob`
- `easy_run --kill long_running_process`
