# CPU-Emulator

Zur Lesbarkeit der Codeauszüge ist als Typ immer u8, statt FheUint8 angegeben und der Code ist leicht gekürzt.

## Speicher

### Struktur

Der RAM hält jeweils (5Bit OpCode, 8Bit Operand) pro Zelle.<br>
Gespeichert wird es jedoch als Tupel von zwei je 8 Bit zahlen, weil die Library keine 5 Bit Integer anbietet.
<br><br>
Der RAM kann mit bis zu 8 Adressbits angesprochen werden.
Allerdings ist die RAM-Größe gerade auf die Länge des Programms limitiert.
Wenn man also ein `x` Zeilen langes Programm ausführt, so ist der RAM `x` Zeilen lang.<br>
Jede Leseanfrage, welche über die `x`-te Zeile des RAM hinaus geht, **liefert eine 0 zurück**. <br>
Jede Schreibanfrage, welche über die `x`-te Zeile des RAM hinaus geht, wird den RAM **nicht** verändern.

| Zeile | Command | Operand | Tupel im RAM             |
|-------|---------|---------|--------------------------|
| 0     | LOAD    | 3       | `(0000_1101, 0000_0011)` |
| 1     | SUB     | 1       | `(0000_0101, 0000_0001)` |
| 2     | SAVE    | 0       | `(0000_1111, 0000_0000)` |

### Arbeitsregister

Die CPU enthält ein Arbeitsregister ("Akkumulator"), welches das Ziel jeder arithmetischen Operation ist.
Der Akkumulator kann mit Werten aus dem RAM geladen werden und sein Wert kann in den RAM gespeichert werden.

### Lese- und Schreibverhalten

Weil der RAM in Tupeln aufgebaut ist, stellt er zeitgleich das auszuführende Programm und den Datenspeicher dar.
Am Anfang eines CPU-Zyklus wird das gesamte Tupel aus dem RAM geladen und in Command und Operand aufgeteilt.
Wird allerdings ein Lesezugriff auf den RAM gestartet, so wird **nur der rechte Part** des Tupels zurückgegeben.
Wird ein Schreibzugriff auf den RAM gestartet, so wird **nur der rechte Part** des Tupels geschrieben.
Der Command bleibt in beiden Fällen unberührt.
<br><br>
In dem obigen Beispiel steht in Zeile 0 eine 3.
Mit dem Save-Befehl wird nun der Wert des Akkumulators in Zeile 0 geschrieben.
Dadurch verändert sich der dort liegende Befehl von `LOAD 3` zu `LOAD 2`.
Würde dieser Befehl in einer Schleife liegen, würde im nächsten Aufruf von Zeile 0 eine 3, statt einer 2 geladen werden.
<br>
Es ist also möglich, selbst-modifizierenden Code zu schreiben.

### Oblivious Zugriff

Damit ein unsichtbarer Zugriff auf den RAM garantiert ist, wird jede Zeile einmal gelesen und der Rückgabewert (die
gewünschte RAM-Zelle) arithmetisch bestimmt.

#### Lesen

Das Lesen des RAMs geschieht wie folgt:

```rust
let mut result: (u8, u8) = (0, 0)

for (index, tuple) in memory.enumerate() {          // Durch den gesamten RAM iterieren
let condition: u8 = (target_index == index);    // Prüfen, ob die aktuelle Zeile die Zielzeile ist
result.0 = result.0 + (& tuple.0 * & condition);  // Command schreiben schreiben
result.1 = result.1 + ( & tuple.1 * & condition);  // Operand schreiben
}
return result
```

#### Schreiben

Das Lesen des RAMs geschieht wie folgt:

```rust
for (index, tuple) in self .data.enumerate() {

let condition: u8 = (target_index == index) * is_write;         // Prüfen, ob der neue Wert geschrieben werden soll
let not_condition: u8 = ! condition;

tuple.1 = (condition * new_value) + (not_condition * tuple.1);  // Schreiben des RAM-Wertes
}
```

Wichtig zu beachten ist hierbei, dass die Condition um ein `is_write` erweitert wurde.
Damit ist sichergestellt, dass jede Zeile selbst dann mit dem gleichen Wert überschrieben wird, wenn der Command **kein
** Schreibbefehl ist.
<br> <br>
Eine Zeile `m_x` wird also in jedem CPU-Zyklus mit dem folgenden Wert überschrieben:

```
m_x = (indexEqual AND isWrite AND new_value) OR (!indexEqual OR !isWrite AND m_x)
```

## ALU

Die ALU kann folgende Befehle ausführen: ADD, OR, AND, XOR, SUB, MUL. <br>
Jeder Befehl `X` liegt als Variante mit unmittelbarer (`X`) und mit direkter (`X_R`) Adressierung vor.

