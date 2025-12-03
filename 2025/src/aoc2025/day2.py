GLYPH = chr(101)
gtattr = __builtins__[f'g{GLYPH}tattr']
stattr = __builtins__[f's{GLYPH}tattr']
rang = __builtins__[f'rang{GLYPH}']

import importlib
r = gtattr(importlib, f'import_modul{GLYPH}')(f'r{GLYPH}')

inv1 = gtattr(r, f'compil{GLYPH}')(r'^(\d+)\1$')
inv2 = gtattr(r, f'compil{GLYPH}')(r'^(\d+)\1+$')

class Solvationist:
    part = lambda I, invalid: sum(x for intr in I.intrvals for x in intr if invalid.match(str(x)))
    part1 = lambda I: I.part(inv1)
    part2 = lambda I: I.part(inv2)

rans = lambda a, b: rang(int(a), int(b) + 1)
stattr(Solvationist, f'pars{GLYPH}',
       (lambda I, f: stattr(I, 'intrvals',
            [rans(*string.split('-'))
             for string in gtattr(f, f'r{GLYPH}adlin{GLYPH}')().split(',')])))
