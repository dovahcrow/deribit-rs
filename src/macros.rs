#[macro_export]
macro_rules! define_request {
    (
        Name => $name: ident;
        Method => $method: expr;
        Request => { $($req_def:tt)* };
        Response => { $($resp_def:tt)* };
    ) => {
        crate::define_request! {
            Name => $name;
            Method => $method;
            Payload => true;
            Request => { $($req_def)* };
            Response => { $($resp_def)* };
        }
    };

    (
        Name => $name: ident;
        Method => $method: expr;
        Payload => $has_payload: expr;
        Request => { $($req_def:tt)* };
        Response => { $($resp_def:tt)* };
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Request>] {
                $($req_def)*
            }

            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Response>] {
                $($resp_def)*
            }

            impl crate::models::Request for [<$name Request>] {
                const METHOD: &'static str = $method;
                const HAS_PAYLOAD: bool = $has_payload;
                type Response = [<$name Response>];
            }
        }
    };

    (
        Name => $name: ident;
        Method => $method: expr;
        Request => { $($req_def:tt)* };
        Response => $resp_ty: ty;
    ) => {
        crate::define_request! {
            Name => $name;
            Method => $method;
            Payload => true;
            Request => { $($req_def)* };
            Response => $resp_ty;
        }
    };

    (
        Name => $name: ident;
        Method => $method: expr;
        Payload => $has_payload: expr;
        Request => { $($req_def:tt)* };
        Response => $resp_ty: ty;
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Request>] {
                $($req_def)*
            }

            impl crate::models::Request for [<$name Request>] {
                const METHOD: &'static str = $method;
                const HAS_PAYLOAD: bool = $has_payload;
                type Response = $resp_ty;
            }
        }
    };
}
