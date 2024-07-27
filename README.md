# au3i: Among Us 3 Interpreter

## Colors
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

## Actions
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
