# au3i: Among Us 3 Interpreter

## What?
Among Us 3 is an experience suitable for children of all ages. It can also be experienced by parents too. All you need is a device supported by us.

## How?
```
cargo run
```

## What again?
Among Us 3 is a "stack-based", color-based, sus-like, amogus-based interpreted "programming language", that is you have a stack, you can use command to modify the stack, and you can read and print from the terminal.

Every command comes in a pair of `color` and `action`, the **Actions** section shows how actions and colors can be mixed, with `{}` substituted with a color, and `[]` substituted with a command. Yes, `DID []?` and `[1] then [2]` are the exceptions.

### Colors
| color | function |
| --- | --- |
| RED | identity |
| BLUE | top = top + 1 |
| GREEN | top = top - 1 |
| PINK | top = top + second |
| ORANGE | top = top << second |
| YELLOW | top = top >> second |
| BLACK | top = ~top |
| WHITE | top = top & second |
| PURPLE | top = top ^ second |
| BROWN | top = random |
| CYAN | print top as ascii |
| LIME | read input from terminal |
| IMPOSTOR | debug print the stack |

### Actions
| action | outcome |
| --- | --- |
| \{\} SUS | apply \{\} |
| \{\} INNOCENT | pop top, then apply \{\} |
| \{\} VENTED | duplicate top, then apply \{\} |
| \{\} SUSSY | rotate stack left, then apply \{\} |
| VOUCH \{\} | rotate stack right, then apply \{\} |
| VOTE \{\} | swap the first two on the stack, then apply \{\} |
| DID \[\]? | repeat \[\] until top is zero |
| \[1\] THEN \[2\] | do \[1\] then do \[2\] |

## Why?
Think of the children.

## Credit
Heavily influenced by the [Among Us programming language](https://esolangs.org/wiki/Among_Us), idk, maybe even a sequel.
