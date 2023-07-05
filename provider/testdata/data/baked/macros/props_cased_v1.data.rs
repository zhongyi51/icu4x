// @generated
/// Implement [`DataProvider<CasedV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_cased_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.64"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_CASED_V1: &'static <icu_properties::provider::CasedV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0a\0\0\0{\0\0\0\xAA\0\0\0\xAB\0\0\0\xB5\0\0\0\xB6\0\0\0\xBA\0\0\0\xBB\0\0\0\xC0\0\0\0\xD7\0\0\0\xD8\0\0\0\xF7\0\0\0\xF8\0\0\0\xBB\x01\0\0\xBC\x01\0\0\xC0\x01\0\0\xC4\x01\0\0\x94\x02\0\0\x95\x02\0\0\xB9\x02\0\0\xC0\x02\0\0\xC2\x02\0\0\xE0\x02\0\0\xE5\x02\0\0E\x03\0\0F\x03\0\0p\x03\0\0t\x03\0\0v\x03\0\0x\x03\0\0z\x03\0\0~\x03\0\0\x7F\x03\0\0\x80\x03\0\0\x86\x03\0\0\x87\x03\0\0\x88\x03\0\0\x8B\x03\0\0\x8C\x03\0\0\x8D\x03\0\0\x8E\x03\0\0\xA2\x03\0\0\xA3\x03\0\0\xF6\x03\0\0\xF7\x03\0\0\x82\x04\0\0\x8A\x04\0\x000\x05\0\x001\x05\0\0W\x05\0\0`\x05\0\0\x89\x05\0\0\xA0\x10\0\0\xC6\x10\0\0\xC7\x10\0\0\xC8\x10\0\0\xCD\x10\0\0\xCE\x10\0\0\xD0\x10\0\0\xFB\x10\0\0\xFC\x10\0\0\0\x11\0\0\xA0\x13\0\0\xF6\x13\0\0\xF8\x13\0\0\xFE\x13\0\0\x80\x1C\0\0\x89\x1C\0\0\x90\x1C\0\0\xBB\x1C\0\0\xBD\x1C\0\0\xC0\x1C\0\0\0\x1D\0\0\xC0\x1D\0\0\0\x1E\0\0\x16\x1F\0\0\x18\x1F\0\0\x1E\x1F\0\0 \x1F\0\0F\x1F\0\0H\x1F\0\0N\x1F\0\0P\x1F\0\0X\x1F\0\0Y\x1F\0\0Z\x1F\0\0[\x1F\0\0\\\x1F\0\0]\x1F\0\0^\x1F\0\0_\x1F\0\0~\x1F\0\0\x80\x1F\0\0\xB5\x1F\0\0\xB6\x1F\0\0\xBD\x1F\0\0\xBE\x1F\0\0\xBF\x1F\0\0\xC2\x1F\0\0\xC5\x1F\0\0\xC6\x1F\0\0\xCD\x1F\0\0\xD0\x1F\0\0\xD4\x1F\0\0\xD6\x1F\0\0\xDC\x1F\0\0\xE0\x1F\0\0\xED\x1F\0\0\xF2\x1F\0\0\xF5\x1F\0\0\xF6\x1F\0\0\xFD\x1F\0\0q \0\0r \0\0\x7F \0\0\x80 \0\0\x90 \0\0\x9D \0\0\x02!\0\0\x03!\0\0\x07!\0\0\x08!\0\0\n!\0\0\x14!\0\0\x15!\0\0\x16!\0\0\x19!\0\0\x1E!\0\0$!\0\0%!\0\0&!\0\0'!\0\0(!\0\0)!\0\0*!\0\0.!\0\0/!\0\x005!\0\09!\0\0:!\0\0<!\0\0@!\0\0E!\0\0J!\0\0N!\0\0O!\0\0`!\0\0\x80!\0\0\x83!\0\0\x85!\0\0\xB6$\0\0\xEA$\0\0\0,\0\0\xE5,\0\0\xEB,\0\0\xEF,\0\0\xF2,\0\0\xF4,\0\0\0-\0\0&-\0\0'-\0\0(-\0\0--\0\0.-\0\0@\xA6\0\0n\xA6\0\0\x80\xA6\0\0\x9E\xA6\0\0\"\xA7\0\0\x88\xA7\0\0\x8B\xA7\0\0\x8F\xA7\0\0\x90\xA7\0\0\xCB\xA7\0\0\xD0\xA7\0\0\xD2\xA7\0\0\xD3\xA7\0\0\xD4\xA7\0\0\xD5\xA7\0\0\xDA\xA7\0\0\xF2\xA7\0\0\xF7\xA7\0\0\xF8\xA7\0\0\xFB\xA7\0\x000\xAB\0\0[\xAB\0\0\\\xAB\0\0j\xAB\0\0p\xAB\0\0\xC0\xAB\0\0\0\xFB\0\0\x07\xFB\0\0\x13\xFB\0\0\x18\xFB\0\0!\xFF\0\0;\xFF\0\0A\xFF\0\0[\xFF\0\0\0\x04\x01\0P\x04\x01\0\xB0\x04\x01\0\xD4\x04\x01\0\xD8\x04\x01\0\xFC\x04\x01\0p\x05\x01\0{\x05\x01\0|\x05\x01\0\x8B\x05\x01\0\x8C\x05\x01\0\x93\x05\x01\0\x94\x05\x01\0\x96\x05\x01\0\x97\x05\x01\0\xA2\x05\x01\0\xA3\x05\x01\0\xB2\x05\x01\0\xB3\x05\x01\0\xBA\x05\x01\0\xBB\x05\x01\0\xBD\x05\x01\0\x80\x07\x01\0\x81\x07\x01\0\x83\x07\x01\0\x86\x07\x01\0\x87\x07\x01\0\xB1\x07\x01\0\xB2\x07\x01\0\xBB\x07\x01\0\x80\x0C\x01\0\xB3\x0C\x01\0\xC0\x0C\x01\0\xF3\x0C\x01\0\xA0\x18\x01\0\xE0\x18\x01\0@n\x01\0\x80n\x01\0\0\xD4\x01\0U\xD4\x01\0V\xD4\x01\0\x9D\xD4\x01\0\x9E\xD4\x01\0\xA0\xD4\x01\0\xA2\xD4\x01\0\xA3\xD4\x01\0\xA5\xD4\x01\0\xA7\xD4\x01\0\xA9\xD4\x01\0\xAD\xD4\x01\0\xAE\xD4\x01\0\xBA\xD4\x01\0\xBB\xD4\x01\0\xBC\xD4\x01\0\xBD\xD4\x01\0\xC4\xD4\x01\0\xC5\xD4\x01\0\x06\xD5\x01\0\x07\xD5\x01\0\x0B\xD5\x01\0\r\xD5\x01\0\x15\xD5\x01\0\x16\xD5\x01\0\x1D\xD5\x01\0\x1E\xD5\x01\0:\xD5\x01\0;\xD5\x01\0?\xD5\x01\0@\xD5\x01\0E\xD5\x01\0F\xD5\x01\0G\xD5\x01\0J\xD5\x01\0Q\xD5\x01\0R\xD5\x01\0\xA6\xD6\x01\0\xA8\xD6\x01\0\xC1\xD6\x01\0\xC2\xD6\x01\0\xDB\xD6\x01\0\xDC\xD6\x01\0\xFB\xD6\x01\0\xFC\xD6\x01\0\x15\xD7\x01\0\x16\xD7\x01\x005\xD7\x01\x006\xD7\x01\0O\xD7\x01\0P\xD7\x01\0o\xD7\x01\0p\xD7\x01\0\x89\xD7\x01\0\x8A\xD7\x01\0\xA9\xD7\x01\0\xAA\xD7\x01\0\xC3\xD7\x01\0\xC4\xD7\x01\0\xCC\xD7\x01\0\0\xDF\x01\0\n\xDF\x01\0\x0B\xDF\x01\0\x1F\xDF\x01\0%\xDF\x01\0+\xDF\x01\x000\xE0\x01\0n\xE0\x01\0\0\xE9\x01\0D\xE9\x01\x000\xF1\x01\0J\xF1\x01\0P\xF1\x01\0j\xF1\x01\0p\xF1\x01\0\x8A\xF1\x01\0") }, 4526u32)
            });
        }
        #[clippy::msrv = "1.64"]
        impl icu_provider::DataProvider<icu_properties::provider::CasedV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::CasedV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_CASED_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::CasedV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
