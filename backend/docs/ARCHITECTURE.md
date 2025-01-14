# Introduction
We give here only a small excerpt of the architecture of the backend of the application. You are encouraged to read also the related documentation:
* [Rocket](https://rocket.rs/guide/) - web framework used

We implement the backend using Rust. This means it can be widely deployed on different devices and the cloud. For example, you can expose a web frontend of a software-defined radio (SDR) device or you can provide a highly scalable cloud web application.

# Rust
We use the [Rust Edition](https://doc.rust-lang.org/edition-guide/editions/index.html) 2021. Rust editions allow you to manage updates to the compiler and its infrastructure gracefully without breaking the application code.

# Rocket
We use as a web application framework [Rocket](https://rocket.rs/guide/) with the latest version. Rocket is a lightweight and powerful framework.

It has not an extensive ecosystem as, for example, Spring. However, it can extended in a very flexible manner using
* [Fairings](https://rocket.rs/guide/v0.5/fairings/#fairings): intercept and rewrite requests/responses (with certain limitations)
* [Request Guards](https://rocket.rs/guide/v0.5/requests/#request-guards): Intercept requests and decide to allow or deny them. This can be used for authentication/authorisation and also for preventing malicious requests.
* [Sentinels](https://api.rocket.rs/v0.5/rocket/trait.Sentinel): A line of defense to abort starting the application if routes are added who have not implemented a sentinel


# Configuration
Rocket allows flexible configuration via [Rocket.toml](https://rocket.rs/guide/configuration/). This is mainly for configuring Rocket itself.

The application configures Rocket to also read an app-specific configuration file, where it [reads custom values](https://rocket.rs/guide/v0.5/configuration/#extracting-values) for the application itself.


See also [CONFIGURE.md](./CONFIGURE.md)

# Single Page Application
The frontend is a Single Page Application (SPA) written using SvelteKit. In Svelte (and also in many other frameworks) the concept of routes exists, i.e. the URL is changed to redirect the user at the frontend to different frontend views (components). This leads to a situation, where the backend interprets those as requests to the backend, which is incorrect.

Hence, we needed to configure a special Rocket route that makes sure that routes to frontend components are only redirected to the frontend.

You can find it in [../src/routes/redirect_frontend.rs](../src/routes/redirect_frontend.rs)
# Database
tbd

# Logging
Logging is supported using [tracing](https://docs.rs/tracing/latest/tracing/). The main reason is that in async scenarios, such as we implement with Rocket requests, it is difficult to bring the messages in a consistent ordered view establishing temporality and causality when using "traditional" loggers, such as [log](https://docs.rs/log/latest/log/).

You can log events (moments in time) or time spans.

# Security
## General
Rocket has 

## Input/Output Sanitization
We provide input/output sanitization mechanisms based on [Ammonia](
https://github.com/rust-ammonia/ammonia). This sanitization removes malicious HTML/scripts from any text input from the user/output to the user. The idea is that you can sanitize any user input (e.g. new orders) before storing it in the database and to sanitize anything that comes from the database before returning it to the users. By doing so you can protect your users and your business from any harm coming from cross-site-scripting attacks (e.g. in our demo case malicious orders from customers or leaking of orders of other customers).

## Form Validation
Additionally to input/output sanitization, one should use the mechanism provided by [Rocket for form validation](https://rocket.rs/guide/upgrading/#field-validation), if you use forms. This prevents some issues related to untrusted data from forms (all data from forms should be treated as untrusted - even if they are validated in the browser by Javascript).

## Limit request size
Another way of input/output sanitization is to limit the size what can be submitted in one request. Otherwise people will try to bring your application down by flooding it with large requests. 

This can be done by [configuring](https://rocket.rs/guide/configuration/#configuration) limits in Rocket for form submissions and JSON posts. You should also limit it for other type of requests.

If your application needs very large requests then you should not allow large limits, instead you should design your application that it splits large requests on the client side in smaller bits. For example, a 1 TB file can be split into 5 GB chunks and posted in 5 GB chunks to be afterwards reassembled on server side by a dedicated backend that is not the web application.

In this way, you can keep memory, cpu and other resource usage under control.