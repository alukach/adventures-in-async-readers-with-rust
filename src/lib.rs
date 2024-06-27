use futures::io::AsyncRead;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct BufferedReader {
    current: u8,
    pages: u8,
}

// TODO: The buffered reader should generate more data than it stores in the buffer. It
// should be able to generate incrementally larger chunks of data and only push that into
// the buffer as the buffer is consumed.

impl BufferedReader {
    // TODO: Make this async
    fn fetch_page_of_data(start: u8, len: usize) -> Vec<u8> {
        // TODO: Add a sleep here to simulate a slow data source...
        (start..start + len as u8).collect::<Vec<u8>>()
    }
}

impl AsyncRead for BufferedReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let next_vals = Self::fetch_page_of_data(self.current, buf.len());
        self.current += next_vals.len() as u8;
        self.pages += 1;

        let len = buf.len().min(next_vals.len());
        println!("next_vals.len(): {:?}", next_vals.len());
        println!("buf.len(): {:?}", buf.len());
        buf[..len].copy_from_slice(&next_vals[..len]);
        return Poll::Ready(Ok(len));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::AsyncReadExt;

    #[tokio::test]
    async fn test_integer_generator_initial_value() {
        let mut reader = BufferedReader {
            current: 0,
            pages: 0,
        };

        let mut output = [0; 25];
        let bytes = reader.read(&mut output[..]).await.unwrap();

        println!("bytes v1: {:?}, data: {:?}", bytes, &output[..bytes]);
        println!("The position: {:?}", &reader.current);
        let mut output = [0; 22];
        
        let bytes = reader.read(&mut output[..]).await.unwrap();
        println!("bytes v2: {:?}, data: {:?}", bytes, &output[..bytes]);
        println!("The position: {:?}", &reader.current);
        assert_eq!(reader.pages, 2);
    }
}
