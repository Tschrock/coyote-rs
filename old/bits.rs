//! Helper functions for bit-level operations
//! 

pub fn read_u4<R: io::Read + io::Seek>(reader: &mut Reader<R>) -> Result<u4, DekuError> {
    Ok(u4::new(u8::from_reader_with_ctx(
        reader,
        BitSize(u4::BITS),
    )?))
}

pub fn write_u4<W: io::Write + io::Seek>(value: u4, writer: &mut Writer<W>) -> Result<(), DekuError> {
    value.value().to_writer(writer, BitSize(u4::BITS))
}
