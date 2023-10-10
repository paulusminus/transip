// impl Drop for ApiClient {
//     fn drop(&mut self) {
//         if let Some(token) = self.token.take() {
//             if let Err(e) =  self.configuration.token_file().dump_json(token) {
//                 tracing::error!("Failed to dump token to file: {}", e);
//             }
//         }
//     }
// }

// pub trait Persist<T>
// where
//     T: Serialize + DeserializeOwned,
// {
//     fn load_json(&self) -> Result<T>;
//     fn dump_json(&self, t: T) -> Result<()>;
// }

// impl<P, T> Persist<T> for P
// where
//     P: AsRef<Path>,
//     T: Serialize + DeserializeOwned,
// {
//     fn dump_json(&self, t: T) -> Result<()> {
//         std::fs::File::create(self)
//             .map_err(Into::into)
//             .and_then(|file| ureq::serde_json::to_writer_pretty(file, &t).map_err(Into::into))
//     }

//     fn load_json(&self) -> Result<T> {
//         std::fs::File::open(self)
//             .map_err(Into::into)
//             .and_then(|file| ureq::serde_json::from_reader(file).map_err(Into::into))
//     }
// }
