fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(
        &["./proto/mesh.proto"],
        &["./proto/"], // include paths for imports
    )?;

    Ok(())
}