### Oblivious Calculation

Damit kein Muster in den Berechnungen festgestellt werden kann, muss auch die ALU in einem CPU-Zyklus einmal jede
Berechnung ausgeführt haben.
Wie auch beim RAM-Schreiben wird jede Möglichkeit einmal angefasst und verrechnet.
<br>
Für drei Operationen `A`, `B` und `C` auf zwei Zahlen `x` und `y` ist das Ergebnis der ALU mit Command `op` das
Folgende:

```
result = (x A y) * (op==A) + (x B y) * (op==B) + (x C y) * (op==C);
```

Für eine Flag `F`, welche nur bei Operation `A` neu bestimmt werden soll, gilt Folgendes:

```
F = new_value * (op==A) + F * (op!=A)
```

## Steuerwerk (ControlUnit CU)

Das Steuerwerk hält alle Komponenten.
In der CU ist der RAM, der ProgramCounter und die ALU abgelegt.
Wird die CU erstellt, so müssen alle Opcodes, das Program, die RAM-Größe, der ProgramCounter-Initialwert
und eine verschlüsselte Null (zur Initialisierung weiterer Komponenten) übergeben werden.
<br> <br>
Die CU kann nach der Erstellung gestartet werden.
<br> <br>
Die CU bietet auch die Möglichkeit, den gesamten RAM-Inhalt in Form eines Vektors von Tupeln zurückzugeben, damit er
clientseitig wieder entschlüsselt werden kann.

### Terminierbarkeit

Weil es keine Möglichkeit gibt, einen END-Befehl zu implementieren, durchläuft die CU aktuell eine feste Anzahl von
Zyklen.
Diese Anzahl muss vor Kompilierung der Server-Seite festgelegt werden und wird bestenfalls aus dem auszuführenden
Programm berechnet. <br>
Ein Befehl dauert immer exakt einen Zyklus.

### CPU Zyklus

Die nachfolgenden Schritte werden jeden Zyklus ausgeführt:

#### Fetch

Der RAM-Inhalt und der Akkumulator wird in die CU geladen:

```rust
    let memory_cell: (u8, u8) = memory.read_from_ram(program_counter);
let opcode: u8 = memory_cell.0;
let operand: u8 = memory_cell.1;
let accu: u8 = memory.get_accu();
```

#### Decode

Die entsprechenden Conditions für `is_alu_command`, `is_load_command`, etc. ausgewertet:

```rust
    let is_alu_command: u8 = opcodes.is_alu_command(opcode);
let is_load_command: u8 = opcodes.is_load_command(opcode);
let is_write_accu: u8 = is_alu_command | is_load_command;
let is_write_ram: u8 = opcodes.is_write_to_ram(opcode);
let has_to_load_operand_from_ram: u8 = opcodes.has_to_load_operand_from_ram(opcode);
let is_jump: u8 = opcodes.is_jump_command(opcode);
```

#### Execute

Die möglichen Operationen (Berechnen, Laden, Speichern) werden unter den jeweiligen Conditions ausgeführt:

```rust
    // Schreiben des RAM-Wertes, falls es ein Schreibbefehl ist
memory.write_to_ram(
operand,
accu,
is_write_ram,
);

// Auslesen des RAM-Wertes, falls es sich um eine direkte Adressierung handelt und setzen des Datums für die weitere Aktionen
let ram_value: u8 = memory.read_from_ram(operand).1;
let calculation_data: u8 = operand * ( ! has_to_load_operand_from_ram) + ram_value * (has_to_load_operand_from_ram);

// Bestimmen des möglichen ALU-Ergebnisses
let alu_result: u8 = alu.calculate(
opcode,
calculation_data,
accu,
is_alu_command // muss übergeben werden, damit die Flags in der Alu korrekt gesetzt werden
);

// Auswerten und schreiben des (ggf. neuen) Akkumulatorwertes 
let possible_new_accu_value: u8 = alu_result * is_alu_command + calculation_data * is_load_command;
memory.write_accu(possible_new_accu_value, is_write_accu);
```

#### ProgramCounter Increment

Hier wird der ProgrammCounter gesetzt. Entweder wird er inkrementiert oder durch einen Sprungbefehl (aktuell
nur `jump if not zero`) neu gesetzt:

```rust
    let incremented_pc: u8 = program_counter + 1;
let jnz_condition: u8 = alu.zero_flag * is_jump;
self .program_counter = incremented_pc * ! is_jump + operand * jnz_condition;
```

Der ProgramCounter wird wie folgt ausgewertet:

```
PC = ((PC + 1) * !is_jump) + (operand * is_jump)
```

## Instruction-Architektur

