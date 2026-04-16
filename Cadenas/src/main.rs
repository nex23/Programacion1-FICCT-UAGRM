use std::io::{self, Write};

const N: usize = 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}

impl Cadena {
    fn new() -> Cadena {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    fn obt_longitud(&self) -> usize {
        self.longitud
    }

    fn add_char(&mut self, c: char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    fn obt_char(&self, posicion: usize) -> char {
        //dada una posicion, devolver el caracter en esa posicion
        if posicion > 0 && posicion <= self.longitud {
            self.caracteres[posicion-1]
        } else {
            '\0'
        }
    }

    fn cant_apar_char(&self, c: char) -> u32 {
        //dado un caracter, devolver la cantidad de veces
        //que aparece en la cadena
        let mut contador = 0;
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1
            }            
        }
        contador
    }

   /*fn mas_repetido(&self) -> char {
        //Devuelve el caracter que mas veces se repite.
        //si todos aparecen 1 sola vezm, devuelve el 1ro
   } */

    /*fn conertir_mayuscula(&self) {
    }*/

    /*fn invertir(&self) {
    }*/    

    /*fn es_palindromo(&self) -> bool {
    }*/

// NUEVOS EJERCICIOS    
    
    /*fn reemplazar_pos_car(&self, pos, c) {
       //Recibe una posición y un caracter. Se debe reemaplazar el caracter en esa posición por el nuevo caracter.
    }*/

    /*fn reemplazar_car(&self, c, x) {
       //Recibe 2 caracteres. El primero es uno que contiene la cadena, que debe ser reemplazdo por el otro caracter
    }*/

    /*fn cont_voc_cons(&self) -> u8 {
       //Contar la cantidad de vocales y consonantes que contiene la cadena.
    }*/
    
    // limpia la cadena para poder ingresar una nueva
    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    // muestra la cadena completa carácter por carácter
    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }
        println!();
    }
}

// ── helpers de entrada ──────────────────────────────────────────────
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}

fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

// ── menú ────────────────────────────────────────────────────────────
fn mostrar_menu(c: &Cadena) {
    // construimos la cadena actual para mostrarla en el encabezado
    let mut preview = String::new();
    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }
    if preview.is_empty() {
        preview = String::from("(vacía)");
    }

    println!("\n╔══════════════════════════════════╗");
    println!("║   CADENA: {:>22}  ║", preview);
    println!("╠══════════════════════════════════╣");
    println!("║  1. Ingresar nueva cadena        ║");
    println!("║  2. Mostrar cadena               ║");
    println!("║  3. Longitud                     ║");
    println!("║  4. Obtener carácter (posición)  ║");
    println!("║  5. Cant Repetido carácter       ║");
    println!("║  6. Char más repetido            ║");
    println!("║  7. Convertir a mayúsculas       ║");
    println!("║  8. Invertir cadena              ║");
    println!("║  9. ¿Es palíndromo?              ║");
    println!("║  10. Reemplazar char (posición)  ║");
    println!("║  11. Reemplazar char (por otro)  ║");
    println!("║  12. Vocales y consonantes       ║");
    println!("║  13. Extraer subcadena           ║");
    println!("║  14. Eliminar carácter (posición)║");
    println!("╠══════════════════════════════════╣");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Cadenas - POO — Programación I   ");
    println!("════════════════════════════════════");

    let mut c = Cadena::new(); //definiendo la instancia de tipo Cadena

    loop {
        mostrar_menu(&c);
        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                println!("  Ingresa la cadena:");
                let entrada = leer_linea();

                c.limpiar(); // reiniciamos antes de cargar la nueva

                // ── proceso artesanal: carácter por carácter ──
                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("  ✓ Cadena cargada ({} caracteres)", c.obt_longitud());
            }

            "2" => {
                print!("  Cadena: ");
                c.mostrar();
            }

            "3" => println!("  Longitud: → {}", c.obt_longitud()),

           "4" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                        println!("  Carácter en posición {}: → '{}'", pos, c.obt_char(pos));
                    }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obt_longitud()),
                    None    => println!("  Posición inválida."),
                }
            } 

            "5" => {
                println!("  Ingresa el caracter:");
                let entrada =leer_linea();
                match entrada.chars().next() {
                    Some(carac) => {
                        let resultado = c.cant_apar_char(carac);
                        println!(" '{}' aparece {} vez/veces", carac, resultado);                        
                    }
                    None => println!("No hay caracter")
                }
            }

            "6" => {
                if c.obt_longitud() == 0 {
                    println!("  La cadena está vacía.");
                } else {
                    let resultado = c.mas_repetido();
                    println!("  El carácter que más se repite es: '{}'", resultado);
                }
            }

            "7" => {
                c.convertir_mayusculas();
                print!("  Cadena convertida: ");
                c.mostrar();
            }

            "8" => {
                c.invertir();
                print!("  Cadena invertida: ");
                c.mostrar();
            }
            
            "9" => {
                if c.es_palindromo() {
                    println!("  Sí es palíndromo.");
                } else {
                    println!("  No es palíndromo.");
                }
            }

            "10" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                    println!("  Ingresa el nuevo carácter:");
                    let entrada = leer_linea();
                    match entrada.chars().next() {
                        Some(nuevo) => {
                            c.reemplazar_en_posicion(pos, nuevo);
                            print!("  Cadena resultante: ");
                            c.mostrar();
                        }
                        None => println!("  No ingresaste ningún carácter."),
                    }
                }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obt_longitud()),
                    None    => println!("  Posición inválida."),
                }
            }

            "11" => {
                println!("  Ingresa el carácter a reemplazar:");
                let entrada1 = leer_linea();
                match entrada1.chars().next() {
                    Some(viejo) => {
                        println!("  Ingresa el carácter nuevo:");
                        let entrada2 = leer_linea();
                        match entrada2.chars().next() {
                            Some(nuevo) => {
                                c.reemplazar(viejo, nuevo);
                                print!("  Cadena resultante: ");
                                c.mostrar();
                            }
                            None => println!("  No ingresaste el carácter nuevo."),
                        }
                    }
                    None => println!("  No ingresaste ningún carácter."),
                }
            }
            
            "12" => {
                let (vocales, consonantes) = c.contar_vocales_consonantes();
                println!("  Vocales:      {}", vocales);
                println!("  Consonantes:  {}", consonantes);
            }

            "13" => {
                if c.obt_longitud() == 0 {
                    println!(" La cadena está vacía.");
                } else {
                    println!(" Ingresa la posición de inicio (1-based):");
                    if let Some(inicio) = leer_numero() {
                        println!(" Ingresa la cantidad de caracteres a extraer:");
                        if let Some(long) = leer_numero() {
                            let sub = c.subcadena(inicio, long);
                            print!(" Subcadena extraída: ");
                            sub.mostrar();
                        } else {
                            println!(" Cantidad inválida.");
                        }
                    } else {
                        println!(" Posición inválida.");
                    }
                }
            }

            "14" => {
                if c.obt_longitud() == 0 {
                    println!(" La cadena está vacía.");
                } else {
                    println!(" Ingresa la posición a eliminar (1-based):");
                    match leer_numero() {
                        Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                            c.eliminar_en_posicion(pos);
                            print!(" Cadena resultante: ");
                            c.mostrar();
                        }
                        Some(_) => println!(" Posición fuera de rango (1 a {}).", c.obt_longitud()),
                        None => println!(" Posición inválida."),
                    }
                }
            }
            
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
                        _          => println!("  Opción no válida."),
            }
        }
}
