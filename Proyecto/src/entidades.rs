use std::io::{stdout, Write};
use crate::Tecla;

const ANCHO: usize = 80;
const ALTO: usize = 24;

#[derive(Clone, Copy)]
struct Posicion {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Disparo {
    pos: Posicion,
    direccion: i32,
}

#[derive(Clone, Copy)]
struct Enemigo {
    pos: Posicion,
    activo: bool,
}

struct VectorDisparos {
    dimension: usize,
    elementos: [Disparo; 20],
}

impl VectorDisparos {
    fn new() -> Self {
        Self {
            dimension: 0,
            elementos: [Disparo { pos: Posicion { x: 0, y: 0 }, direccion: 0 }; 20],
        }
    }

    fn dim(&self) -> usize { self.dimension }

    fn add(&mut self, disparo: Disparo) {
        if self.dimension < 20 {
            self.elementos[self.dimension] = disparo;
            self.dimension += 1;
        }
    }

    fn actualizar(&mut self) {
        let mut i = 0;
        while i < self.dimension {
            let mut disparo = self.elementos[i];
            disparo.pos.y = (disparo.pos.y as i32 + disparo.direccion) as usize;

            if disparo.pos.y >= ALTO || disparo.pos.y == 0 {
                for j in i..self.dimension.saturating_sub(1) {
                    self.elementos[j] = self.elementos[j + 1];
                }
                self.dimension -= 1;
            } else {
                self.elementos[i] = disparo;
                i += 1;
            }
        }
    }
}

struct Jugador {
    pos: Posicion,
    vidas: u32,
}

impl Jugador {
    fn new() -> Self {
        Self {
            pos: Posicion { x: ANCHO / 2, y: ALTO - 2 },
            vidas: 3,
        }
    }

    fn mover(&mut self, tecla: crate::input::Tecla) {
        match tecla {
            crate::input::Tecla::FlechaIzquierda => if self.pos.x > 0 { self.pos.x -= 1; }
            crate::input::Tecla::FlechaDerecha   => if self.pos.x < ANCHO - 1 { self.pos.x += 1; }
            _ => {}
        }
    }
}

pub struct Juego {
    jugador: Jugador,
    disparos: VectorDisparos,
    enemigos: [Enemigo; 20],
    num_enemigos: usize,
    direccion_enemigos: i32,
    puntos: u32,
}

impl Juego {
    pub fn new() -> Self {
        let mut juego = Self {
            jugador: Jugador::new(),
            disparos: VectorDisparos::new(),
            enemigos: [Enemigo { pos: Posicion { x: 0, y: 0 }, activo: false }; 20],
            num_enemigos: 0,
            direccion_enemigos: 1,
            puntos: 0,
        };
        juego.crear_enemigos();
        juego
    }

    fn crear_enemigos(&mut self) {
        let inicio_x = 10;
        let espaciado = 4;

        for i in 0..8 {
            self.enemigos[self.num_enemigos] = Enemigo {
                pos: Posicion { x: inicio_x + i * espaciado, y: 3 },
                activo: true,
            };
            self.num_enemigos += 1;
        }

        for i in 0..8 {
            self.enemigos[self.num_enemigos] = Enemigo {
                pos: Posicion { x: inicio_x + i * espaciado, y: 6 },
                activo: true,
            };
            self.num_enemigos += 1;
        }
    }

    pub fn actualizar(&mut self) {
        print!("\x1B[2J\x1B[H");

        print!("Vidas: {} Puntos: {:04}   \r", self.jugador.vidas, self.puntos);

        // Jugador
        print!("\x1B[{};{}H▲", ALTO - 1, self.jugador.pos.x + 1);

        // Disparos
        self.disparos.actualizar();
        for i in 0..self.disparos.dim() {
            let d = self.disparos.elementos[i];
            if d.pos.y < ALTO {
                print!("\x1B[{};{}H|", d.pos.y + 1, d.pos.x + 1);
            }
        }

        // Enemigos
        self.mover_enemigos();
        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                print!("\x1B[{};{}H☻", self.enemigos[i].pos.y + 1, self.enemigos[i].pos.x + 1);
            }
        }

        let _ = stdout().flush();
    }

    pub fn dibujar(&self) {
        // Vacío — actualizar ya dibuja
    }

    fn mover_enemigos(&mut self) {
        let mut borde = false;

        for i in 0..self.num_enemigos {
            if self.enemigos[i].activo {
                let nueva_x = (self.enemigos[i].pos.x as i32 + self.direccion_enemigos) as usize;
                if nueva_x == 0 || nueva_x >= ANCHO - 2 {
                    borde = true;
                    break;
                }
            }
        }

        if borde {
            self.direccion_enemigos *= -1;
            for i in 0..self.num_enemigos {
                if self.enemigos[i].activo {
                    self.enemigos[i].pos.y += 1;
                }
            }
        } else {
            for i in 0..self.num_enemigos {
                if self.enemigos[i].activo {
                    self.enemigos[i].pos.x = (self.enemigos[i].pos.x as i32 + self.direccion_enemigos) as usize;
                }
            }
        }
    }

    pub fn procesar_tecla(&mut self, tecla: Tecla) {
        match tecla {
            Tecla::FlechaIzquierda | Tecla::FlechaDerecha => {
                self.jugador.mover(tecla);
            }
            Tecla::Espacio => {
                self.disparos.add(Disparo {
                    pos: Posicion { x: self.jugador.pos.x, y: self.jugador.pos.y - 1 },
                    direccion: -1,
                });
            }
            Tecla::Q => {
                crossterm::terminal::disable_raw_mode().ok();
                print!("\n¡Gracias por jugar!\n");
                std::process::exit(0);
            }
            _ => {}
        }
    }
}