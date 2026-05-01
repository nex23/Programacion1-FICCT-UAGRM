# 🚀 **Space Invaders - Proyecto Final Rust** 🚀

Un clon clásico de **Space Invaders** implementado en **Rust** con gráficos en consola usando la crate `crossterm`.

## 🎮 **Estructuras del Juego**

```rust
// 📍 Posición en la cuadrícula del juego
struct Posicion {
    x: u32,
    y: u32,
}

// 👾 Tipos de enemigos
#[derive(Clone, Copy)]
enum TipoEnemigo {
    Normal,
    Rapido,
    Fuerte,
}

// ➡️ Direcciones de movimiento
#[derive(Clone, Copy)]
enum Direccion {
    Izquierda,
    Derecha,
    Abajo,
}

// 🎯 Jugador principal
struct Jugador {
    vida: u32,           // ❤️ 3 vidas iniciales
    posicion: Posicion,
    disparos: Vec<Disparo>,
}

// 👽 Enemigo individual
struct Enemigo {
    tipo: TipoEnemigo,
    posicion: Posicion,
    activo: bool,
}

// 💥 Disparo
struct Disparo {
    posicion: Posicion,
    direccion: Direccion,
}

// 🕹️ Estado completo del juego
struct Juego {
    jugador: Jugador,
    enemigos: Vec<Enemigo>,
    puntuacion: u32,
    vidas: u32,
    game_over: bool,
}
