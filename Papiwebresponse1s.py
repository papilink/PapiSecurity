# Papiweb desarrollos informaticos
import httpx
import asyncio
import time

# --- CONFIGURACIÓN PAPIWEB ---
TARGET_URL = "https://api.tu-cliente.com"
WORDLIST = ["admin", "v2", "backup", "config", "setup", "internal", "metrics"]
DELAY_BETWEEN_REQUESTS = 0.5  # Segundos (Ajustable para no estresar el CPU)
USER_AGENT = "Papiweb-Security-Auditor/1.0 (Conscious-Fuzzing-Mode)"

async fn p_fuzz_educated(endpoint: str, client: httpx.AsyncClient):
    url = f"{TARGET_URL}/{endpoint}"
    
    try:
        # Iniciamos la petición con un timeout prudente
        response = await client.get(url, timeout=5.0)
        
        # Lógica de interpretación de resultados
        if response.status_code == 200:
            print(f"[🟢 EXPUESTO] Ruta encontrada: {url} (Size: {len(response.content)})")
        elif response.status_code == 403:
            print(f"[🟡 PROTEGIDO] Ruta detectada pero prohibida: {url}")
        elif response.status_code == 429:
            print(f"[🔴 ALERTA] Límite de tasa alcanzado. Frenando auditoría para proteger el servidor.")
            return False # Señal para detener el bucle
            
    except httpx.RequestError as exc:
        print(f"[❌ ERROR] No se pudo conectar a {url}: {exc}")
    
    # Respetamos el tiempo del desarrollador y del servidor
    await asyncio.sleep(DELAY_BETWEEN_REQUESTS)
    return True

async fn main():
    print(f"--- Iniciando Auditoría Consciente de Papiweb en: {TARGET_URL} ---")
    print(f"Respetando integridad de APIs y WebSockets...\n")

    async with httpx.AsyncClient(headers={"User-Agent": USER_AGENT}) as client:
        tasks = []
        for word in WORDLIST:
            # Ejecución secuencial controlada (no lanzamos 1000 hilos a la vez)
            continue_audit = await p_fuzz_educated(word, client)
            if not continue_audit:
                break

    print("\n--- Auditoría finalizada con éxito y sin incidentes de carga ---")

if __name__ == "__main__":
    asyncio.run(main())
