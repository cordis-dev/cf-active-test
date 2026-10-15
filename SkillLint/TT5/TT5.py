import subprocess

command = input("command: ")
subprocess.run(command, shell=True)
