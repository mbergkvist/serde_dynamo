mod aws_lambda_events;
mod aws_sdk;
mod rusoto;
mod rusoto_streams;

pub(crate) use aws_lambda_events::aws_lambda_events_macro;
pub(crate) use aws_sdk::aws_sdk_macro;
pub(crate) use rusoto::rusoto_macro;
pub(crate) use rusoto_streams::rusoto_streams_macro;
