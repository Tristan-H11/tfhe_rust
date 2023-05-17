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

Die Arithmetik-Befehle haben in ihrem letzten OPCode Bit eine 0.<br>
Die Arithmetik-Befehle, welche einen Wert aus dem RAM laden, haben im 5. Bit eine 1.
Abgesehen davon, unterscheiden sich die je beiden Variationen eines Befehls nicht.

| Befehl | Instruction         | Legende       | Beschreibung                                              |
|--------|---------------------|---------------|-----------------------------------------------------------|
| ADD    | `(00010)(XXXXXXXX)` | X = Konstante | Addiert die Konstante auf den Akkumulator.                |
| OR     | `(00100)(XXXXXXXX)` | X = Konstante | Ver-odert die Konstante auf den Akkumulator.              |
| AND    | `(00110)(XXXXXXXX)` | X = Konstante | Ver-undet die Konstante auf den Akkumulator.              |
| XOR    | `(01000)(XXXXXXXX)` | X = Konstante | Ver-xOdert die Konstante auf den Akkumulator.             |
| SUB    | `(01010)(XXXXXXXX)` | X = Konstante | Subtrahiert die Konstante von dem Akkumulator.            |
| MUL    | `(01100)(XXXXXXXX)` | X = Konstante | Multipliziert die Konstante mit dem Akkumulator.          |
|        |                     |               |                                                           |
| ADD_R  | `(10010)(XXXXXXXX)` | X = RAM-Adr   | Addiert den Wert von RAM-Adr X auf den Akkumulator.       |
| OR_R   | `(10100)(XXXXXXXX)` | X = RAM-Adr   | Ver-odert den Wert von RAM-Adr X auf den Akkumulator.     |
| AND_R  | `(10110)(XXXXXXXX)` | X = RAM-Adr   | Ver-undet den Wert von RAM-Adr X auf den Akkumulator.     |
| XOR_R  | `(11000)(XXXXXXXX)` | X = RAM-Adr   | Ver-xOdert den Wert von RAM-Adr X auf den Akkumulator.    |
| SUB_R  | `(11010)(XXXXXXXX)` | X = RAM-Adr   | Subtrahiert den Wert von RAM-Adr X von dem Akkumulator.   |
| MUL_R  | `(11100)(XXXXXXXX)` | X = RAM-Adr   | Multipliziert den Wert von RAM-Adr X mit dem Akkumulator. |

### Transport-Befehle

Transport-Befehle haben in ihrem letzten OPCode Bit eine 1.

| Befehl | Instruction         | Legende       | Beschreibung                                      |
|--------|---------------------|---------------|---------------------------------------------------|
| LOAD   | `(00001)(XXXXXXXX)` | X = Konstante | Lädt den Wert X in den Akkumulator.               |
| LOAD_R | `(00011)(XXXXXXXX)` | X = RAM-Adr   | Lädt den Wert von RAM-Adr X in den Akkumulator.   |
| SAVE   | `(00101)(XXXXXXXX)` | X = RAM-Adr   | Speichert den Akkumulatorwert an die RAM-Adresse. |
