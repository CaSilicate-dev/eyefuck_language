# Eyefuck Programming Language
Eyefuck is a minimalist programming language based on `Brainfuck`, and make it more unreadable

## Mapping

| Brainfuck | Eyefuck |
|-----------|---------|
| `>`       | `III`   |
| `<`       | `IIl`   |
| `+`       | `IlI`   |
| `-`       | `Ill`   |
| `.`       | `lII`   |
| `,`       | `lIl`   |
| `[`       | `llI`   |
| `]`       | `lll`   |

## Examples 

Print "Hello World!"

**Brainfuck** code:
```Brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

**Eyefuck** code:
```Eyefuck
IlIIlIIlIIlIIlIIlIIlIIlIllIIIIIlIIlIIlIIlIllIIIIIlIIlIIIIIlIIlIIlIIIIIlIIlIIlIIIIIlIIIlIIlIIlIIlIlllllIIIIlIIIIIlIIIIIllIIIIIIIlIllIIIllllIIlIlllllIIIIIIlIIIIIIllIllIlllIIIlIIlIIlIIlIIlIIlIIlIlIIlIIIlIIlIIlIlIIIIIIIIlIIIIlIlllIIIIllIIIlIIlIIlIlIIIllIllIllIllIllIlllIIIllIllIllIllIllIllIllIlllIIIIIIIIIlIlIIIIIIlIIlIlII

```