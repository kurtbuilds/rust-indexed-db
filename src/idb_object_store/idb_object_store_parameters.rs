use crate::idb_key_path::IdbKeyPath;
use delegate_display::DelegateDebug;
use fancy_constructor::new;
use web_sys::IdbObjectStoreParameters as Base;

/// Wrapper for [`IdbObjectStore`](crate::idb_object_store::IdbObjectStore) optional parameters
#[derive(DelegateDebug, Clone, new)]
pub struct IdbObjectStoreParameters(#[new(val(Base::new()))] Base);

impl IdbObjectStoreParameters {
    /// Set the `auto_increment` option
    #[inline]
    pub fn auto_increment(&mut self, val: bool) -> &mut Self {
        self.0.auto_increment(val);
        self
    }

    /// Set the `key_path` option
    pub fn key_path(&mut self, val: Option<&IdbKeyPath>) -> &mut Self {
        self.0.key_path(val.map(IdbKeyPath::as_js_value));
        self
    }

    /// Get the enclosed `web_sys` parameters object
    #[inline]
    #[must_use]
    pub fn as_js_value(&self) -> &web_sys::IdbObjectStoreParameters {
        self.0.as_ref()
    }
}

impl AsRef<web_sys::IdbObjectStoreParameters> for IdbObjectStoreParameters {
    #[inline]
    fn as_ref(&self) -> &web_sys::IdbObjectStoreParameters {
        self.as_js_value()
    }
}

impl Default for IdbObjectStoreParameters {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<web_sys::IdbObjectStoreParameters> for IdbObjectStoreParameters {
    #[inline]
    fn from(raw: web_sys::IdbObjectStoreParameters) -> Self {
        Self(raw)
    }
}


#[cfg(feature = "json")]
mod extras {
    use serde_json::Value;
    use super::*;

    impl Into<IdbObjectStoreParameters> for Option<Value> {
        fn into(self) -> IdbObjectStoreParameters {
            match self {
                Some(val) => val.into(),
                None => IdbObjectStoreParameters::default(),
            }
        }
    }

    impl Into<IdbObjectStoreParameters> for Value {
        fn into(self) -> IdbObjectStoreParameters {
            let mut params = IdbObjectStoreParameters::default();
            if let Some(val) = self.get("auto_increment") {
                params.auto_increment(val.as_bool().unwrap());
            }
            if let Some(val) = self.get("key_path") {
                params.key_path(Some(&IdbKeyPath::str(val.as_str().unwrap())));
            }
            params
        }
    }
}