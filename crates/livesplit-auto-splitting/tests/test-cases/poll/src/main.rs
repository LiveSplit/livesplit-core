#[no_mangle]
pub unsafe extern "C" fn configure() {
    let mut out: wasi::Event = std::mem::zeroed();

    wasi::poll_oneoff(
        &wasi::Subscription {
            userdata: 0,
            r#type: wasi::EVENTTYPE_FD_READ,
            u: wasi::SubscriptionU {
                fd_readwrite: wasi::SubscriptionFdReadwrite { file_descriptor: 0 },
            },
        },
        &mut out,
        1,
    )
    .unwrap();

    wasi::poll_oneoff(
        &wasi::Subscription {
            userdata: 1,
            r#type: wasi::EVENTTYPE_FD_WRITE,
            u: wasi::SubscriptionU {
                fd_readwrite: wasi::SubscriptionFdReadwrite { file_descriptor: 1 },
            },
        },
        &mut out,
        1,
    )
    .unwrap();
}

fn main() {}
