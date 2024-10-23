use web3::types::Address;
use bcrypt::{hash, DEFAULT_COST};

pub fn register(user: User) -> Result<String, String> {
    // Hashear contraseña
    let hashed_password = hash(user.password, DEFAULT_COST).unwrap();

    // Crear nuevo usuario en la blockchain
    let address = Address::new(user.address);
    let tx = web3::Transaction::new(
        address,
        user.username.clone(),
        hashed_password,
    );
    web3::send_transaction(tx).unwrap();

    // Devolver token de autenticación
    let token = jsonwebtoken::encode(
        &User {
            id: user.id.clone(),
            username: user.username.clone(),
            address: user.address.clone(),
        },
        &jsonwebtoken::Header::default(),
    ).unwrap();

    Ok(token)
}

pub fn login(username: String, password: String) -> Result<String, String> {
    // Buscar usuario en la blockchain
    let user = web3::get_user(username).unwrap();

    // Verificar contraseña
    if bcrypt::verify(password, &user.password).unwrap() {
        // Devolver token de autenticación
        let token = jsonwebtoken::encode(
            &User {
                id: user.id.clone(),
                username: user.username.clone(),
                address: user.address.clone(),
            },
            &jsonwebtoken::Header::default(),
        ).unwrap();

        Ok(token)
    } else {
        Err("Contraseña incorrecta".to_string())
    }
}

