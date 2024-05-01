pub fn sumar_dos_numeros(numero1: f32, numero2: f32) -> f32 {
    return numero1 + numero2;
}

pub fn restar_dos_numeros(numero1: f32, numero2: f32) -> f32 {
    return numero1 - numero2;
}

pub fn dividir_dos_numeros(numero1: f32, numero2: f32) -> Result<f32, &'static str> {
    if numero2 == 0.0 {
        Err("Error, no puedes dividir un número entre cero")
    } else {
        Ok(numero1 / numero2)
    }
}

