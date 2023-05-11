# TODO

Auf HAvard Architektur umbauen. Nur ein einziges Register. Der Akkumulator.<br>
Jeder Befehl hat nur kein oder ein Argument. Nicht zwei.<br>
Load X => Schreib X in den Accu <br>
ADD 3 => Addiert 3 auf den Accu rauf <br>
S 12 => Speichert den Wert aus dem Accu an Adresse 12

## Cycle

### Fetch

Wert ziehen und OPCODE und OPERAND speichern

### Speicherzugriff

Wenn OPCODE = Speicherbefehl, dann wird OPERAND mit dem Wert aus dem Speicher überschrieben, sonst bleibt es bei OPERAND

### Execute

ES gibt Befehle, die in den Accu speichern. (load und alle alu befehle)

# CPU-Emulator

### Simples Beispielprogramm

```
LOAD 5  ; schreibt 5 in den Akkumulator
ADD 3   ; Addiert 3 auf den Akkumulator (3+5 => 8)
SAVE 12 ; Speichert den Wert des Akkumulators (8) in RAM-Zeile 12
```

## Speicher

Der RAM wird mit 8 Adressbits angesprochen und hält jeweils (5Bit OpCode, 8Bit Operand) pro Zelle.

Damit ein unsichtbarer Zugriff auf den RAM garantiert ist, wird jede Zeile einmal gelesen und der Rückgabewert (die
gewünschte RAM-Zelle) arithmetisch bestimmt.

Es gibt nur ein Akkumulator-Register und keine weiteren Arbeitsregister.

Ein Speicher und ein Ladebefehl arbeitet nur auf dem Operanden einer Zeile. Dadurch kann man selbstmodifizierenden Code
schreiben.
Wenn in Zeile 12 ein JMP mit Operand X steht und ein `SAVE 12` aufgerufen, dann springt der JMP Befehl dorthin, wo der
Wert
vom Akku stand, als er reingeschrieben wurde.

## Instruction-Architektur

Es wird sich grundsätzlich an einer Akkumulator-Architektur orientiert.
Damit haben Befehle nur keinen oder einen Operanden.

### Arithmetik-Befehle

Die Arithmetik-Befehle haben in ihrem letzten OPCode Bit eine 0.

| Befehl | Instruction         | Legende       | Beschreibung                                  |
|--------|---------------------|---------------|-----------------------------------------------|
| ADD    | `(00010)(XXXXXXXX)` | X = Konstante | Addiert die Konstante auf den Akkumulator.    |
| OR     | `(00100)(XXXXXXXX)` | X = Konstante | Ver-odert die Konstante auf den Akkumulator.  |
| AND    | `(00110)(XXXXXXXX)` | X = Konstante | Ver-undet die Konstante auf den Akkumulator.  |
| XOR    | `(01000)(XXXXXXXX)` | X = Konstante | Ver-xOdert die Konstante auf den Akkumulator. |

### Transport-Befehle

Transport-Befehle haben in ihrem letzten OPCode Bit eine 1.

| Befehl | Instruction         | Legende       | Beschreibung                                      |
|--------|---------------------|---------------|---------------------------------------------------|
| LOAD   | `(00001)(XXXXXXXX)` | X = Konstante | Lädt den Wert X in den Akkumulator.               |
| SAVE   | `(00011)(XXXXXXXX)` | X = RAM-Adr   | Speichert den Akkumulatorwert an die RAM-Adresse. |
