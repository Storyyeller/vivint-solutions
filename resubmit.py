import os
import sys
import subprocess
import time
import random

dirs = [f for f in os.listdir('.') if os.path.isdir(f)]
random.shuffle(dirs)
print(dirs)

for dir in dirs:
    with open(os.path.join(dir, 'Makefile'), 'a') as f:
        f.write('\n')

    subprocess.run('git commit -a -m "automated-resubmit"'.split(), cwd=dir)
    time.sleep(11)
    subprocess.run('git push'.split(), cwd=dir)