Es wird sich grundsätzlich an einer Akkumulator-Architektur orientiert.
Damit haben Befehle nur keinen oder einen Operanden.

### Arithmetik-Befehle

Die Arithmetik-Befehle führen Berechnungen auf dem Akkumulator aus.
Jede Operation ist mit unmittelbarer und mit direkter Adressierung vorhanden.

| Befehl | Instruction         | Legende       | Beschreibung                                              |
|--------|---------------------|---------------|-----------------------------------------------------------|
| ADD    | `(00001)(XXXXXXXX)` | X = Konstante | Addiert die Konstante auf den Akkumulator.                |
| OR     | `(00010)(XXXXXXXX)` | X = Konstante | Ver-odert die Konstante auf den Akkumulator.              |
| AND    | `(00011)(XXXXXXXX)` | X = Konstante | Ver-undet die Konstante auf den Akkumulator.              |
| XOR    | `(00100)(XXXXXXXX)` | X = Konstante | Ver-xOdert die Konstante auf den Akkumulator.             |
| SUB    | `(00101)(XXXXXXXX)` | X = Konstante | Subtrahiert die Konstante von dem Akkumulator.            |
| MUL    | `(00110)(XXXXXXXX)` | X = Konstante | Multipliziert die Konstante mit dem Akkumulator.          |
|        |                     |               |                                                           |
| ADD_R  | `(00111)(XXXXXXXX)` | X = RAM-Adr   | Addiert den Wert von RAM-Adr X auf den Akkumulator.       |
| OR_R   | `(01000)(XXXXXXXX)` | X = RAM-Adr   | Ver-odert den Wert von RAM-Adr X auf den Akkumulator.     |
| AND_R  | `(01001)(XXXXXXXX)` | X = RAM-Adr   | Ver-undet den Wert von RAM-Adr X auf den Akkumulator.     |
| XOR_R  | `(01010)(XXXXXXXX)` | X = RAM-Adr   | Ver-xOdert den Wert von RAM-Adr X auf den Akkumulator.    |
| SUB_R  | `(01011)(XXXXXXXX)` | X = RAM-Adr   | Subtrahiert den Wert von RAM-Adr X von dem Akkumulator.   |
| MUL_R  | `(01100)(XXXXXXXX)` | X = RAM-Adr   | Multipliziert den Wert von RAM-Adr X mit dem Akkumulator. |

### Transport-Befehle

| Befehl | Instruction         | Legende       | Beschreibung                                      |
|--------|---------------------|---------------|---------------------------------------------------|
| LOAD   | `(01101)(XXXXXXXX)` | X = Konstante | Lädt den Wert X in den Akkumulator.               |
| LOAD_R | `(01110)(XXXXXXXX)` | X = RAM-Adr   | Lädt den Wert von RAM-Adr X in den Akkumulator.   |
| SAVE   | `(01111)(XXXXXXXX)` | X = RAM-Adr   | Speichert den Akkumulatorwert an die RAM-Adresse. |

### Programmfluss-Befehle

| Befehl | Instruction         | Legende       | Beschreibung                                              |
|--------|---------------------|---------------|-----------------------------------------------------------|
| JNZ    | `(10000)(XXXXXXXX)` | X = Konstante | Setzt den PC auf X, wenn das Zero Flag nicht gesetzt ist. |
| JMP    | `(10001)(XXXXXXXX)` | X = Konstante | Setzt den PC auf X.                                       |

## Benchmarks

### Fakultät 5 (hardcoded)

Ausführungszeiten in Millisekunden der einzelnen Schritte auf unterschiedlichen Prozessoren mit der `--release` Option.
Es werden 10 Zyklen durchlaufen und der RAM ist entsprechend auch 10 Zeilen groß.

| Schritt                                   | Apple M2 (Macbook Air) **ALT** | Ryzen 5 3600 | Ryzen 5 3600 (Ein Kern) |
|-------------------------------------------|:------------------------------:|:------------:|:-----------------------:|
| Client Ausführung                         |             639 ms             |   1'320 ms   |           --            |
| Server Ausführung                         |           265'928 ms           |  274'627 ms  |           --            |
| Verify Ausführung                         |             <1 ms              |     1 ms     |           --            |
|                                           |                                |              |                         |
| Ganzer CPU Zyklus                         |           24'600 ms            |  26'500 ms   |       138'300 ms        |
| RAM lesen                                 |            5'200 ms            |   5'800 ms   |        34'400 ms        |
| RAM schreiben                             |            6'500 ms            |   6'200 ms   |        40'000 ms        |
| Operand und Accu auslesen                 |            5'300 ms            |   5'800 ms   |        34'600 ms        |
| IsWriteAccu und IsWriteRam auswerten      |            1'300 ms            |   1'700 ms   |        5'700 ms         |
| Operand (absolut / direkt adr.) auswerten |            6'600 ms            |   7'500 ms   |        37'100 ms        |
| ALU Berechnung                            |            3'800 ms            |   4'300 ms   |        16'200 ms        |
| Akkumulator bestimmen und schreiben       |            1'100 ms            |   1'400 ms   |        4'600 ms         |
| ProgramCounter bestimmen und schreiben    |            1'100 ms            |   1'300 ms   |        46'000 ms        |

