use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

#[smartmodule(map)]
pub fn remove_linebreaks(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let text = String::from_utf8_lossy(record.value());

    let cleaned_text = text.replace("\n", " ").replace("\r", " ");

    Ok((record.key().cloned(), RecordData::from(cleaned_text)))
}
