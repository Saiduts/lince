# Almacenamiento de Datos

Este módulo define cómo se persisten los datos dentro del framework Lince.

---

## Estructura del Módulo

- **[Introducción al Storage](intro.md)**
  Propósito general del sistema de almacenamiento y su rol en el flujo IoT.

- **[MemoryStorage](memory_storage.md)**
  Almacenamiento en RAM. Sin persistencia, ideal para pruebas y desarrollo.

- **[SqliteStorage](sqlite_storage.md)**
  Almacenamiento persistente en SQLite. Incluye `flush_pending()` para reenviar
  automáticamente los mensajes que no pudieron enviarse por desconexión.

- **[Implementar Storage Personalizado](custom_storage.md)**
  Guía para crear nuevas implementaciones sobre el trait `Storage`.

---

## Comparativa

| Característica | `MemoryStorage` | `SqliteStorage` |
|----------------|-----------------|-----------------|
| Persistencia | No | Sí |
| Reintento por desconexión | No | Sí, con `flush_pending()` |
| Dependencias extra | Ninguna | `rusqlite` |
| Ideal para | Tests, desarrollo | Producción |

---

## Conceptos Clave

- **Trait `Storage`**: interfaz común para todas las implementaciones.
- **`sent = 0 / 1`**: columna en SQLite que distingue pendientes de enviados.
- **`flush_pending(comm)`**: el storage reenvía su propia cola usando el comunicador
  que se le pase. No requiere ningún componente externo adicional.