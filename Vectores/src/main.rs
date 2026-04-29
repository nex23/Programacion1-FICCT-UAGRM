const MAX: usize = 512;

struct Vectores {
    dimension: usize,
    elementos: [u64; MAX],
}

impl Vectores {
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elementos: [0; MAX],
        }
    }
    
    fn dim(&self) -> usize {
        self.dimension
    }

    fn dimensionar(&mut self, d: usize) {
        self.dimension = d;
    }

    fn addelemento(&mut self, e: u64) {
        if self.dimension < MAX {
            self.elementos[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn mostrar(&self) {
        for i in 0..self.dimension {
            println!("   elementos[{}] = {}", i, self.elementos[i]);
        }
    }
}



fn main() {
    let mut v = Vectores::new();

    v.addelemento(7);
    v.addelemento(9);
    v.addelemento(3);
    v.addelemento(4);

    v.mostrar();
    println!("La langitud del vector es: {}", v.dim());
}
