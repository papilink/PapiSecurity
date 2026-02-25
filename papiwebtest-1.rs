// Papiweb desarrollos informáticos - Professional Audit System
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};
use std::collections::HashMap;

// 1. Definición de tipos de auditoría mejorada
#[derive(Debug, Clone)]
enum AuditType {
    Log(String, AuditMetadata),
    Financial { 
        amount: f64, 
        currency: String, 
        tx_id: String,
        metadata: AuditMetadata 
    },
    CodeAnalysis { 
        file: String, 
        issues_found: u32,
        metadata: AuditMetadata 
    },
}

#[derive(Debug, Clone)]
struct AuditMetadata {
    timestamp: Instant,
    priority: AuditPriority,
    source: String,
}

#[derive(Debug, Clone, PartialEq)]
enum AuditPriority {
    Low,
    Medium,
    High,
    Critical,
}

// 2. Estructura del Dashboard
struct AuditDashboard {
    total_processed: u32,
    logs_processed: u32,
    financial_processed: u32,
    code_processed: u32,
    priority_stats: HashMap<AuditPriority, u32>,
    start_time: Instant,
    last_audit_time: Instant,
    processing_times: Vec<Duration>,
}

impl AuditDashboard {
    fn new() -> Self {
        let mut priority_stats = HashMap::new();
        priority_stats.insert(AuditPriority::Low, 0);
        priority_stats.insert(AuditPriority::Medium, 0);
        priority_stats.insert(AuditPriority::High, 0);
        priority_stats.insert(AuditPriority::Critical, 0);
        
        Self {
            total_processed: 0,
            logs_processed: 0,
            financial_processed: 0,
            code_processed: 0,
            priority_stats,
            start_time: Instant::now(),
            last_audit_time: Instant::now(),
            processing_times: Vec::new(),
        }
    }
    
    fn update(&mut self, audit_type: &AuditType, processing_time: Duration) {
        self.total_processed += 1;
        self.processing_times.push(processing_time);
        
        match audit_type {
            AuditType::Log(_, metadata) => {
                self.logs_processed += 1;
                *self.priority_stats.get_mut(&metadata.priority).unwrap() += 1;
            },
            AuditType::Financial { metadata, .. } => {
                self.financial_processed += 1;
                *self.priority_stats.get_mut(&metadata.priority).unwrap() += 1;
            },
            AuditType::CodeAnalysis { metadata, .. } => {
                self.code_processed += 1;
                *self.priority_stats.get_mut(&metadata.priority).unwrap() += 1;
            },
        }
        
        self.last_audit_time = Instant::now();
    }
    
    fn display(&self) {
        println!("\n{}", "=".repeat(60));
        println!("🔍 PAPIWEB AUDIT DASHBOARD - PROFESSIONAL EDITION");
        println!("{}", "=".repeat(60));
        println!("🏢 Marca: Papiweb desarrollos informáticos");
        println!("{}", "-".repeat(60));
        
        let elapsed = self.start_time.elapsed();
        println!("⏱️  Tiempo de ejecución: {:.2?}", elapsed);
        println!("📊 Total auditorías procesadas: {}", self.total_processed);
        println!("{}", "-".repeat(60));
        
        println!("📋 DESGLOSE POR TIPO:");
        println!("   📝 Logs: {} auditorías", self.logs_processed);
        println!("   💰 Financieras: {} auditorías", self.financial_processed);
        println!("   💻 Análisis código: {} auditorías", self.code_processed);
        println!("{}", "-".repeat(60));
        
        println!("⚠️  DESGLOSE POR PRIORIDAD:");
        for (priority, count) in &self.priority_stats {
            let icon = match priority {
                AuditPriority::Low => "🔵",
                AuditPriority::Medium => "🟡",
                AuditPriority::High => "🟠",
                AuditPriority::Critical => "🔴",
            };
            println!("   {} {:?}: {}", icon, priority, count);
        }
        println!("{}", "-".repeat(60));
        
        if !self.processing_times.is_empty() {
            let avg_time = self.processing_times.iter().sum::<Duration>() / self.processing_times.len() as u32;
            println!("⚡ Rendimiento:");
            println!("   📈 Tiempo promedio procesamiento: {:.2?}", avg_time);
            println!("   🚀 Tasa procesamiento: {:.2} ops/segundo", 
                     self.total_processed as f64 / elapsed.as_secs_f64());
        }
        
        println!("{}", "=".repeat(60));
        println!("✅ Dashboard actualizado: {}", chrono::Local::now().format("%H:%M:%S"));
        println!("{}", "=".repeat(60));
    }
}

// 3. Estructura principal del Auditor mejorada
struct Auditor {
    id: u32,
    name: String,
}

impl Auditor {
    fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
    
    fn run(&self, tx: mpsc::Sender<(AuditType, Instant)>, audit_type: AuditType) {
        let auditor_name = self.name.clone();
        let auditor_id = self.id;
        
        thread::spawn(move || {
            let start_time = Instant::now();
            
            // Simulamos procesamiento intenso
            thread::sleep(Duration::from_millis(100)); // Simulación de trabajo
            
            let processing_time = start_time.elapsed();
            
            tx.send((audit_type, processing_time))
                .expect("Error al enviar reporte de auditoría");
            
            println!("[Auditor {}:{}] Procesado en {:?}", 
                     auditor_name, auditor_id, processing_time);
        });
    }
}

