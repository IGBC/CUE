//! #Bifrost
//! API for applications native to the CUE windowing system.
//! Allows transparent use of cursive to intergrate into CUE.
//! If cue is unavailable this API will create and wrap a cursive instance.
//! 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod windows;
pub mod bifrost;