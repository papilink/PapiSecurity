
// Papiweb desarrollos informaticos
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

// 1. Definición de tipos de auditoría
enum AuditType {
    Log(String),
    Financial { amount: f64, currency: String, tx_id: String },
    CodeAnalysis { file: String, issues_found: u32 },
}

// 2. Estructura principal del Auditor
struct Auditor {
    id: u32,
}

impl Auditor {
    fn run(&self, tx: mpsc::Sender<AuditType>, data: AuditType) {
        // Rust garantiza que 'tx' se puede clonar de forma segura entre hilos
        thread::spawn(move || {
            // Simulamos procesamiento intenso
            tx.send(data).expect("Error al enviar reporte de auditoría");
        });
    }
}

fn main() {
    // Canal seguro: Múltiples productores, un solo consumidor
    let (tx, rx) = mpsc::channel();
    
    let auditor = Auditor { id: 1 };

    // --- PROCESAMIENTO CONCURRENTE ---

    // A. Auditoría de Log (Stress: Alto volumen)
    let tx_logs = tx.clone();
    auditor.run(tx_logs, AuditType::Log("INFO: Acceso autorizado user_123".to_string()));

    // B. Auditoría Financiera (Stress: Integridad/Precisión)
    let tx_fin = tx.clone();
    auditor.run(tx_fin, AuditType::Financial { 
        amount: 1500.50, 
        currency: "USD".to_string(), 
        tx_id: "TX-9982".to_string() 
    });

    // C. Auditoría de Código (Stress: Uso de CPU/AST)
    let tx_code = tx.clone();
    auditor.run(tx_code, AuditType::CodeAnalysis { 
        file: "payment_processor.rs".to_string(), 
        issues_found: 2 
    });

    // 3. Consumidor Centralizado (Evita bloqueos y corrupción de datos)
    drop(tx); // Cerramos el transmisor original para que el loop termine

    println!("Iniciando procesamiento centralizado...");
    for received in rx {
        match received {
            AuditType::Log(msg) => println!("[LOG AUDIT]: {}", msg),
            AuditType::Financial { amount, tx_id, .. } => {
                println!("[FINANCIAL AUDIT]: Alerta en transacción {} por ${}", tx_id, amount)
            },
            AuditType::CodeAnalysis { file, issues_found } => {
                println!("[CODE AUDIT]: Archivo {} analizado. Hallazgos: {}", file, issues_found)
            },
        }
    }
}
