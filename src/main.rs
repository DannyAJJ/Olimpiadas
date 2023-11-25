use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

    
fn main() {
    
    let _lee = leer();
    

}
struct Linea<'h> {
    nombre:&'h str,
    apellido:&'h str,
    n_1:f32,
    n_2:f32,
    n_3:f32,
    n_4:f32,
    n_5:f32,
    n_6:f32,
    n_7:f32,
    n_8:f32,
    n_9:f32,
    n_10:f32,
}
struct Puntuaciones{
    n_1: f32,
    n_2: f32,
    n_3: f32,
    n_4: f32,
    n_5: f32,
}
fn juntar<'i>(lienea1: Linea<'i>, lienea2: Linea<'i>,lienea3: Linea<'i>,puntos1: Puntuaciones,puntos2: Puntuaciones,puntos3: Puntuaciones)->String{
    let impresion= format!("{} {} {} {} {} {} {}\n{} {} {} {} {} {} {}\n{} {} {} {} {} {} {}\n", lienea1.nombre, lienea1.apellido,puntos1.n_1,puntos1.n_2,puntos1.n_3,puntos1.n_4,puntos1.n_5,lienea2.nombre, lienea2.apellido,puntos2.n_1,puntos2.n_2,puntos2.n_3,puntos2.n_4,puntos2.n_5,lienea3.nombre, lienea3.apellido,puntos3.n_1,puntos3.n_2,puntos3.n_3,puntos3.n_4,puntos3.n_5);
    impresion
}

fn calculos<'h>(lalista: Linea)->Puntuaciones{
    let lista = [(lalista.n_1 as f32 *0.4+lalista.n_2 as f32*0.6),(lalista.n_3 as f32*0.4+lalista.n_4 as f32*0.6)];
    let puntuacion1:Puntuaciones =Puntuaciones { n_1: (lista[0]), n_2: (lista[1]), n_3: (lalista.n_5 as f32*0.4)+(lalista.n_6 as f32*0.6), n_4: (lalista.n_7 as f32*0.4)+(lalista.n_8 as f32*0.6), n_5: (lalista.n_9 as f32*0.4)+(lalista.n_10 as f32*0.6) };
    puntuacion1
}

fn construir<'e>(valores:[&'e str;12]) ->Linea{
    let linea_lista = Linea { nombre: valores[0], apellido: valores[1], n_1: valores[2].parse::<f32>().unwrap(), n_2: valores[3].parse::<f32>().unwrap(), n_3: valores[4].parse::<f32>().unwrap(), n_4: valores[5].parse::<f32>().unwrap(), n_5: valores[6].parse::<f32>().unwrap(), n_6: valores[7].parse::<f32>().unwrap(), n_7: valores[8].parse::<f32>().unwrap(), n_8: valores[9].parse::<f32>().unwrap(), n_9: valores[10].parse::<f32>().unwrap(), n_10: valores[11].parse::<f32>().unwrap() };
    linea_lista
}
fn separar_espacios<'d>(linea: &'d str) -> [&'d str;12]{
    let mut valores:[&str;12]=["","","","","","","","","","","",""];
    let mut x= 0;
    let line =linea;
    for s in line.split_whitespace(){valores[x]=s;x+=1;}
    valores
}
fn separar_lineas<'c>(documento: &'c str)->[&'c str;3]{
    let mut x= 0;
    let mut lineas:[&str;3]= ["","",""];
    for s in documento.lines(){lineas[x]=s;x+=1;};
    lineas
}
fn elegir_linea<'b>(lineas: [&'b str;3], elegir: usize)->&'b str{
    let linea_seleccionada = lineas[elegir];
    linea_seleccionada
}
fn leer<'h>() -> std::io::Result<()>
    {
    let mut archivo =File::open("Calificaion.txt")?;
    let mut cotenido_completo:String =String::new();
    archivo.read_to_string(&mut cotenido_completo)?;
    let primera_lista= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 1)));
    let primera_lista_= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 1)));
    let segunda_lista= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 0)));
    let segunda_lista_= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 0)));
    let tercera_lista= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 2)));
    let tercera_lista_= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 2)));
    let impresion = juntar(primera_lista, segunda_lista, tercera_lista, calculos(primera_lista_), calculos(segunda_lista_), calculos(tercera_lista_));
    let _ = agregar(impresion);
   
    Ok(())
    }
fn agregar(impresion: String) -> std::io::Result<()> {
    let mut archivo = OpenOptions::new().append(true).create(true).open("Puntaje_Final.txt")?;
    archivo.write_all(impresion.as_bytes())?;
    Ok(())
}

pub fn mai() {

    let cadena = "Primera 1 cadena sep Segunda 2/cadena sep Tercera 3 cadena";
    
    // Ejemplo 1:
    println!("\nEJEMPLO {} <{}>:", 1, ".split(\" sep \")");
    for s in cadena.split(" sep ") { println!("{}", s); }
    
    // Ejemplo 2:
    println!("\nEJEMPLO {} <{}>:", 2, ".split('/')");
    for s in cadena.split('/') { println!("{}", s); }
    
    // Ejemplo 3:
    println!("\nEJEMPLO {} <{}>:", 3, ".split(char::is_numeric)");
    for s in cadena.split(char::is_numeric) { println!("{}", s); }

    // Ejemplo 4:
    println!("\nEJEMPLO {} <{}>:", 4, ".split_whitespace()");
    for s in cadena.split_whitespace() { println!("{}", s); }

    // Ejemplo 5:
   // println!("\nEJEMPLO {} <{}>:", 5, "Regex");
   // for s in Regex::new(r"\s").unwrap().split("Uno Dos Tres") { println!("{}", s); }

    // Ejemplo 6:
    println!("\nEJEMPLO {} <{}>:", 5, ".lines()");
    for s in "Línea 1\nLínea 2\nLínea 3".lines() { println!("{}", s); }
}