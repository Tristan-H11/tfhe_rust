# CPU-Emulator

Die CPU hat eine 8 Bit Daten-Architektur und akzeptiert die Maschinenbefehle in 16 Bit.
Der ProgramCounter hat folglich auch eine Breite von 8 Bit.

## Speicher

Die CPU hat 4 Register mit folgenden Adressen:

Reg A: `00` <br>
Reg B: `01` <br>
Reg C: `10` <br>
Reg D: `11` <br>

Der RAM wird mit 8 Adressbits angesprochen und hält jeweils 8 Bit pro Zelle.

Damit ein unsichtbarer Zugriff auf den RAM garantiert ist, wird jede Zeile einmal gelesen und der Rückgabewert (die
gewünschte RAM-Zelle) arithmetisch bestimmt. <br>
Damit die Register die gleiche Sicherheit aufweisen, sind die Register in den ersten vier RAM-Zellen untergebracht.
Ein Ansprechen der RAM-Adressen `00` bis `11` ist demnach identisch mit dem Ansprechen der Register A bis D.

## Instruction-Architektur

MSB `XXXX_XXXX_XX_BB_BBBB` LSB

X = Kombination von Operanden, B = Instruction

### Arithmetik-Befehle

Arithmetik-Befehle haben in ihrem 6. und 5. Bit eine `00`.

| Befehl      | Instruction            | Legende             | Beschreibung                                     |
|-------------|------------------------|---------------------|--------------------------------------------------|
| ADD Reg-Ram | `XXXX_XXXX_RR_00_0000` | X = RAM, R = Reg    | Addiert den Wert aus dem RAM auf das Register.   |
| ADD Reg-Reg | `0000_00AA_BB_00_0001` | A = Reg1, B = Reg2  | Addiert den Wert von Reg1 auf Reg2.              |
|             |                        |                     |                                                  |
| AND Reg-Ram | `XXXX_XXXX_RR_00_0010` | X = RAM, R = Reg    | Ver-undet den Wert aus dem RAM auf das Register. |
| AND Reg-Reg | `0000_00AA_BB_00_0011` | A = Reg 1, B = Reg2 | Ver-undet den Wert aus Reg1 auf Reg2.            |
|             |                        |                     |                                                  |
| OR Reg-Ram  | `XXXX_XXXX_RR_00_0100` | X = RAM, R = Reg    | Ver-odert den Wert aus dem RAM auf das Register. |
| OR Reg-Reg  | `0000_00AA_BB_00_0101` | A = Reg1, B = Reg2  | Ver-odert den Wert aus Reg1 auf Reg2.            |
|             |                        |                     |                                                  |
| XOR Reg-Ram | `XXXX_XXXX_RR_00_0110` | X = RAM, R = Reg    | Ver-xort den Wert aus dem RAM auf das Register.  |
| XOR Reg-Reg | `0000_00AA_BB_00_0111` | A = Reg1, B = Reg2  | Ver-xort den Wert aus Reg1 auf Reg2.             |

### Transport-Befehle

Transport-Befehle haben in ihrem 6. und 5. Bit eine `01`.

| Befehl         | Instruction            | Legende                | Beschreibung                                                      |
|----------------|------------------------|------------------------|-------------------------------------------------------------------|
| MOV RAM-Reg    | `XXXX_XXXX_RR_01_0000` | X = RAM, R = Reg       | Verschiebt einen Wert vom RAM in das Register.                    |
| MOVR Reg-RAM   | `XXXX_XXXX_RR_01_0001` | X = RAM, R = Reg       | Verschiebt einen Wert vom Register in den RAM.                    |
|                |                        |                        |                                                                   |
| LOAD Const-Reg | `CCCC_CCCC_RR_01_0010` | C = Konstante, R = Reg | Lädt einen Konstanten-Wert in das Register.                       |
|                |                        |                        |                                                                   |
| SWAP Reg-Reg   | `0000_00AA_BB_01_0011` | A = Reg1, B = Reg2     | Tauscht die Werte von Reg1 und Reg2.                              |
|                |                        |                        |                                                                   |
| OUT RAM        | `XXXX_XXXX_00_01_0100` | X = RAM                | Schreibt einen Wert aus den RAM in die zu serialisierenden Daten. |

### Programmfluss-Befehle

Programmfluss-Befehle haben in ihrem 6. und 5. Bit eine `10`. <br>
Die einzige Ausnahme stellt der End-Befehl dar, der `11` trägt.

| Befehl        | Instruction            | Legende       | Beschreibung                                                       |
|---------------|------------------------|---------------|--------------------------------------------------------------------|
| JMP Const-PC  | `CCCC_CCCC_00_10_0000` | C = Konstante | Springt im Programmcode, in dem PC auf die Konstante gesetzt wird. |
| JMPC Const-PC | `CCCC_CCCC_00_10_0001` | C = Konstante | Führt ein JMP aus, wenn das Carry-Flag aktiv ist.                  |
| JMPO Const-PC | `CCCC_CCCC_00_10_0010` | C = Konstante | Führt ein JMP aus, wenn das Overflow-Flag aktiv ist.               |
| JMPZ Const-PC | `CCCC_CCCC_00_10_0011` | C = Konstante | Führt ein JMP aus, wenn das Zero-Flag aktiv ist.                   |
| JMPR RAM-PC   | `XXXX_XXXX_00_10_0100` | X = RAM       | Setzt den PC auf den Wert, der an der RAM-Adresse gespeichert ist  |
|               |                        |               |                                                                    |
| END           | `0000_0000_00_11_0000` | --            | Beendet das Programm.                                              |

