# Papiweb desarrollos informaticos
# 🛠️ Papiweb Security CLI: Compilación y Despliegue

Este documento detalla cómo transformar los scripts de auditoría de **Papiweb** en binarios autónomos (.exe para Windows o binarios de Linux). Esto permite que el cliente ejecute la auditoría en su infraestructura con un solo clic, sin instalar Python ni librerías externas.

---

## 📦 Generación del Ejecutable (Build)

Utilizamos `PyInstaller` para empaquetar el script, todas sus dependencias (`rich`, `httpx`) y el intérprete de Python en un único archivo.

### 1. Preparar el entorno
```bash
pip install pyinstaller rich httpx
