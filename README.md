# PapiSecurity
🛡️ Garantía de Integridad Papiweb
Sin Instalación: No ensucia el sistema del cliente con registros ni variables globales.
Portabilidad: Funciona en servidores sin acceso a internet (si el target es local).
Firma de Seguridad: Se recomienda enviar el hash SHA-256 junto al archivo para que el cliente verifique que el binario no fue alterado:
Windows PowerShell: Get-FileHash ./Papiweb_Auditor_v1.exe

---

### Un toque de "Papiweb" para el cliente:
Cuando entregues el `.exe`, enviale también un pequeño archivo de texto que diga:
> "Este binario de **Papiweb** ha sido compilado para ser ligero y no invasivo. Respeta la memoria de su servidor y la estabilidad de sus conexiones activas."

** preguntar por script en Rust que verifique si el servidor tiene los puertos correctos abiertos antes de lanzar el auditor pesado?**



Haz que la seguridad sea un estado de bienestar para todos los que trabajamos en el desarrollo. gracias  