// 4. Función de testeo
fn run_audit_tests() {
    println!("\n{}", "#".repeat(60));
    println!("🧪 INICIANDO BATERÍA DE TESTS - PAPIWEB AUDIT SYSTEM");
    println!("{}", "#".repeat(60));
    
    // Test 1: Verificar creación de dashboard
    let dashboard = AuditDashboard::new();
    assert_eq!(dashboard.total_processed, 0);
    println!("✅ Test 1: Dashboard creado correctamente");
    
    // Test 2: Verificar metadata
    let metadata = AuditMetadata {
        timestamp: Instant::now(),
        priority: AuditPriority::High,
        source: "test_system".to_string(),
    };
    assert_eq!(metadata.priority, AuditPriority::High);
    println!("✅ Test 2: Metadata funciona correctamente");
    
    // Test 3: Verificar procesamiento concurrente
    let (tx, rx) = mpsc::channel();
    let auditor = Auditor::new(999, "TEST_AUDITOR");
    
    let test_audit = AuditType::Log(
        "TEST: Mensaje de prueba".to_string(),
        metadata.clone()
    );
    
    auditor.run(tx.clone(), test_audit);
    
    // Pequeña pausa para permitir procesamiento
    thread::sleep(Duration::from_millis(200));
    
    // Verificar que se recibió algo
    drop(tx);
    let received: Vec<_> = rx.iter().collect();
    assert!(!received.is_empty(), "Debería haber recibido al menos un mensaje");
    println!("✅ Test 3: Procesamiento concurrente funciona");
    
    println!("{}", "#".repeat(60));
    println!("🎉 TODOS LOS TESTS PASARON EXITOSAMENTE");
    println!("{}", "#".repeat(60));
}

// 5. Función para generar auditorías de prueba
fn generate_test_audits(count: u32) -> Vec<AuditType> {
    let mut audits = Vec::new();
    let priorities = [AuditPriority::Low, AuditPriority::Medium, 
                      AuditPriority::High, AuditPriority::Critical];
    
    for i in 0..count {
        let priority = &priorities[i as usize % priorities.len()];
        let metadata = AuditMetadata {
            timestamp: Instant::now(),
            priority: priority.clone(),
            source: format!("source_{}", i),
        };
        
        let audit = match i % 3 {
            0 => AuditType::Log(
                format!("LOG: Evento de prueba #{}", i),
                metadata
            ),
            1 => AuditType::Financial {
                amount: 1000.0 + (i as f64 * 100.0),
                currency: "USD".to_string(),
                tx_id: format!("TX-{:04}", i),
                metadata,
            },
            _ => AuditType::CodeAnalysis {
                file: format!("file_{}.rs", i),
                issues_found: i % 10,
                metadata,
            },
        };
        
        audits.push(audit);
    }
    
    audits
}

fn main() {
    println!("🚀 PAPIWEB AUDIT SYSTEM v2.0 - INICIANDO...");
    println!("Copyright © 2024 Papiweb desarrollos informáticos");
    
    // Ejecutar tests
    run_audit_tests();
    
    // Canal seguro: Múltiples productores, un solo consumidor
    let (tx, rx) = mpsc::channel();
    
    // Crear dashboard
    let mut dashboard = AuditDashboard::new();
    
    // Crear auditor
    let auditor = Auditor::new(1, "PRINCIPAL");
    
    println!("\n📡 INICIANDO AUDITORÍAS CONCURRENTES...");
    
    // Generar y enviar auditorías de prueba
    let test_audits = generate_test_audits(10);
    
    for audit in test_audits {
        let tx_clone = tx.clone();
        auditor.run(tx_clone, audit);
    }
    
    // Pequeña pausa para asegurar que todos los hilos comiencen
    thread::sleep(Duration::from_millis(100));
    
    // 3. Consumidor Centralizado con Dashboard
    drop(tx); // Cerramos el transmisor original
    
    println!("\n📊 INICIANDO PROCESAMIENTO CENTRALIZADO CON DASHBOARD...");
    
    for (received, processing_time) in rx {
        match &received {
            AuditType::Log(msg, metadata) => {
                println!("[{}][LOG AUDIT][{:?}]: {}", 
                         metadata.source, metadata.priority, msg);
            },
            AuditType::Financial { amount, tx_id, metadata, .. } => {
                println!("[{}][FINANCIAL AUDIT][{:?}]: Alerta en transacción {} por ${:.2}", 
                         metadata.source, metadata.priority, tx_id, amount);
            },
            AuditType::CodeAnalysis { file, issues_found, metadata } => {
                println!("[{}][CODE AUDIT][{:?}]: Archivo {} analizado. Hallazgos: {}", 
                         metadata.source, metadata.priority, file, issues_found);
            },
        }
        
        // Actualizar dashboard
        dashboard.update(&received, processing_time);
        
        // Mostrar dashboard periódicamente
        if dashboard.total_processed % 3 == 0 {
            dashboard.display();
            thread::sleep(Duration::from_millis(500)); // Pausa para mejor visualización
        }
    }
    
    // Mostrar dashboard final
    println!("\n📊 DASHBOARD FINAL:");
    dashboard.display();
    
    println!("\n✨ Sistema de auditoría completado exitosamente");
    println!("🏢 Papiweb desarrollos informáticos - Calidad y Profesionalismo");
}