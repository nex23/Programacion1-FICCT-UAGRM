use std::io::{self, Write};  // Para leer entrada y mostrar el menú sin salto de línea
struct Numero {
    valor: u64
}

impl Numero {
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }

    fn es_primo(&self) -> bool {
        let num: u64 = self.valor;

        if num < 2 {
            return false;
        }
        if num == 2 {
           return true;
        }

        if num % 2 == 0 {
            return false;
        }

        let t: u64 = (num as f64).sqrt() as u64 + 1;
        let mut d: u64 = 3;

        while d <= t {
            if num % d == 0 {
                return false;
            }
            d += 2;
        };

        true
    }

    fn cantidad_digitos(&self) -> u32 {
        let mut count: u32 = 0;
        let mut num: u64 = self.valor;

        while num > 0 {
            num /= 10;
            count += 1;
        }

        count
    }

    fn invertir(&self) -> u64 {
        let mut num: u64 = self.valor;
        let mut invertido: u64 = 0;

        while num > 0 {
            let digito: u64 = num % 10;
            invertido = invertido * 10 + digito;
            num /= 10;
        }

        invertido
    }

    fn es_capicua(&self) -> bool {
        self.valor == self.invertir()
    }

    fn elevado(&self, base: u64, exp: u64) -> u64 {
        let mut conta = 1;
        for _ in 0..exp {
            conta *= base;
        }
        conta
    }

    fn es_armstrong(&self) -> bool {
        let mut num = self.valor;
        let digitos = self.cantidad_digitos();
        let mut suma = 0;

        while num > 0 {
            let digito = num % 10;
            suma += self.elevado(digito, digitos as u64);
            num /= 10;
        }
        suma  == self.valor
    }

    fn raiz_digital(&self) -> u16 {
        //La raiz digital, es aquel que sumando sus digitos, se obtiene un nuevo valor.
        //este valor tambien se debe sumar sus digitos, hasta que el valor sea de 1 cifra.
    }

/*     fn fibonacci(&self) -> u64 {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 0..self.valor {
            let temp: u64 = a;
            a = b;
            b = temp + b;
        }

        a
    }*/

    fn resetear(&mut self, nuevo: u64) {
        self.valor = nuevo;
    }
}
// Función para leer una línea de entrada del usuario
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
// Función para leer un número u64 del usuario, devuelve None si no es válido
fn leer_numero() -> Option<u64> {
    leer_linea().parse::<u64>().ok()
}

fn mostrar_menu(n: &Numero) {
    println!("\n╔══════════════════════════════════╗");
    println!("║   NÚMERO ACTUAL: {:>14}  ║", n.valor);
    println!("╠══════════════════════════════════╣");
    println!("║  Consulta                        ║");
    println!("║  1. ¿Es par?                     ║");
    println!("║  2. ¿Es primo?                   ║");
    println!("║  3. Cantidad de dígitos          ║");
    println!("║  4. Invertir                     ║");    
    println!("║  5. ¿Es capicúa?                 ║");
    println!("║  6. ¿Es Armstrong?               ║");
    println!("╠══════════════════════════════════╣");
    println!("║  0. Ingresar nuevo número        ║");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Números - POO — Programación I");
    println!("════════════════════════════════════");
    println!("Ingresa un número para comenzar:");
    // Validar que el usuario ingrese un número válido antes de crear la instancia de Numero
    let valor_inicial: u64 = loop { //loop se repite hasta que encuentre un break
        match leer_numero() {
            Some(num) => break num,
            None    => println!("Número inválido. Intenta de nuevo:"),
        }
    };
    //creando una instancia de Numero con el valor inicial ingresado por el usuario
    //ESTA ES LA INSTANCIA DEL OBJETO NUMERO, A PARTIR DE AQUÍ SE UTILIZARÁ PARA REALIZAR TODAS LAS CONSULTAS
    let mut n =  Numero::new(valor_inicial);

    loop { //el menu se mostrará en un bucle infinito hasta que el usuario decida salir usando la opción 'Q' (break)
        mostrar_menu(&n);
        let opcion = leer_linea();

        match opcion.as_str() {  //usando match, se puede llamar a la función correspondiente.
            // Consultas
            "1" => println!("  ¿Es par?          → {}", n.es_par()),
            "2" => println!("  ¿Es primo?        → {}", n.es_primo()),
            "3" => println!("  Cantidad Digitos: → {}", n.cantidad_digitos()),
            "4" => println!("  Invertir:         → {}", n.invertir()),
            "5" => println!("  ¿Es capicúa?      → {}", n.es_capicua()),
            "6" => println!("  ¿Es Armstrong?    → {}", n.es_armstrong()),
            "0" => {
                println!("  Ingresa el nuevo número:");
                match leer_numero() {
                    Some(num) => { n.resetear(num); println!("  ✓ Nuevo número: {}", n.valor); }
                    None    => println!("  Número inválido, se mantiene {}", n.valor),
                }
            }
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; } //aquí se rompe el ciclo con "q" o "Q"
            _ => println!("  Opción no válida."),
        }
    }
}
