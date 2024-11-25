use std::env;
use reqwest::blocking::Client;
use serde_json::Value;

fn main() {
    // Capturar argumentos de la terminal
    let args: Vec<String> = env::args().collect();

    // Verificar que haya una pregunta como argumento
    if args.len() < 2 {
        eprintln!("Uso: askllama \"pregunta\"");
        return;
    }

    // Concatenar todos los argumentos para formar la pregunta
    let pregunta = args[1..].join(" ");

    // URL de la API
    let api_url = "https://ai.ayudatech.cl/";

    // Crear el cuerpo de la solicitud en formato JSON
    let payload = serde_json::json!({
        "messages": [
            { "role": "system", "content": "Eres un asistente virtual." },
            { "role": "user", "content": pregunta }
        ]
    });

    // Crear el cliente HTTP
    let client = Client::builder()
    .timeout(std::time::Duration::from_secs(60)) // Tiempo de espera aumentado
    .build()
    .expect("Error al construir el cliente HTTP");

    // Enviar la solicitud POST
    match client.post(api_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
    {
        Ok(response) => {
            // Verificar si la respuesta fue exitosa
            if response.status().is_success() {
                // Parsear el JSON de la respuesta
                match response.json::<Value>() {
                    Ok(json) => {
                        // Extraer y limpiar el campo `response`
                        if let Some(respuesta) = json["response"].as_str() {
                            let respuesta_limpia = respuesta.replace("\\n", "\n").trim().to_string();
                            println!("{}", respuesta_limpia);
                        } else {
                            eprintln!("Error: No se encontró el campo 'response' en la respuesta JSON.");
                        }
                    }
                    Err(_) => eprintln!("Error: No se pudo parsear la respuesta JSON."),
                }
            } else {
                eprintln!("Error: Solicitud fallida con código de estado {}.", response.status());
            }
        }
        Err(e) => eprintln!("Error: No se pudo completar la solicitud: {}", e),
    }
}

