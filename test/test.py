#!/usr/bin/python
import time
import os
import subprocess

subprocess.run("cargo build --release",shell=True)
subprocess.run("cargo test",shell=True)

cmd = os.path.dirname(__file__) + "/../target/release/defiro"

test_dirs = subprocess.run(f"find {os.path.dirname(__file__)} -type d -name case*",shell=True,stdout=subprocess.PIPE).stdout.decode().split("\n")

test_dirs = filter(lambda x: x != "",test_dirs)


all_ok = True
for test_dir in test_dirs:
    time.sleep(0.5)
    input_file = test_dir + "/input.txt"
    out_file = test_dir + "/out.txt"
    err_file = test_dir + "/err.txt"

    time_sta = time.perf_counter()   
    output = subprocess.run(f"{cmd} {input_file}",shell=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    output_stdout = output.stdout.decode()
    output_stderr = output.stderr.decode()

    time_end = time.perf_counter()
    
    run_time_ms = int((time_end - time_sta) * 1000)

    with open(out_file) as f:
        require_stdout = f.read()
    with open(err_file) as f:
        require_stderr = f.read()

    if require_stdout == output_stdout and require_stderr == output_stderr:
        result = "\033[32mok\033[m"
    else:
        result = "\033[31mfault\033[m"
        all_ok = False
    test_name = test_dir.split("/").pop()

    print(f"{test_name.ljust(15,' ')} {str(run_time_ms).rjust(4,' ')}ms ... {result}")


if all_ok:
    print()
    print("all ok!!")

