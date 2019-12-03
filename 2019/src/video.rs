pub trait Palette:
	Copy + enum_iterator::IntoEnumIterator + Eq + std::hash::Hash + std::fmt::Debug
{
	fn color(self) -> &'static [u8; 3];
	fn filename(self) -> Option<&'static str>;
}

#[allow(dead_code)]
pub const BLACK: [u8; 3] = [0, 0, 0];

macro_rules! palette {
	(@color $x:literal) => {
		&crate::video::BLACK
	};
	(@color $x:expr) => {
		&$x
	};
	(@filename $x:literal) => {
		Some($x)
	};
	(@filename $x:expr) => {
		None
	};
	($p:ident {
		$(
			$n:ident = $v:tt,
		)+
	}) => {
		#[derive(Clone, Copy, enum_iterator::IntoEnumIterator, Eq, PartialEq, Hash, Debug)]
		enum $p {
			$(
				$n,
			)+
		}

		impl crate::video::Palette for $p {
			fn color(self) -> &'static [u8; 3] {
				match self {
					$(
						$p::$n => palette!(@color $v),
					)+
				}
			}

			fn filename(self) -> Option<&'static str> {
				match self {
					$(
						$p::$n => palette!(@filename $v),
					)+
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

#[allow(dead_code)]
struct Video<P: Palette> {
	width: u16,
	height: u16,
	scale: u16,
	#[cfg(feature = "video")]
	encoder: gif::Encoder<std::fs::File>,
	sprites: std::collections::HashMap<P, Vec<u8>>,
}

#[allow(dead_code)]
pub struct OptionalVideo<P: Palette>(Option<Video<P>>);

#[cfg(feature = "video")]
impl<'a, P: Palette + 'a> OptionalVideo<P> {
	pub fn new(
		enabled: bool,
		name: &str,
		width: u16,
		height: u16,
		scale: u16,
	) -> Result<Self, crate::Error> {
		if !enabled {
			return Ok(Self(None));
		}
		let mut color_map = Vec::new();
		let mut known_colors: std::collections::HashMap<[u8; 3], u8> =
			std::collections::HashMap::new();
		let mut next_index = 0;
		let mut get_color = |rgb: [u8; 3]| {
			*known_colors.entry(rgb).or_insert_with(|| {
				color_map.extend_from_slice(&rgb);
				next_index += 1;
				next_index - 1
			})
		};
		let mut path: std::path::PathBuf = std::path::Path::new(file!()).parent()?.parent()?.into();
		let mut read_path = path.clone();
		read_path.push("sprites");
		let mut sprites = std::collections::HashMap::new();
		for p in P::into_enum_iter() {
			sprites.insert(
				p,
				if let Some(f) = p.filename() {
					let mut read_path = read_path.clone();
					read_path.push(f);
					let mut img = image::open(read_path)?;
					let img = img.crop(0, 0, scale.into(), scale.into());
					let img = img
						.to_rgb()
						.into_raw()
						.chunks(3)
						.map(|rgb| get_color([rgb[0], rgb[1], rgb[2]]))
						.collect();
					img
				} else {
					let color = get_color(*p.color());
					vec![color; scale as usize * scale as usize * 3]
				},
			);
		}
		path.push("video");
		std::fs::create_dir_all(&path)?;
		path.push(format!("{}.gif", name));
		let image = std::fs::File::create(&path)?;
		let mut encoder = gif::Encoder::new(image, width * scale, height * scale, &color_map)?;
		use gif::SetParameter;
		encoder.set(gif::Repeat::Infinite)?;
		Ok(Self(Some(Video {
			width,
			height,
			scale,
			encoder,
			sprites,
		})))
	}
}

#[cfg(feature = "video")]
impl<'a, P: Palette + 'a> OptionalVideo<P> {
	pub fn frame<I: Iterator<Item = Vec<P>>>(&mut self, rows: I) -> Result<(), crate::Error> {
		let this = if let Some(this) = &mut self.0 {
			this
		} else {
			return Ok(());
		};
		let mut frame = gif::Frame::default();
		frame.height = this.height * this.scale;
		frame.width = this.width * this.scale;
		let mut data: Vec<u8> = vec![0; frame.height as usize * frame.width as usize];
		let mut offset = 0;
		for row in rows {
			for y in 0..this.scale {
				for color in &row {
					let img = this.sprites.get(&color)?;
					for x in 0..this.scale {
						data[offset] = img[y as usize * this.scale as usize + x as usize];
						offset += 1;
					}
				}
			}
		}
		frame.buffer = std::borrow::Cow::Owned(data);
		this.encoder.write_frame(&frame)?;
		Ok(())
	}
}
