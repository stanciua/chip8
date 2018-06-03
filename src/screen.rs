pub mod screen {
    #[derive(Debug)]
    pub struct Screen<'a> {
        screen: &'a mut [u8],
    }

    impl<'a> Screen<'a> {
        pub fn with_display(display: &mut [u8]) -> Screen {
            Screen { screen: display }
        }

        pub fn render(&self) {
            println!("Rendering the screen");
        }

        #[inline(always)]
        pub fn size(&self) -> usize {
            self.screen.len()
        }

        #[inline(always)]
        pub fn width(&self) -> usize {
            32
        }

        #[inline(always)]
        pub fn height(&self) -> usize {
            64
        }
        #[inline(always)]
        pub fn screen(&mut self) -> &mut [u8] {
            self.screen
        }
    }

}
