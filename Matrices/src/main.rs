use std::io::{self, Write};

// equivale a:  const Nf = 120;  Nc = 120;
const NF: usize = 120;
const NC: usize = 120;

struct Matriz {
    nro_filas:    usize,
    nro_columnas: usize,
    celdas: [[u64; NC]; NF],
}



impl Matriz {

    // ── Constructor ────────────────────────────────────────────────
    fn new(nro_filas: usize, nro_columnas: usize) -> Matriz {
        Matriz {
            nro_filas,
            nro_columnas,
            celdas: [[0; NC]; NF],
        }
    }

    // ── Getters de dimensión ──────────────────────────────────────

    fn obt_nro_filas(&self) -> usize {
        self.nro_filas
    }

    fn obt_nro_columnas(&self) -> usize {
        self.nro_columnas
    }

    // ── Acceso a celdas  →  igual que Celdas[F,C] en Pascal ───────

    // leer celda
    fn get_celda(&self, f: usize, c: usize) -> u64 {
        self.celdas[f][c]          // ← self.celdas[f][c]  igual que  Celdas[F,C]
    }

    // escribir celda
    fn set_celda(&mut self, f: usize, c: usize, valor: u64) {
        self.celdas[f][c] = valor; // ← self.celdas[f][c]  igual que  Celdas[F,C]
    }

    // ── Mostrar ───────────────────────────────────────────────────

    fn mostrar(&self) {
        println!();
        for f in 0..self.nro_filas {
            // línea superior
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("┌─────────┐");
            }
            println!();
            // valores
            print!("  ");
            for c in 0..self.nro_columnas {
                print!("│{:^9}│", self.celdas[f][c]); // ← celdas[f][c]
            }
            println!();
            // línea inferior
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("└─────────┘");
            }
            println!();
        }
        // índices de columna
        print!("  ");
        for c in 0..self.nro_columnas {
            print!("{:^11}", format!("c{}", c));
        }
        println!("\n");
    }

    // ── Operaciones básicas ───────────────────────────────────────

    fn suma_total(&self) -> u64 {
        let mut suma = 0;
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                suma += self.celdas[f][c]; // ← celdas[f][c]
            }
        }
        suma
    }

    fn promedio(&self) -> u64 {
        let total = (self.nro_filas * self.nro_columnas) as u64;
        self.suma_total() / total
    }

    fn maximo(&self) -> u64 {
        let mut max = self.celdas[0][0];
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] > max { // ← celdas[f][c]
                    max = self.celdas[f][c];
                }
            }
        }
        max
    }

    fn minimo(&self) -> u64 {
        let mut min = self.celdas[0][0];
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] < min { // ← celdas[f][c]
                    min = self.celdas[f][c];
                }
            }
        }
        min
    }

    fn suma_fila(&self, f: usize) -> u64 {
        let mut suma = 0;
        for c in 0..self.nro_columnas {
            suma += self.celdas[f][c]; // ← celdas[f][c]
        }
        suma
    }

    fn suma_columna(&self, c: usize) -> u64 {
        let mut suma = 0;
        for f in 0..self.nro_filas {
            suma += self.celdas[f][c]; // ← celdas[f][c]
        }
        suma
    }

    // ── Operaciones matriciales ───────────────────────────────────

    fn sumar(&self, otra: &Matriz) -> Matriz {
        let mut resultado = Matriz::new(self.nro_filas, self.nro_columnas);
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                resultado.celdas[f][c] = self.celdas[f][c] + otra.celdas[f][c];
            }
        }
        resultado
    }

    fn restar(&self, otra: &Matriz) -> Matriz {
        let mut resultado = Matriz::new(self.nro_filas, self.nro_columnas);
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                resultado.celdas[f][c] = self.celdas[f][c] - otra.celdas[f][c];
            }
        }
        resultado
    }

    // requiere self.nro_columnas == otra.nro_filas
    fn multiplicar(&self, otra: &Matriz) -> Matriz {
        let mut resultado = Matriz::new(self.nro_filas, otra.nro_columnas);
        for f in 0..self.nro_filas {
            for c in 0..otra.nro_columnas {
                let mut suma = 0;
                for k in 0..self.nro_columnas {
                    suma += self.celdas[f][k] * otra.celdas[k][c]; // ← celdas[f][k]
                }
                resultado.celdas[f][c] = suma;
            }
        }
        resultado
    }

    fn escalar(&mut self, factor: u64) {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                self.celdas[f][c] *= factor; // ← celdas[f][c]
            }
        }
    }

    fn transponer(&self) -> Matriz {
        let mut resultado = Matriz::new(self.nro_columnas, self.nro_filas);
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                resultado.celdas[c][f] = self.celdas[f][c]; // ← celdas[f][c]
            }
        }
        resultado
    }

    // ── Diagonal (solo matrices cuadradas) ───────────────────────

    fn suma_diagonal_principal(&self) -> u64 {
        let mut suma = 0;
        for f in 0..self.nro_filas {
            suma += self.celdas[f][f]; // ← celdas[f][f]  diagonal principal
        }
        suma
    }

    fn suma_diagonal_secundaria(&self) -> u64 {
        let mut suma = 0;
        for f in 0..self.nro_filas {
            suma += self.celdas[f][self.nro_columnas - 1 - f]; // ← diagonal secundaria
        }
        suma
    }

    // ── Verificaciones ────────────────────────────────────────────

    fn es_cuadrada(&self) -> bool {
        self.nro_filas == self.nro_columnas
    }

    fn es_simetrica(&self) -> bool {
        if !self.es_cuadrada() { return false; }
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] != self.celdas[c][f] { // ← celdas[f][c]
                    return false;
                }
            }
        }
        true
    }

    fn es_identidad(&self) -> bool {
        if !self.es_cuadrada() { return false; }
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                let esperado = if f == c { 1 } else { 0 };
                if self.celdas[f][c] != esperado { // ← celdas[f][c]
                    return false;
                }
            }
        }
        true
    }

    // ── Búsqueda ──────────────────────────────────────────────────

    // retorna (fila, columna) o (usize::MAX, usize::MAX) si no existe
    fn buscar(&self, valor: u64) -> (usize, usize) {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] == valor { // ← celdas[f][c]
                    return (f, c);
                }
            }
        }
        (usize::MAX, usize::MAX)
    }

    fn contar(&self, valor: u64) -> usize {
        let mut cuenta = 0;
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] == valor { // ← celdas[f][c]
                    cuenta += 1;
                }
            }
        }
        cuenta
    }
}


// ═══════════════════════════════════════════════════════════════════
//  MAIN
// ═══════════════════════════════════════════════════════════════════

fn main() {
    println!("════════════════════════════════════════");
    println!("  Matrices - POO — Programación I      ");
    println!("════════════════════════════════════════");

    // ── crear la matriz 3x3 ──────────────────────────────────────
    let mut m = Matriz::new(3, 3);

    // ── cargar datos directamente ────────────────────────────────
    //        fila  col  valor
    m.celdas[0][0] = 1;   m.celdas[0][1] = 2;   m.celdas[0][2] = 3;
    m.celdas[1][0] = 4;   m.celdas[1][1] = 5;   m.celdas[1][2] = 6;
    m.celdas[2][0] = 7;   m.celdas[2][1] = 8;   m.celdas[2][2] = 9;

    // ── mostrar ──────────────────────────────────────────────────
    println!("Matriz original:");
    m.mostrar();
}
