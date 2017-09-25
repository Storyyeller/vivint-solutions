import random
import subprocess

digits = '0123456789'

def randnum():
    base = ''.join(random.choice(digits) for _ in range(18*3))

    if not random.getrandbits(3):
        base = base[:random.randint(0, len(base)+1)]

    if not random.getrandbits(3):
        base = '0'*random.randint(0, 20)

    if not random.getrandbits(3):
        base = '9'*random.randint(0, 20)

    if not random.getrandbits(3):
        base = base[::-1]

    if not random.getrandbits(3):
        base = base[:random.randint(0, len(base)+1)]

    if not base:
        return randnum()

    return base

def run(s):
    print('running', s)
    answer = subprocess.run(["./run"], input=s, stdout=subprocess.PIPE, check=True).stdout
    print('got', answer)
    return answer

random.seed(111)

while 1:
    x = randnum()
    answer = run("{}".format(x).encode())
    assert int(answer.strip()) == int(x)

    x = randnum()
    y = randnum()
    answer = run("app app add {} {}".format(x, y).encode())
    assert int(answer.strip()) == int(x) + int(y)

    x = randnum()
    y = randnum()
    answer = run("app app app if (app app gt {} {}) 1 0".format(x, y).encode())
    assert answer.strip() == (b'1' if int(x) > int(y) else b'0')







