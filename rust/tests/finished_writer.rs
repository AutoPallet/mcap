use std::io::Cursor;

#[test]
fn dropping_finished_writer_does_not_allocate() -> anyhow::Result<()> {
    let mut writer = mcap::WriteOptions::new()
        .use_chunks(false)
        .create(Cursor::new(Vec::new()))?;
    let schema = writer.add_schema("sample", "raw", &[0])?;
    writer.add_channel(schema, "/sample", "raw", &Default::default())?;
    writer.finish()?;

    let allocations = allocation_counter::measure(|| drop(writer));
    assert_eq!(allocations.count_total, 0);
    Ok(())
}
