PROYECTO FINAL - RUST:

Space Invaders - Juego

struct Jugador { vida: u32, posicion: Posicion, disparos: Vec (Arreglo)}
struct Enemigo {tipo: TipoEnemigo, posicion: Posicion, activo: bool}
struct Disparo {posicion: Posicion, direccion: Direccion}
struct Juego {jugador: Jugador, enemigos: Vec(Enemigos), puntuacion: u32}

El juego debe ser sencillo:
1.- Deben las naves enemigas moverse de derecha a izquierdau viceversa, y de arriba hacia abajo.
2.- El jugador o la nave, debe igual moverse de izquierda a derecha o viceversa.
3.- Se debe llevar control de la puntuacion.
4.- debe tener vidas, al menos 3 vidas. Despues de 3 "muertes", termina el juego.

OPCIONAL:
5.- Si desean, el usuario puede recibir mas de un disparo antes de destruirse.
6.- Puede tener muros de proteccion.
