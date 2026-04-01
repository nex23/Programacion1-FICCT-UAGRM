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

    fn cantidad_digitos(&self) -> u16 {
        let mut count: u16 = 0;
        let mut num: u64 = self.valor;

        while num > 0 {
            num /= 10;
            count += 1;
        }

        count
    }

    fn invertir(&self) -> u64 {
        if self.valor < 10 {
            return self.valor;
        }
        else {
            let mut num = self.valor;
            let mut invertido = 0;

            while num > 0 {
                let digito = num % 10;
                invertido = invertido * 10 + digito;
                num /= 10;
            }
            invertido
        }
    }

    fn es_capicua(&self) -> bool {
        self.valor == self.invertir()
    }

    fn elevado(&self, base: u64, exp: u16) -> u64 {
        let mut resultado: u64 = 1;
        for _ in 0..exp {
            resultado = resultado * base;
        }
        resultado
    }

    fn es_armstrong(&self) -> bool {
        let mut n = self.valor;
        let mut suma = 0;
        let expo: u16 = self.cantidad_digitos();

        while n > 0 {
            let digito = n % 10;
            suma = suma + self.elevado(digito, expo);
            n /= 10; // esto es igual a: n = n / 10;
        }
        suma == self.valor
    } 

    //funcion que devuelva la cantidad de digitos pares que contiene un numero, ej:
    //341 = 1 digito par - 379 = 0 digitos pares - 482 = 3 digitos pares.
    fn cant_dig_pares(&self) -> u64 {
        let mut num = self.valor;
        let mut cont = 0;

        while num > 0 {
            let digito = num % 10;
            if digito % 2 == 0 {
                cont += 1;
            }
            num /= 10;
        }
        cont
    }

    fn raiz_digital(&self) -> u64 {
        let mut n: u64 = self.valor;
        while n >= 10 {
            let mut suma = 0;
            let mut temp = n;
            while temp > 0 {
                suma = suma + (temp % 10);
                temp = temp / 10;
            }
            n = suma;
        }
        n
    }

    //1.- La conjetura de Collatz: Si el número es par, divídelo entre 2; si es impar, multiplícalo por 3
    //y súmale 1. Repetir hasta llegar a 1. Implementar un método que cuente los pasos necesarios y otro que
    //encuentre el valor máximo alcanzado durante la secuencia.
    
    fn collatz(&self) -> (u64, u64) {
    let mut n = self.valor;
    let mut pasos: u64 = 0;
    let mut maximo: u64 = 0;

      while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        pasos += 1;
        if n > maximo {
            maximo = n;
        }
      }
      (pasos,  maximo)
    }


    //2.- Insertar un digito en una posicion
    fn insertar_digito(&self, digito: u64, posicion: u32) -> u64 {
    let total_cifras = self.cantidad_cifras();

    let mut divisor: u64 = 1;

    for _ in 0..(total_cifras - posicion + 1) {  // ← +1 para que la posición empiece en 1
        divisor = divisor * 10;
    }

    let parte_izquierda = self.valor / divisor;
    let parte_derecha   = self.valor % divisor;

    parte_izquierda * divisor * 10 + digito * divisor + parte_derecha
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
    println!("║  5. Es capicua?                  ║");    
    println!("║  6. ¿Es Armstrong?               ║");
    println!("║  7. Cantidad Dig Par             ║");
    println!("║  8. Raiz Digital                 ║");
    println!("║  9. Collatz                      ║");
    println!("║  10. Insertar dígito             ║");
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
            "5" => println!("  Es capicua?:      → {}", n.es_capicua()),
            "6" => println!("  ¿Es Armstrong?    → {}", n.es_armstrong()),
            "7" => println!("  Cantidad de Digitos Pares es    → {}", n.cant_dig_pares()),
            "8" => println!("  La raiz gitital es    → {}", n.raiz_digital()),
            "9" => println!("  Collatz            → {}", n.collatz()),
            "10" => {
                println!("  Ingresa el dígito a insertar (0-9):");
                match leer_numero() {
                    Some(digito) if digito <= 9 => {
                        println!("  Ingresa la posición (1 = izquierda):");
                        match leer_numero() {
                            Some(posicion) => {
                                let resultado = n.insertar_digito(digito, posicion as u32);
                                println!("  Insertar dígito {} en posición {}: → {}", digito, posicion, resultado);
                            }
                            None => println!("  Posición inválida."),
                        }
                    }
                    Some(_) => println!("  El dígito debe estar entre 0 y 9."),
                    None    => println!("  Dígito inválido."),
                }
            }
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
