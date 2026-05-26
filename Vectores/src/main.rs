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

    fn ceros_final(&mut self) {       
        let dimencion_inicial = self.dimension;
        self.elim_menor_e(1);

        let diferencia = dimencion_inicial - self.dimension;

        for _ in 0..diferencia {
            self.addelemento(0);
        }
    }

    fn eliminar_repe(&mut self) {
        let mut i = 0;
        while i < self.dimension {
            let num = self.elemento[i];
            let mut repetido = false;
            for j in (i+1)..self.dimension {
                if self.elemento[i] == self.elemento[j] {
                    repetido = true;
                }
            }
            if repetido {
                self.eliminar(num);
            }else{
                i += 1
            }
        }
    }

    fn elim_solo_rep(&mut self) {
        for i in 0..self.dimension {
            let num = self.elemento[i];
            for j in (i+1)..self.dimension {
                if num == self.elemento[j] {
                    for k in j..(self.dimension - 1) {
                        self.elemento[j] = self.elemento[k];
                    }
                    self.dimension -= 1
                }
            }
        }
    }
    fn burbuja_sort(& mut self) {
        for i in 0..self.dimension-2 {
            for j in i+1..self.dimension-1 {
                if self.elemento[i] > self.elemento[j] {
                    let aux = self.elemento[i];
                    self.elemento[i] = self.elemento[j];
                    self.elemento[j] = aux;
                }
            }
        }
    }

    fn seleccion_sort(& mut self) {
        let mut i = 0;
        while i < self.dimension-1 {
            let mut min = i;
            let mut j = i+1;
            while j < self.dimension {
                if self.elemento[j] < self.elemento[min] {
                    min = j;
                }
                j += 1;
            }
            if min != i {
            let aux = self.elemento[i];
            self.elemento[i] = self.elemento[min];
            self.elemento[min] = aux;
            }
            i += 1;
        }        

    }

    fn insercion_sort(& mut self) {
        for i in 1..self.dimension {
            let min = self.elemento[i];
            let mut j = i;
            while j > 0 && self.elemento[j-1] > min {
                self.elemento[j] = self.elemento[j-1];
                j -= 1;
            }
            if j != i {
                self.elemento[j] = min;
            }
        }
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
