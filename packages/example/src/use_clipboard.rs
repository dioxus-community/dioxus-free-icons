use anyhow::{Context, Result};
use gloo_utils::errors::JsError;
pub async fn copy_to_clipboard(text: &str) -> Result<()> {
    wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .context("Unable to get the window object")?
            .navigator()
            .clipboard()
            // .context("Unable to get the clipboard object")?
            .write_text(text),
    )
    .await
    .map_err(|err| JsError::try_from(err).unwrap())
    .context("Unable to copy text into the clipboard")?;

    Ok(())
}
