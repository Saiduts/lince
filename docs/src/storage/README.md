# Almacenamiento de Datos

Este módulo define cómo se manejan y persisten los datos dentro del framework **Lince**.  
Permite almacenar temporal o permanentemente la información recolectada por los sensores antes de ser transmitida o procesada.

---

## Estructura del Módulo

- **[Introducción al Storage](intro.md)**  
  Explica el propósito general del sistema de almacenamiento y su rol dentro del flujo IoT.

- **[MemoryStorage](memory_storage.md)**  
  Implementación por defecto del almacenamiento en memoria. Ideal para pruebas o entornos donde no se requiere persistencia.

- **[Implementar Storage Personalizado](custom_storage.md)**  
  Guía para crear nuevas implementaciones de almacenamiento.
---

## Conceptos Clave

- **Storage Trait:** interfaz común para todas las implementaciones de almacenamiento.  
- **Persistencia Opcional:** el framework no impone una forma de guardar datos; permite definir estrategias según el contexto.  
- **Interoperabilidad:** cada tipo de storage puede integrarse con sensores, redes o módulos de procesamiento.

---