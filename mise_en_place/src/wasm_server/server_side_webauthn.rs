use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_sessions::extractors::WritableSession;
use uuid::Uuid;
use webauthn_rs::prelude::{
    CreationChallengeResponse, PasskeyAuthentication, PasskeyRegistration, PublicKeyCredential,
    RegisterPublicKeyCredential,
};

use crate::wasm_server::ServerData;

pub(crate) async fn start_register(
    Extension(server_data): Extension<ServerData>,
    mut session: WritableSession,
    Path(username): Path<String>,
) -> Result<Json<CreationChallengeResponse>, &'static str> {
    let unique_user_id = {
        let guard = server_data.user_data.lock().await;
        guard
            .name_to_id
            .get(&username)
            .copied()
            .unwrap_or_else(Uuid::new_v4)
    };
    session.remove("reg_state");
    let exclude_credentials = {
        let users_guard = server_data.user_data.lock().await;
        users_guard
            .keys
            .get(&unique_user_id)
            .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
    };
    let res = match server_data.webauthn.start_passkey_registration(
        unique_user_id,
        &username,
        &username,
        exclude_credentials,
    ) {
        Ok((ccr, reg_state)) => {
            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the reg_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            session
                .insert("reg_state", (username, unique_user_id, reg_state))
                .expect("Failed to insert");
            tracing::info!("Registration Successful!");
            Json(ccr)
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            return Err("Unknown");
        }
    };
    Ok(res)
}

pub(crate) async fn finish_register(
    Extension(app_state): Extension<ServerData>,
    mut session: WritableSession,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> Result<impl IntoResponse, &'static str> {
    let (username, user_unique_id, reg_state): (String, Uuid, PasskeyRegistration) =
        session.get("reg_state").ok_or("Corrupt Session")?; //Corrupt Session

    session.remove("reg_state");

    let res = match app_state
        .webauthn
        .finish_passkey_registration(&reg, &reg_state)
    {
        Ok(sk) => {
            let mut users_guard = app_state.user_data.lock().await;

            //TODO: This is where we would store the credential in a db, or persist them in some other way.
            users_guard
                .keys
                .entry(user_unique_id)
                .and_modify(|keys| keys.push(sk.clone()))
                .or_insert_with(|| vec![sk.clone()]);

            users_guard.name_to_id.insert(username, user_unique_id);

            StatusCode::OK
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };
    Ok(res)
}

pub(crate) async fn start_authentication(
    Extension(app_state): Extension<ServerData>,
    mut session: WritableSession,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, &'static str> {
    tracing::info!("Start Authentication");
    // We get the username from the URL, but you could get this via form submission or
    // some other process.

    // Remove any previous authentication that may have occured from the session.
    session.remove("auth_state");

    // Get the set of keys that the user possesses
    let users_guard = app_state.user_data.lock().await;

    // Look up their unique id from the username
    let user_unique_id = users_guard
        .name_to_id
        .get(&username)
        .copied()
        .ok_or("User Not Found")?;

    let allow_credentials = users_guard
        .keys
        .get(&user_unique_id)
        .ok_or("User Has No Credentials")?;

    let res = match app_state
        .webauthn
        .start_passkey_authentication(allow_credentials)
    {
        Ok((rcr, auth_state)) => {
            // Drop the mutex to allow the mut borrows below to proceed
            drop(users_guard);

            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the auth_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            session
                .insert("auth_state", (user_unique_id, auth_state))
                .expect("Failed to insert");
            Json(rcr)
        }
        Err(e) => {
            tracing::debug!("challenge_authenticate -> {:?}", e);
            return Err("Unknown");
        }
    };
    Ok(res)
}

pub(crate) async fn finish_authentication(
    Extension(app_state): Extension<ServerData>,
    mut session: WritableSession,
    Json(auth): Json<PublicKeyCredential>,
) -> Result<impl IntoResponse, &'static str> {
    let (user_unique_id, auth_state): (Uuid, PasskeyAuthentication) =
        session.get("auth_state").ok_or("CorruptSession")?;

    session.remove("auth_state");

    let res = match app_state
        .webauthn
        .finish_passkey_authentication(&auth, &auth_state)
    {
        Ok(auth_result) => {
            let mut users_guard = app_state.user_data.lock().await;

            // Update the credential counter, if possible.
            users_guard
                .keys
                .get_mut(&user_unique_id)
                .map(|keys| {
                    keys.iter_mut().for_each(|sk| {
                        // This will update the credential if it's the matching
                        // one. Otherwise it's ignored. That is why it is safe to
                        // iterate this over the full list.
                        sk.update_credential(&auth_result);
                    })
                })
                .ok_or("User Has No Credentials")?;
            StatusCode::OK
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };
    tracing::info!("Authentication Successful!");
    Ok(res)
}
