//! Contains the ingredients needed to create wrappers over tokio AsyncRead/AsyncWrite items
//! to automatically reconnect upon failures. This is done so that a user can use them without worrying
//! that their application logic will terminate simply due to an event like a temporary network failure.
//!
//! This crate will try to provide commonly used io items, for example, the [StubbornTcpStream](StubbornTcpStream).
//! If you need to create your own, you simply need to implement the [UnderlyingIo](crate::tokio::UnderlyingIo) trait.
//! Once implemented, you can construct it easily by creating a [StubbornIo](crate::tokio::StubbornIo) type as seen below.
//!
//! *This crate requires at least version 1.39 of the Rust compiler.*
//!
//! ### Motivations
//! This crate was created because I was working on a service that needed to fetch data from a remote server
//! via a tokio TcpConnection. It normally worked perfectly (as does all of my code ☺), but every time the
//! remote server had a restart or turnaround, my application logic would stop working.
//! **stubborn-io** was born because I did not want to complicate my service's logic with TcpStream
//! reconnect and disconnect handling code. With stubborn-io, I can keep the service exactly the same,
//! knowing that the StubbornTcpStream's sensible defaults will perform reconnects in a way to keep my service running.
//! Once I realized that the implementation could apply to all IO items and not just TcpStream, I made it customizable as
//! seen below.
//!
//! ## Example on how a Stubborn IO item might be created
//! ```
//! use std::io;
//! use std::future::Future;
//! use std::path::PathBuf;
//! use std::pin::Pin;
//! use stubborn_io::tokio::{StubbornIo, UnderlyingIo};
//! use tokio::fs::File;
//!
//! struct MyFile(File); // Struct must implement AsyncRead + AsyncWrite
//!
//! impl UnderlyingIo<PathBuf> for MyFile {
//!     // Establishes an io connection.
//!     // Additionally, this will be used when reconnect tries are attempted.
//!     fn establish(path: PathBuf) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send>> {
//!         Box::pin(async move {
//!             // In this case, we are trying to "connect" a file that
//!             // should exist on the system
//!             let tokio_file = File::open(path).await?;
//!             Ok(MyFile(tokio_file))
//!         })
//!     }
//! }
//!
//! # async fn test() -> io::Result<()> {
//! // Because StubbornIo implements deref, you are able to invoke
//! // the original methods on the File struct.
//! type HomemadeStubbornFile = StubbornIo<MyFile, PathBuf>;
//! let path = PathBuf::from("./foo/bar.txt");
//!
//! let stubborn_file = HomemadeStubbornFile::connect(path).await?;
//! // ... application logic here!
//!  # Ok(())
//!  # }
//! ```

pub mod config;
pub mod strategies;

// in the future, there may be a mod for synchronous regular io too, which is why
// tokio is specifically chosen to place the async stuff
pub mod tokio;

#[doc(inline)]
pub use self::config::ReconnectOptions;
#[doc(inline)]
pub use self::tokio::StubbornTcpStream;
