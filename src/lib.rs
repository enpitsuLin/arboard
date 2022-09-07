#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::borrow::Cow;

use napi::{bindgen_prelude::*, JsBuffer};

#[napi]
pub struct Clipboard {
  inner: arboard::Clipboard,
}

#[napi(object)]
pub struct ImageData {
  pub height: u32,
  pub width: u32,
  pub bytes: JsBuffer,
}

fn clipboard_error_to_js_error(err: arboard::Error) -> Error {
  Error::new(Status::GenericFailure, format!("{err}"))
}

#[napi]
impl Clipboard {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Clipboard {
      inner: arboard::Clipboard::new().map_err(clipboard_error_to_js_error)?,
    })
  }

  #[napi]
  pub fn set_text(&mut self, text: String) -> Result<()> {
    self
      .inner
      .set_text(text)
      .map_err(clipboard_error_to_js_error)?;
    Ok(())
  }

  #[napi]
  pub fn get_text(&mut self) -> Result<String> {
    self.inner.get_text().map_err(clipboard_error_to_js_error)
  }

  #[napi]
  /// Returns a object contains raw RGBA pixels data buffer and size 
  pub fn get_image(&mut self, env: Env) -> Result<ImageData> {
    let image = self
      .inner
      .get_image()
      .map_err(clipboard_error_to_js_error)?;

    let bytes = unsafe {
      env.create_buffer_with_borrowed_data(
        image.bytes.as_ptr(),
        image.bytes.len(),
        image.clone(),
        |i, _| {
          drop(i);
        },
      )
    }
    .map(|b| b.into_raw())?;

    let width = image.width;

    let height = image.height;

    Ok(ImageData {
      width: width.try_into().unwrap(),
      height: height.try_into().unwrap(),
      bytes,
    })
  }

  #[napi]
  /// RGBA bytes
  pub fn set_image(&mut self, width: u32, height: u32, image: Buffer) -> Result<()> {
    self
      .inner
      .set_image(arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::Borrowed(image.as_ref()),
      })
      .map_err(clipboard_error_to_js_error)
  }
}
