use crate::arc_rwlock_serde;
use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Arc};
use napi_derive::napi;
use tokenizers::AddedToken;
use tokenizers::models::{bpe::BpeTrainer, TrainerWrapper};
use tokenizers::Trainer;



#[derive(Clone, Serialize, Deserialize)]
#[napi(js_name = "Trainer2")]
pub struct JsTrainer {
  #[serde(flatten, with = "arc_rwlock_serde")]
  pub(crate) trainer: Option<Arc<RwLock<TrainerWrapper>>>,
}

impl From<TrainerWrapper> for JsTrainer {
  fn from(trainer: TrainerWrapper) -> Self {
    Self {
      trainer: Some(Arc::new(RwLock::new(trainer))),
    }
  }
}

#[napi]
impl JsTrainer {

   #[napi(constructor)]
   pub fn new() -> Self {
      Self { trainer: None }
  }

  #[napi]
  pub fn should_show_progress(&self) -> bool {
    let result = match &self.trainer {
      Some(trainer_arc) => {
       match trainer_arc.read() {
         Ok(lock) => {
          let trainer = &*lock; // Borrow then dereference
          let should_show_progress = trainer.should_show_progress();
          should_show_progress
        },
        Err(_) => false   
       }
      },
      None => false,
    };

    return result;
  }
}

#[napi(object)]
pub struct BpeTrainerOptions {
  pub special_tokens: Option<Vec<String>>,
}

#[napi(factory)]
// #[js_function(0)]
pub fn bpe_trainer(options: Option<BpeTrainerOptions>) -> JsTrainer {
  let mut bpe_trainer_builder = BpeTrainer::builder();

  if let Some(tokens) = options.unwrap().special_tokens {
    let special_tokens = tokens
    .into_iter()
    .map(|token| AddedToken::from(token, true))
    .collect::<Vec<AddedToken>>();

    bpe_trainer_builder = bpe_trainer_builder.special_tokens(special_tokens);
  }
  

  let bpe_trainer = bpe_trainer_builder.build();

  let wrapper = TrainerWrapper::BpeTrainer(bpe_trainer);
  JsTrainer::from(wrapper)
}