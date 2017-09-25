rot1 = [3,6,9,2,5,8,1,4,7,12,15,18,11,14,17,10,13,16,21,24,27,20,23,26,19,22,25]
rot2 = [19,20,21,10,11,12,1,2,3,22,23,24,13,14,15,4,5,6,25,26,27,16,17,18,7,8,9]

seen = set()
stack = [tuple(range(27))]

while stack:
    state = stack.pop()
    if state in seen:
        continue
    seen.add(state)

    assert sorted(state) == range(27)

    for transform in (rot1, rot2):
        stack.append(tuple(state[ti-1] for ti in transform))

print 'num states', len(seen)

faces = set()
for state in seen:
    faces.add(state[0:3] + state[9:12] + state[18:21])

print 'num faces', len(faces)
print sorted(faces)


