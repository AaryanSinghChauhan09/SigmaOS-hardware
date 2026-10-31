# SigmaOS Hardware

Rust library of hardware abstraction and device-management models. The public
modules include a hardware device registry, compatibility records, mount and
printer models, Bluetooth state, and other device-facing interfaces.

## Hardware evidence limits

The presence of a device type or operation in the API is not proof that the
crate scans hardware, performs a mount, connects a Bluetooth device, or controls
a printer on a real system. This README does not claim a supported-device
matrix. Treat each path as experimental until its implementation and target
evidence establish the behavior.

Hardware detection should be read-only by default and report platform,
capability, permissions, and evidence boundaries accurately. Operations that
change mounts, power, pairing, or device state need explicit authorization and
failure recovery.

## Development

The crate depends on `SigmaOS-kernel` through a sibling path dependency. CI
resolves SigmaOS path dependencies and runs `cargo test --all-targets --locked`.
