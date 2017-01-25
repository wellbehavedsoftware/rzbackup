use futures::Async;
use futures::Poll;
use futures::future::Future;
use futures::future::Shared;

pub struct CloningSharedFuture <FutureType: Future> {
	inner: Shared <FutureType>,
}

impl <FutureType> Future
for CloningSharedFuture <FutureType>
where
	FutureType: Future,
	FutureType::Item: Clone,
	FutureType::Error: Clone {

	type Item = FutureType::Item;
	type Error = FutureType::Error;

	fn poll (
		& mut self,
	) -> Poll <FutureType::Item, FutureType::Error> {

		match self.inner.poll () {

			Ok (Async::Ready (value_ref)) =>
				Ok (Async::Ready (value_ref.clone ())),

			Ok (Async::NotReady) =>
				Ok (Async::NotReady),

			Err (error) =>
				Err (error.clone ()),

		}

	}

}

impl <FutureType> Clone
for CloningSharedFuture <FutureType>
where
	FutureType: Future,
	FutureType::Item: Clone,
	FutureType::Error: Clone
{

	fn clone (
		& self,
	) -> Self {

		CloningSharedFuture {
			inner: self.inner.clone (),
		}

	}

}

pub trait IntoCloningSharedFuture <FutureType: Future> {

	fn cloning (
		self,
	) -> CloningSharedFuture <FutureType>;

}

impl <FutureType> IntoCloningSharedFuture <FutureType>
for Shared <FutureType>
where
	FutureType: Future,
	FutureType::Item: Clone,
	FutureType::Error: Clone
{

	fn cloning (
		self,
	) -> CloningSharedFuture <FutureType> {

		CloningSharedFuture {
			inner: self,
		}

	}

}

// ex: noet ts=4 filetype=rust
