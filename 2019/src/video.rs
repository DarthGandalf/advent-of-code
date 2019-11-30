pub struct Video {
	width: u16,
	height: u16,
	scale: u16,
	encoder: gif::Encoder<std::fs::File>,
}

impl Video {
	pub fn new(
		name: &str,
		width: u16,
		height: u16,
		scale: u16,
		colors: &[[u8; 3]],
	) -> Result<Self, crate::Error> {
		let mut path: std::path::PathBuf = std::path::Path::new(file!()).parent()?.parent()?.into();
		path.push("video");
		std::fs::create_dir_all(&path)?;
		path.push(format!("{}.gif", name));
		let image = std::fs::File::create(&path)?;
		let color_map: Vec<_> = colors.iter().flatten().cloned().collect();
		let mut encoder = gif::Encoder::new(image, width * scale, height * scale, &color_map)?;
		use gif::SetParameter;
		encoder.set(gif::Repeat::Infinite)?;
		Ok(Self {
			width,
			height,
			scale,
			encoder,
		})
	}

	pub fn frame<'a, I: Iterator<Item = &'a Vec<u8>>>(
		&mut self,
		rows: I,
	) -> Result<(), std::io::Error> {
		let mut frame = gif::Frame::default();
		frame.height = self.height * self.scale;
		frame.width = self.width * self.scale;
		let mut data: Vec<u8> = vec![0; frame.height as usize * frame.width as usize];
		let mut offset = 0;
		for row in rows {
			for _ in 0..self.scale {
				for color in row {
					for _ in 0..self.scale {
						data[offset] = *color;
						offset += 1;
					}
				}
			}
		}
		frame.buffer = std::borrow::Cow::Owned(data);
		self.encoder.write_frame(&frame)
	}
}