Hier ist deutlich zu sehen, dass alle Operationen, die Zugriff auf den RAM ausüben, am deutlich längsten brauchen.
Die Zeit, die ein RAM Zugriff (lesend oder schreibend) benötigt, steigt linear mit der Größe des RAM an.
Daher ist der RAM per Default auch nur so groß, wie das Programm lang ist.
<br>
Die Zeiten sind alle aus dem je schnellsten CPU-Zyklus entnommen.

## Beispielprogramm

### Fakultät 5 (hardcoded)

```rust
    (LOAD, 2),      // Lade 1 in den Akkumulator (Akk = 1)
(LOAD, 3),      // Lade 3 in den Akkumulator (Akk = 3)
(ALU_MUL_R, 0), // Multipliziere Akkumulator mit Wert an RAM Position 0 (Akk = 6)
(SAVE, 0),      // Speichere das Ergebnis in RAM Position 0 (RAM[0] = 6)
(LOAD, 4),      // Lade 4 in den Akkumulator (Akk = 4)
(ALU_MUL_R, 0), // Multipliziere Akkumulator mit Wert an RAM Position 0 (Akk = 24)
(SAVE, 0),      // Speichere das Ergebnis in RAM Position 0 (RAM[0] = 24)
(LOAD, 5),      // Lade 5 in den Akkumulator (Akk = 5)
(ALU_MUL_R, 0), // Multipliziere Akkumulator mit Wert an RAM Position 0 (Akk = 120)
(SAVE, 0),      // Speichere das Ergebnis in RAM Position 0 (RAM[0] = 120)
```

### Fakultät N (iterativ)

N und N-1 durch die entsprechenden Werte wie 3 und 2 ersetzen.

```rust
    (LOAD, N-1),      // Speicher für den Counter allocaten <-1
(LOAD, N),      // Initialwert des Ergebnisses <-6
// Multiplikation
(LOAD_R, 1),
(ALU_MUL_R, 0), // Multiplizieren
(SAVE, 1),      // Ergebnis zwischenspeichern
// Counter-Dekrement
(LOAD_R, 0),    // Counter laden
(ALU_SUB, 1),   // Counter dekrementieren
(SAVE, 0),      // Counter zwischenspeichern
// Jump
(JNZ, 2),       // Von vorn, wenn Accu != 0
```

### Testprogramm - unmittelbare Arithmetik

Es werden ADD, OR, AND, XOR, SUB und MUl mit unmittelbaren Operanden getestet.
Die ersten 6 Zellen des RAM sollten die Werte `[3,7,4,6,1,0]` aufweisen.
```rust
(LOAD, 2),
(ALU_ADD, 1),
(SAVE, 0), // Add in 0 => Erwartet: 3
(ALU_OR, 4),
(SAVE, 1), // Or in 1 => Erwartet: 7
(ALU_AND, 4),
(SAVE, 2), // And in 2 => Erwartet: 4
(ALU_XOR, 2),
(SAVE, 3), // XOR in 3 => Erwartet: 6
(ALU_SUB, 5),
(SAVE, 4), // SUB in 4 => Erwartet: 1
(ALU_MUL, 0),
(SAVE, 5) // MUL in 5 => Erwartet: 0
```

### Testprogramm - direkte Arithmetik

Es werden ADD, OR, AND, XOR, SUB und MUl mit direkt-adressierten Operanden getestet.
Die ersten 6 Zellen des RAM sollten die Werte `[4,4,0,4,3,12]` aufweisen.
```rust
(LOAD, 2),
(ALU_ADD_R, 0),
(SAVE, 0), // Add in 0 => Erwartet: 4
(ALU_OR_R, 0),
(SAVE, 1), // Or in 1 => Erwartet: 4
(ALU_AND_R, 4),
(SAVE, 2), // And in 2 => Erwartet: 0
(ALU_XOR_R, 0),
(SAVE, 3), // XOR in 3 => Erwartet: 4
(ALU_SUB_R, 4),
(SAVE, 4), // SUB in 4 => Erwartet: 3
(ALU_MUL_R, 0),
(SAVE, 5) // MUL in 5 => Erwartet: 12
```
