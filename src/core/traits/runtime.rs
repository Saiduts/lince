/// Controla el ciclo de ejecución del sistema.
///
/// Se encarga de coordinar sensores, actuadores, comunicación y almacenamiento.
/// Normalmente se implementa en el **núcleo** del framework.
pub trait RuntimeController {
    /// Ejecuta el ciclo principal del programa.
    fn run(&mut self);
}
