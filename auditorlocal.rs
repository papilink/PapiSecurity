
    // Papiweb desarrollos informaticos
use std::env;
use std::collections::HashMap;

fn main() {
    // Lista de palabras clave sospechosas de ser secretos
    let keywords = vec!["SECRET", "PASSWORD", "KEY", "TOKEN", "AUTH", "PWD"];
    let mut findings = HashMap::new();

    println!("--- AUDITORÍA DE SEGURIDAD LOCAL (Demo) ---");
    println!("Buscando posibles credenciales expuestas en entorno...\n");

    // Iteramos sobre las variables de entorno del sistema
    for (key, value) in env::vars() {
        for kw in &keywords {
            if key.to_uppercase().contains(kw) {
                findings.insert(key.clone(), value.clone());
            }
        }
    }

    if findings.is_empty() {
        println!("✅ No se detectaron variables críticas expuestas.");
    } else {
        println!("⚠️ HALLAZGOS CRÍTICOS ({}):", findings.len());
        for (key, _) in findings {
            // No imprimimos el valor por seguridad, solo la clave expuesta
            println!("   [!] Variable sensible encontrada: {}", key);
        }
        println!("\nRECOMENDACIÓN: Mover estos secretos a un Vault o gestor de secretos.");
    }
}
