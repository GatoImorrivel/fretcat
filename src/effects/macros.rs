macro_rules! effects {
  ($($effect:ident),*) => {
    use std::any::Any;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum Effects {
      $(
        $effect($effect),
      )*
    }

    impl Effects {
      pub fn into_inner(self) -> Option<Box<dyn Any>> {
        match self {
          $(
            Effects::$effect(effect) => Some(Box::new(effect) as Box<dyn Any>),
          )*
        }
      }
    }

    $(
      impl Into<Effects> for $effect {
        fn into(self) -> Effects {
          Effects::$effect(self)
        }
      }
    )*
  };
}
