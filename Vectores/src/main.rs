const MAX: usize = 512;

struct Vectores {
    dimension: usize,
    elemento: [u64; MAX],
}

impl Vectores {
    //Constructor
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elemento: [0; MAX],
        }
    }

    fn dim(&self) -> usize {
        self.dimension
    }

    fn dimensionar(& mut self, d: usize) {
        self.dimension = d;
    }

    fn addelemento(& mut self, e: u64) {
        if self.dimension < MAX {
            self.elemento[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn mostrar(&self) {
        for i in 0..self.dimension {
            println!(" Elemento[{}] = {}", i, self.elemento[i]);
        }
    }

    fn insertarele(& mut self, e: u64, p: usize) {
        if p > 0 && p < self.dimension {
            self.dimension += 1;
            let mut inicial = self.dimension-1;
            while inicial >= p {
                self.elemento[inicial] = self.elemento[inicial-1];
                inicial -= 1;
            }
            self.elemento[p-1] = e;
        }
    }

    fn reemplazarele(& mut self, e: u64, p: usize) {
        if p > 0 && p <= self.dimension {
            self.elemento[p-1] = e;
        }
    }
  
     fn obtener_pos(&self, e: u64) -> usize {
        for i in 0..self.dimension {
            if self.elemento[i] == e {
                return i+1;
            }
        }
        return 0;        
    }

    fn obt_pos_real(&self, e: u64) -> usize {
        //devuelve la posicion real (sistema). Si no lo encuentra, que devuelva 600, o que se salga.
        for i in 0..self.dimension {
            if self.elemento[i] == e {
                return i
            }
        }
        return 600;
    }

    fn invertir(& mut self) {
        //invetir el vector.
        let mut izq = 0;
        let mut der = self.dimension-1;

        while izq < der {
            let temp = self.elemento[izq];
            self.elemento[izq] = self.elemento[der];
            self.elemento[der] = temp;

            izq += 1;
            der -= 1;
        }
    }

    fn eliminar(& mut self, e: u64) {
        let pos = self.obt_pos_real(e);
        for i in pos..self.dimension-1 {
            self.elemento[i] = self.elemento[i+1];
        }
        self.dimension -= 1;
    }

    fn eliminar_menor_e(& mut self, e: u64) {
        //Debe eliminar todos los elementos menores al elemento que da el usuario.
        //  3 - 4 - 7 - 0 - 2   usuario, envia el elemento 3
        //  3 - 4 - 7
        let mut i = 0;        
        while i < self.dimension {
            let mut s: bool = true;
            if self.elemento[i] < e {
                self.eliminar(self.elemento[i]);
                s = false;
            }
            if s == true {
                i += 1;
            }            
        }
    }

    fn elim_menor_e(& mut self, e: u64) {
        //Debe eliminar todos los elementos menores al elemento que da el usuario.
        //  3 - 4 - 7 - 0 - 2   usuario, envia el elemento 3
        //  3 - 4 - 7
        let mut i = 0;        
        while i < self.dimension {
            if self.elemento[i] < e {
                self.eliminar(self.elemento[i]);
            }
            else {
                i += 1;
                }
        }
    }

    fn ceros_final(& mut self) {
        //Mover todos los ceros, al final del vector
        // 3 - 7 - 0 - 2 - 0 - 1
        // 3 - 7 - 2 - 1 - 0 - 0
    }

}


fn main() {
    let mut v = Vectores::new();
    v.addelemento(5);
    v.addelemento(8);
    v.addelemento(3);
    v.addelemento(0);
    v.addelemento(9);

    v.mostrar();
    /*println!("---------------------------");
    //println!("Inserta elemento 2, en la posicion 2:");
    //v.insertarele(2, 2);
    println!("La posicion del elemento 0 es: {}", v.obtener_pos(0));
    println!("Reemplazar con el numero 9 en la posicion 4");
    v.reemplazarele(9, 4);

    v.mostrar();
    println!("---------------------------");
    println!("La posicion del elemento 9 es: {}", v.obtener_pos(9));

    println!("---------------------------");
    println!("La posicion real del elemento 9 es: {}", v.obt_pos_real(9));


    println!("---------------------------");
    println!("El Vector invertido es: ");
    v.invertir();
    v.mostrar();*/
    println!("---------------------------");
    v.elim_menor_e(4);
    //v.eliminar(3);
    //println!("La posicion del numero 3 es: {}", v.obt_pos_real(3));
    v.mostrar();
}
