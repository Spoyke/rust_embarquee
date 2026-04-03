# TP Rust Embarqué — Firmware ENSEA

Projet issu du TP de Logiciel embarqué et sécurisé permettant de contrôler les composants d'une carte de l'Ensea avec une Nucleo-64 L476RG. Projet réalisé en rust avec embassy. Réalisé par Simon REMY et Talha FAYYAZ.

## Structure du projet

```
tp_rust_embarquee/
├── src/
│   ├── main.rs         
│   ├── bsp.rs          
│   ├── bargraph.rs     
│   ├── gamepad.rs      
│   └── encoder.rs      
└── examples/
    └── bargraph_example.rs
	 └── gamepad_example.rs
	 └── encoder_example.rs
```

## Architecture

### Board Support Package (`bsp.rs`)

Le BSP centralise toutes les associations entre les périphériques et les pins de la carte. Il instancie la structure `Board` qui regroupe les pins par périphérique.

```rust
let board = Board::new();
```

`Board::new()` appelle `embassy_stm32::init()` une seule fois et configure toutes les pins. Cette fonction ne doit être appelée qu'une seule fois dans le programme.

## Drivers

### Bargraph (`bargraph.rs`)

Contrôle le bargraph à 8 LEDs avec une plage de valeurs configurable.

```rust
let mut bargraph = Bargraph::new(board.bargraph_pins);

bargraph.set_range(0, 100);  
bargraph.set_value(75);      // allume 6 LEDs sur 8
```

La valeur est convertie proportionnellement en nombre de LEDs allumées. Une valeur hors plage est ignorée.

### Gamepad (`gamepad.rs`)

Lecture synchrone de l'état des 5 boutons de la croix directionnelle.

```rust
let gamepad = Gamepad::new(board.gamepad_pins);

// Lire un bouton précis
if gamepad.is_pressed(Button::Top) { ... }

// Lire tous les boutons d'un coup
let state = gamepad.poll();
```

Les boutons sont câblés avec résistance de tirage interne (`Pull::Up`) : pressé = niveau bas.

### Encodeur (`encoder.rs`)

Utilise le Timer TIM2 en mode encodeur quadrature (hardware) pour une lecture sans perte d'impulsions.

```rust
let mut encoder = Encoder::new(board.encoder_pins);

// Lire la valeur courante (i16 pour gérer les valeurs négatives)
let value = encoder.read_enc_value();

// Lire le bouton intégré
if encoder.is_pressed() { ... }
```

Le compteur est un `u16` casté en `i16` à la lecture, ce qui permet de gérer les rotations dans les deux sens :

- Sens horaire → valeur croissante
- Sens anti-horaire → valeur décroissante (valeurs négatives via wrapping)

La valeur de départ et la valeur maximale du compteur sont configurables via les registres PAC :

```rust
let tim2 = embassy_stm32::pac::TIM2;
tim2.arr().write_value(65535); // valeur maximale
tim2.cnt().write_value(0);     // valeur de départ
```

## Compilation

```bash
cargo build
```

### Flashage et exécution

```bash
DEFMT_LOG='info' cargo rb tp_rust_embarquee
```

### Lancer un exemple

```bash
DEFMT_LOG='info' cargo run --example bargraph_example
DEFMT_LOG='info' cargo run --example gamepad_example
DEFMT_LOG='info' cargo run --example encoder_example
```

