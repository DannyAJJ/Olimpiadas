use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::cmp::{min, max};
fn main() {
    let _lee = leer();
}
struct Linea<'h> {
    nombre:&'h str,
    apellido:&'h str,
    n_1:f64,
    n_2:f64,
    n_3:f64,
    n_4:f64,
    n_5:f64,
    n_6:f64,
    n_7:f64,
    n_8:f64,
    n_9:f64,
    n_10:f64,
}
struct Puntuaciones{
    n_1: f64,
    n_2: f64,
    n_3: f64,
    n_4: f64,
    n_5: f64,
    mayorn: f64,
    menorn: f64,
    suma: f64,
    total: f64,
}
struct Limites {
    mayor: f64,
    menor: f64,
}
fn mayor<'j>(x: [f64;5])->Limites{
    let mut x1:[i32;5]= [0,0,0,0,0];
    for y in 0..5 {
        x1[y] = (x[y] * 100 as f64) as i32;
    }
    let pequeno = min(min(min(x1[0],x1[1]),min(x1[2], x1[3])),x1[4]);
    let grande = max(max(max(x1[0], x1[1]),max(x1[2], x1[3])),x1[4]);
    
    
    //println!("{}  {}", grande,pequeno);
    let pequeno = pequeno as f64 /100 as f64;
    let grande = grande as f64 /100 as f64; 
    let limites = Limites {mayor: (grande),menor:(pequeno)};
   limites
}
fn juntar<'i>(lienea1: Linea<'i>, lienea2: Linea<'i>,lienea3: Linea<'i>,puntos1: Puntuaciones,puntos2: Puntuaciones,puntos3: Puntuaciones)->String{
    let impresion= format!("{} {} {} {} {} {} {}\n{} {} {} {} {} {} {}\n{} {} {} {} {} {} {}\n\n
    ({}  - {} - {}) / 3 = {}\n
    ({}  - {} - {}) / 3 = {}\n
    ({}  - {} - {}) / 3 = {}\n\n",
     lienea1.nombre, lienea1.apellido,puntos1.n_1,puntos1.n_2,puntos1.n_3,puntos1.n_4,puntos1.n_5,
     lienea2.nombre, lienea2.apellido,puntos2.n_1,puntos2.n_2,puntos2.n_3,puntos2.n_4,puntos2.n_5,
     lienea3.nombre, lienea3.apellido,puntos3.n_1,puntos3.n_2,puntos3.n_3,puntos3.n_4,puntos3.n_5,
    puntos1.suma,puntos1.mayorn,puntos1.menorn,puntos1.total,
    puntos2.suma,puntos2.mayorn,puntos2.menorn,puntos2.total,
    puntos3.suma,puntos3.mayorn,puntos3.menorn,puntos3.total,);
    impresion
}

fn calculos<'h>(lalista: Linea)->Puntuaciones{
    let mut numero = [lalista.n_1 ,lalista.n_2 ,lalista.n_3 ,lalista.n_4 ,lalista.n_5 ,
    lalista.n_6 ,lalista.n_7,lalista.n_8 ,lalista.n_9 ,lalista.n_10 ];
    let mut y = 0;
    let mut z = 0;
    for x in 0..10 {
        if (x%2) == 0 {
            numero[x] = (((numero[x]*0.4)*100 as f64).round())/100 as f64;
        }else {
            numero[x] = (((numero[x]*0.6)*100 as f64).round())/100 as f64;
        }
        //println!("{}",numero[x]);
    }
    let mut numerof: [f64;5]= [0.0,0.0,0.0,0.0,0.0];
    let mut numeroff = 0.0;
    loop {
        if y==5 {break;}
        numerof[y] = (((numero[z]+numero[z+1])*100 as f64).round())/100 as f64;
        numeroff = numeroff + numerof[y];
        y+=1;
        z+=2;
        
    }
    /*for x in 0..5 {
        println!("{}",numerof[x]);
    }*/
    let limites: Limites = mayor(numerof);
    let totalf = ((((numeroff - limites.mayor - limites.menor) / 3 as f64)*100 as f64).round())/100 as f64;
    let puntuacion1:Puntuaciones =Puntuaciones { n_1: (numerof[0]), n_2: (numerof[1]), n_3: (numerof[2]), n_4: (numerof[3]), n_5: (numerof[4]), mayorn: (limites.mayor), menorn: (limites.menor),suma: (numeroff), total: (totalf) };
    puntuacion1
}

fn construir<'e>(valores:[&'e str;12]) ->Linea{
    let linea_lista = Linea { nombre: valores[0], apellido: valores[1], n_1: valores[2].parse::<f64>().unwrap(), n_2: valores[3].parse::<f64>().unwrap(), n_3: valores[4].parse::<f64>().unwrap(), n_4: valores[5].parse::<f64>().unwrap(), n_5: valores[6].parse::<f64>().unwrap(), n_6: valores[7].parse::<f64>().unwrap(), n_7: valores[8].parse::<f64>().unwrap(), n_8: valores[9].parse::<f64>().unwrap(), n_9: valores[10].parse::<f64>().unwrap(), n_10: valores[11].parse::<f64>().unwrap() };
    linea_lista
}
fn separar_espacios<'d>(linea: &'d str) -> [&'d str;12]{
    let mut valores=[" ";12];
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
    let primera_lista= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 0)));
    let primera_lista_= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 0)));
    let segunda_lista= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 1)));
    let segunda_lista_= construir(separar_espacios(elegir_linea(separar_lineas(cotenido_completo.as_str()), 1)));
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