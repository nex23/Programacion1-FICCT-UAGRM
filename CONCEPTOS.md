# 🦀 Programación I — Grupos SC y SH
 
> **FICCT · UAGRM** · Santa Cruz de la Sierra, Bolivia  
> **Docente:** Msc. Ing. Víctor Hugo Acosta Ortega  
> **Semestre:** I-2026
 
---
 
## 📂 Contenido de esta carpeta
 
Aquí encontrarás todos los proyectos Rust desarrollados en clase con el **Grupos SC y SH**, organizados por bloque temático.
 
```
📁 SC/
├── 01_numeros/
├── 02_cadenas/
├── 03_vectores/
└── 04_matrices/
```
 
---
 
## 🎓 Material de estudio
 
Accede a las presentaciones interactivas preparadas por el docente:
 
| Recurso | Descripción | Enlace |
|---|---|---|
| 🦀 **Introducción a Rust** | Historia, características, ownership, por qué aprender Rust | [hugonex.com/rust](https://hugonex.com/rust) |
| 🧩 **POO en Rust** | `struct`, `impl`, `pub`, `trait` y composición con ejemplos | [hugonex.com/poorust](https://hugonex.com/poorust) |
 
---
 
## 🧠 Conceptos clave de Rust
 
### 📌 Variables y mutabilidad
 
Por defecto, las variables en Rust son **inmutables**. Para modificarlas se usa `mut`:
 
```rust
let x = 5;        // inmutable
let mut y = 10;   // mutable
y = 20;           // ✅ válido
```
 
### 📌 Tipos de datos básicos
 
```rust
let entero: i32 = 42;
let decimal: f64 = 3.14;
let texto: String = String::from("Hola Rust");
let booleano: bool = true;
```
 
### 📌 Inferencia de tipos
 
Rust puede deducir el tipo automáticamente sin que lo declares:
 
```rust
let edad = 20;        // Rust infiere i32
let promedio = 85.5;  // Rust infiere f64
```
 
### 📌 Ownership (Propiedad)
 
Cada valor tiene **un único dueño**. Cuando el dueño sale del alcance, el valor se libera automáticamente — sin recolector de basura:
 
```rust
let nombre = String::from("Ana");
let otro = nombre;  // la propiedad se transfiere
// println!("{}", nombre); ❌ ya no es válido
```
 
### 📌 Borrowing (Préstamo)
 
Se puede **prestar** una referencia sin transferir la propiedad:
 
```rust
fn saludar(s: &String) {
    println!("Hola, {}!", s);
}
 
let nombre = String::from("Luis");
saludar(&nombre);          // préstamo
println!("{}", nombre);    // ✅ sigue siendo mío
```
 
### 📌 Funciones
 
```rust
fn suma(a: i32, b: i32) -> i32 {
    a + b  // sin punto y coma = valor de retorno
}
```
 
### 📌 Control de flujo
 
```rust
// if / else
if nota >= 51 {
    println!("Aprobado");
} else {
    println!("Reprobado");
}
 
// loop con for
for i in 0..5 {
    println!("Iteración {}", i);
}
 
// while
let mut i = 0;
while i < 5 {
    i += 1;
}
```
 
### 📌 Vectores
 
```rust
let mut notas: Vec<f64> = Vec::new();
notas.push(85.0);
notas.push(90.5);
 
for nota in &notas {
    println!("{}", nota);
}
```
 
---
 
## 🧩 POO en Rust — Resumen rápido
 
### `struct` — La unidad de datos
Define los atributos de un objeto (equivale a la clase en Java):
```rust
struct Estudiante {
    nombre: String,
    carnet: u32,
    nota: f64,
}
```
 
### `impl` — El comportamiento
Agrega métodos al struct (constructor, getters, acciones):
```rust
impl Estudiante {
    fn new(nombre: String, carnet: u32, nota: f64) -> Estudiante {
        Estudiante { nombre, carnet, nota }
    }
 
    fn mostrar(&self) {
        println!("{} - Nota: {}", self.nombre, self.nota);
    }
}
```
 
### `pub` — Encapsulamiento
Rust es **privado por defecto**. `pub` expone lo que se quiere compartir:
```rust
pub struct Estudiante {
    pub nombre: String,
    nota: f64,  // privado: solo accesible mediante métodos
}
```
 
### `trait` — Polimorfismo
Define un contrato de comportamiento que varios tipos pueden implementar:
```rust
trait Describible {
    fn describir(&self);
}
 
impl Describible for Estudiante {
    fn describir(&self) {
        println!("Estudiante: {}", self.nombre);
    }
}
```
 
### Composición — Objetos dentro de objetos
Rust favorece **composición** sobre herencia:
```rust
struct Direccion {
    ciudad: String,
    pais: String,
}
 
struct Estudiante {
    nombre: String,
    direccion: Direccion,  // composición
}
```
 
---
 
## 🛠️ Cómo ejecutar los proyectos
 
```bash
# Entrar a la carpeta del proyecto
cd 01_numeros/nombre_proyecto
 
# Compilar y ejecutar
cargo run
 
# Solo compilar
cargo build
```
 
---
 
> 📖 Para profundizar en los temas, revisá las presentaciones del docente:  
> 🔗 [hugonex.com/rust](https://hugonex.com/rust) · [hugonex.com/poorust](https://hugonex.com/poorust)
 
---
 
![Rust](https://img.shields.io/badge/Lenguaje-Rust-orange?style=flat-square&logo=rust)
![Grupo](https://img.shields.io/badge/Grupos-SC-SH-blue?style=flat-square)
![FICCT](https://img.shields.io/badge/Facultad-FICCT-green?style=flat-square)
 
