use super::classes;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use cached::proc_macro::cached;

const SIIAU_MATERIAS_URL: &str = "http://consulta.siiau.udg.mx/wco/sspseca.consulta_oferta";
const SIIAU_INFO_URL: &str = "http://consulta.siiau.udg.mx/wco/sspseca.forma_consulta";

pub async fn get_info() -> Result<(Vec<classes::Ciclo>, Vec<classes::Centro>), Box<dyn std::error::Error>> {
    let resp = reqwest::get(SIIAU_INFO_URL).await.unwrap();
    // check if the request was successful
    if !resp.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            // error with status 
            format!("Request failed with status: {}", resp.status()),
        )));
    }

    let text = resp.text().await.unwrap();
    let document = Html::parse_document(&text);

    let selector_ciclos = Selector::parse(r#"select[name="ciclop"] > option"#).unwrap();

    let mut ciclos: Vec<classes::Ciclo> = vec![];

    for ciclo in document.select(&selector_ciclos) {
        let ciclo_text = ciclo.text().next().unwrap().replace("\n", "");
        // push to ciclos without newlines
        ciclos.push(
            classes::Ciclo {
                id: ciclo.value().attr("value").unwrap().to_string(),
                name: ciclo_text,
            }
        );
    }

    ciclos.truncate(20);

    let selector_centros = Selector::parse(r#"select[name="cup"] > option"#).unwrap();
    let mut centros: Vec<classes::Centro> = vec![];

    for centro in document.select(&selector_centros) {
        let centro_text = centro.text().next().unwrap().replace("\n", "");
        // push to centros without newlines
        centros.push(
            classes::Centro {
                id: centro_text.chars().next().unwrap().to_string(),
                name: centro_text,
            }
        );
    }

    return Ok((ciclos, centros));
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MateriasData {
    pub ciclo: String,
    pub centro: String,
    pub materias: Vec<String>
}

pub async fn get_materias (materias_data: MateriasData) -> Result<Vec<classes::Materia>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut materias : Vec<classes::Materia> = vec![];

    for materia in materias_data.materias {
        let mat = materia.to_uppercase();
        let mut params = HashMap::new();
        params.insert("ciclop", materias_data.ciclo.as_str());
        params.insert("cup", materias_data.centro.as_str());
        params.insert("crsep", mat.as_str());

        let res = client.post(SIIAU_MATERIAS_URL)
        // add ciclo, centro and materia as data
        .form(&params)
        .send()
        .await;

        // check if the request was successful and get the response text
        if res.is_err(){
            continue;
        } 


        let text = res.unwrap().text().await.unwrap();

        // parse the response text into a HTML document
        let document = Html::parse_document(&text);

        // get the table with the materias
        let selector_nrcs = Selector::parse(r#"table[border="1"] > tbody > tr"#).unwrap();
        let nrc_total = document.select(&selector_nrcs).skip(2);


        for instance in nrc_total {
            let mut materia : classes::Materia = classes::Materia::new();
            materia.activo = true;
    
            let td_selector = Selector::parse(r#"td[class="tddatos"]"#).unwrap();
            let sesiones_selector = Selector::parse(r#"td[align="center"]"#).unwrap();
            let tr_selector = Selector::parse(r#"tr"#).unwrap();
            let subtd_selector = Selector::parse(r#"td"#).unwrap();

            let mut tds = instance.select(&td_selector);
        
            let nrc = tds.next().unwrap().text().next().unwrap();
            let clave = tds.next().unwrap().text().next().unwrap();
            let nombre = tds.next().unwrap().text().next().unwrap();
            let seccion = tds.next().unwrap().text().next().unwrap();
            let creditos = tds.next().unwrap().text().next().unwrap();
            let cupos = tds.next().unwrap().text().next().unwrap();
            let disponibles = tds.next().unwrap().text().next().unwrap();


            materia.nrc = nrc.to_string();
            materia.clave = clave.to_string();
            materia.nombre = nombre.to_string();
            materia.seccion = seccion.to_string();
            materia.creditos = creditos.to_string().parse().unwrap();
            materia.cupos = cupos.to_string().parse().unwrap();
            materia.disponibles = disponibles.to_string().parse().unwrap();

            let sesiones_table = instance.select(&sesiones_selector).next().unwrap();
            let sesiones = sesiones_table.select(&tr_selector);

            let mut sesiondata : Vec<classes::Sesion> = vec![];

            for sesion in sesiones {
                let mut subtds = sesion.select(&subtd_selector);

                let _ = subtds.next().unwrap().text().next().unwrap();
                let horas = subtds.next().unwrap().text().next().unwrap();
                let dias = subtds.next().unwrap().text().next().unwrap();
                let edificio = subtds.next().unwrap().text().next().unwrap();
                let aula = subtds.next().unwrap().text().next().unwrap();
                let periodo = subtds.next().unwrap().text().next().unwrap();

                let mut diasdata: Vec<String>= vec![];

                if dias.chars().nth(0) == Some('L') {
                    diasdata.push("Lunes".to_string());
                }
                if dias.chars().nth(2) == Some('M') {
                    diasdata.push("Martes".to_string());
                }
                if dias.chars().nth(4) == Some('I') {
                    diasdata.push("Miércoles".to_string());
                }
                if dias.chars().nth(6) == Some('J') {
                    diasdata.push("Jueves".to_string());
                }
                if dias.chars().nth(8) == Some('V') {
                    diasdata.push("Viernes".to_string());
                }
                if dias.chars().nth(10) == Some('S') {
                    diasdata.push("Sábado".to_string());
                }
                // set horas to horas split with -
                let mut horas_split = horas.split("-");
                let hora0 = horas_split.next().unwrap();
                let hora1 = horas_split.next().unwrap();

                let entrada = (hora0.parse::<f32>().unwrap() / 100.0).ceil() as i32;
                let salida = (hora1.parse::<f32>().unwrap() / 100.0).ceil() as i32;
                
                sesiondata.push(classes::Sesion {
                    entrada,
                    salida,                    
                    dias: diasdata,
                    edificio: edificio.to_string(),
                    aula: aula.to_string(),
                    periodo: periodo.to_string(),
                });
            }

            materia.horas = sesiondata;

            let profesores_table = tds.next().unwrap();

            let profesores = profesores_table.select(&tr_selector);
            let mut profdata = vec![];
            for profesor in profesores {
                let mut subtds = profesor.select(&subtd_selector);

                let _ = subtds.next().unwrap().text().next().unwrap();
                let profesor = subtds.next().unwrap().text().next().unwrap();
                profdata.push(profesor.to_string());
            }

            materia.profesores = profdata;
            materias.push(materia);
        }
    }

    Ok(materias)
}

#[tauri::command]
#[cached]
pub async fn get_info_cicler() -> Option<serde_json::Value>  {
    let req = get_info().await;

    if req.is_err() { 
        return None;
    }

    let info = req.unwrap();
    let ciclos = info.0;
    let centros = info.1;

    let json = serde_json::json!({
        "ciclos": ciclos, 
        "centros": centros,
    });



    return Some(json);
}  