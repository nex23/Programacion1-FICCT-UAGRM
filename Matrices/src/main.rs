const NF: usize = 120;
const NC: usize = 120;

struct Matriz {
    nro_filas:    usize,
    nro_columnas: usize,
    celdas: [[u64; NC]; NF],
}



impl Matriz {
    fn new(nro_filas: usize, nro_columnas: usize) -> Matriz {
        Matriz {
            nro_filas,
            nro_columnas,
            celdas: [[0; NC]; NF],
        }
    }


    fn obt_nro_filas(&self) -> usize {
        self.nro_filas
    }

    fn obt_nro_columnas(&self) -> usize {
        self.nro_columnas
    }

    // leer celda
    fn get_celda(&self, f: usize, c: usize) -> u64 {
        self.celdas[f][c]
    }

    // escribir celda
    fn set_celda(&mut self, f: usize, c: usize, valor: u64) {
        self.celdas[f][c] = valor;
    }

    // ── Mostrar ───────────────────────────────────────────────────

    fn mostrar(&self) {
        println!();
        for f in 0..self.nro_filas {
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("┌─────────┐");
            }
            println!();
            print!("  ");
            for c in 0..self.nro_columnas {
                print!("│{:^9}│", self.celdas[f][c]);
            }
            println!();
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("└─────────┘");
            }
            println!();
        }
        print!("  ");
        for c in 0..self.nro_columnas {
            print!("{:^11}", format!("c{}", c));
        }
        println!("\n");
    }

    // ── Operaciones básicas ───────────────────────────────────────

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
