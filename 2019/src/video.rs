pub trait Palette: Copy + Into<u8> + enum_iterator::IntoEnumIterator {
	fn color(self) -> &'static [u8; 3];
}

macro_rules! palette {
	($p:ident {
		$(
			$n:ident = $v:expr
		),+,
	}) => {
		#[derive(Clone, Copy, enum_iterator::IntoEnumIterator)]
		enum $p {
			$(
				$n,
			)*
		}

		impl crate::video::Palette for $p {
			fn color(self) -> &'static [u8; 3] {
				match self {
					$(
						$p::$n => $v,
					)*
				}
			}
		}

		impl From<$p> for u8 {
			fn from(p: $p) -> u8 {
				p as u8
			}
		}
	};
}

#[cfg(feature = "video")]
pub struct Video<P: Palette> {
	width: u16,
	height: u16,
	scale: u16,
	encoder: gif::Encoder<std::fs::File>,
	_p: std::marker::PhantomData<P>,
}

#[cfg(feature = "video")]
impl<'a, P: Palette + 'a> Video<P> {
	pub fn new(name: &str, width: u16, height: u16, scale: u16) -> Result<Self, crate::Error> {
		let mut path: std::path::PathBuf = std::path::Path::new(file!()).parent()?.parent()?.into();
		path.push("video");
		std::fs::create_dir_all(&path)?;
		path.push(format!("{}.gif", name));
		let image = std::fs::File::create(&path)?;
		let color_map: Vec<_> = P::into_enum_iter()
			.map(|p| p.color())
			.flatten()
			.cloned()
			.collect();
		let mut encoder = gif::Encoder::new(image, width * scale, height * scale, &color_map)?;
		use gif::SetParameter;
		encoder.set(gif::Repeat::Infinite)?;
		Ok(Self {
			width,
			height,
			scale,
			encoder,
			_p: std::marker::PhantomData,
		})
	}

	pub fn frame<I: Iterator<Item = &'a Vec<P>>>(&mut self, rows: I) -> Result<(), std::io::Error> {
		let mut frame = gif::Frame::default();
		frame.height = self.height * self.scale;
		frame.width = self.width * self.scale;
		let mut data: Vec<u8> = vec![0; frame.height as usize * frame.width as usize];
		let mut offset = 0;
		for row in rows {
			for _ in 0..self.scale {
				for &color in row {
					for _ in 0..self.scale {
						data[offset] = color.into();
						offset += 1;
					}
				}
			}
		}
		frame.buffer = std::borrow::Cow::Owned(data);
		self.encoder.write_frame(&frame)
	}
}
