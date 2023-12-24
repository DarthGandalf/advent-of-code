import sys
import z3

s = z3.Solver()
spx, spy, spz, svx, svy, svz = z3.Ints('spx spy spz svx svy svz')

enum = 0
for line in sys.stdin:
    enum += 1
    pos, vec = line.rstrip().split(' @ ')
    pos = [int(x) for x in pos.split(', ')]
    vec = [int(x) for x in vec.split(', ')]
    t = z3.Int(f't{enum}')
    s.add(t >= 0)
    s.add(pos[0] + t * vec[0] == spx + t * svx)
    s.add(pos[1] + t * vec[1] == spy + t * svy)
    s.add(pos[2] + t * vec[2] == spz + t * svz)

s.check()
m = s.model()
print(int(str(m[spx])) + int(str(m[spy])) + int(str(m[spz])))
