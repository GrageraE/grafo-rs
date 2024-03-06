use grafo_rs::grafo_rs::Arista;
use grafo_rs::grafo_rs::Grafo;

#[test]
fn test_creacion_grafo()
{
    let grafo0 = Grafo::from_aristas(&[Arista::arista(1, 2),
                                                        Arista::arista(2, 3),
                                                        Arista::VerticeAislado(5)]);

}

fn main() {}
